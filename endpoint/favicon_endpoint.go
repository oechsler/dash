package endpoint

import (
	"bytes"
	"dash/environment"
	"dash/middleware"
	"encoding/binary"
	"image"
	"image/color"
	"image/draw"
	_ "image/gif"
	_ "image/jpeg"
	"image/png"
	"math"
	"net/http"
	"time"

	"github.com/gofiber/fiber/v2"
	xdraw "golang.org/x/image/draw"
)

// EXPERIMENTAL

// Favicon registers a route that serves a circular-cropped user picture as an .ico at /favicon.ico.
// If no user or no picture is available, it responds with 204 No Content.
func Favicon(env *environment.Env, app *fiber.App) {
	app.Get("/favicon.ico", middleware.GetUserFromIdToken(env), func(c *fiber.Ctx) error {
		user, authorized := middleware.GetCurrentUser(c)
		if !authorized {
			return c.SendStatus(fiber.StatusNoContent)
		}

		if user.Picture == nil {
			return c.SendStatus(fiber.StatusNoContent)
		}
		img, err := fetchAndDecodeImage(*user.Picture)
		if err != nil {
			return c.SendStatus(fiber.StatusNoContent)
		}

		// Create a square crop from center, then scale to 64x64.
		square := centerSquare(img)
		const size = 64
		resized := image.NewNRGBA(image.Rect(0, 0, size, size))
		xdraw.CatmullRom.Scale(resized, resized.Bounds(), square, square.Bounds(), xdraw.Over, nil)

		// Apply circular mask
		circ := circularMask(resized)

		// Build ICO with 64x64 and 32x32 entries embedding PNGs (PNG-in-ICO)
		icoBytes, err := buildICOFromImages([]image.Image{circ}, []int{64, 32})
		if err != nil {
			return c.SendStatus(fiber.StatusNoContent)
		}

		c.Set("Content-Type", "image/x-icon")
		c.Set("Cache-Control", "public, max-age=300")
		return c.SendStream(bytes.NewReader(icoBytes))
	})
}

func fetchAndDecodeImage(url string) (image.Image, error) {
	client := &http.Client{Timeout: 5 * time.Second}
	req, err := http.NewRequest(http.MethodGet, url, nil)
	if err != nil {
		return nil, err
	}
	// Generic accept header for common image types
	req.Header.Set("Accept", "image/*, */*;q=0.8")
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()
	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return nil, fiber.ErrBadRequest
	}

	img, _, err := image.Decode(resp.Body)
	if err != nil {
		return nil, err
	}
	return toNRGBA(img), nil
}

// buildICOFromImages builds an .ico byte slice from a source image, producing requested square sizes, embedding PNG data per entry.
func buildICOFromImages(srcs []image.Image, sizes []int) ([]byte, error) {
	// For simplicity, use only the first src as base and resize to requested sizes.
	base := toNRGBA(srcs[0])
	type entry struct {
		w, h int
		png  []byte
	}
	ents := make([]entry, 0, len(sizes))
	for _, sz := range sizes {
		if sz <= 0 {
			continue
		}
		dst := image.NewNRGBA(image.Rect(0, 0, sz, sz))
		xdraw.CatmullRom.Scale(dst, dst.Bounds(), base, base.Bounds(), xdraw.Over, nil)
		buf := &bytes.Buffer{}
		if err := png.Encode(buf, dst); err != nil {
			return nil, err
		}
		ents = append(ents, entry{w: sz, h: sz, png: buf.Bytes()})
	}
	// ICO header: 6 bytes
	// idReserved(2)=0, idType(2)=1 (icon), idCount(2)=n
	header := make([]byte, 6)
	binary.LittleEndian.PutUint16(header[0:2], 0)
	binary.LittleEndian.PutUint16(header[2:4], 1)
	binary.LittleEndian.PutUint16(header[4:6], uint16(len(ents)))
	// Each ICONDIRENTRY is 16 bytes
	dir := &bytes.Buffer{}
	imgData := &bytes.Buffer{}
	// Calculate offsets: header(6) + n*16 directory
	offset := 6 + 16*len(ents)
	for _, e := range ents {
		w := e.w
		h := e.h
		b := make([]byte, 16)
		// Width and height fields store 0 for 256
		if w >= 256 {
			b[0] = 0
		} else {
			b[0] = byte(w)
		}
		if h >= 256 {
			b[1] = 0
		} else {
			b[1] = byte(h)
		}
		b[2] = 0                                  // color count
		b[3] = 0                                  // reserved
		binary.LittleEndian.PutUint16(b[4:6], 1)  // planes
		binary.LittleEndian.PutUint16(b[6:8], 32) // bit count
		size := len(e.png)
		binary.LittleEndian.PutUint32(b[8:12], uint32(size))
		binary.LittleEndian.PutUint32(b[12:16], uint32(offset))
		dir.Write(b)
		imgData.Write(e.png)
		offset += size
	}
	out := &bytes.Buffer{}
	out.Write(header)
	out.Write(dir.Bytes())
	out.Write(imgData.Bytes())
	return out.Bytes(), nil
}

// centerSquare crops the input image to a centered square.
func centerSquare(src image.Image) image.Image {
	r := src.Bounds()
	w := r.Dx()
	h := r.Dy()
	size := w
	if h < size {
		size = h
	}
	x0 := r.Min.X + (w-size)/2
	y0 := r.Min.Y + (h-size)/2
	crop := image.Rect(0, 0, size, size)
	out := image.NewNRGBA(crop)
	draw.Draw(out, out.Bounds(), src, image.Point{X: x0, Y: y0}, draw.Src)
	return out
}

// circularMask returns a copy of src with pixels outside the inscribed circle fully transparent.
func circularMask(src *image.NRGBA) *image.NRGBA {
	b := src.Bounds()
	cx := float64(b.Dx()) / 2
	cy := float64(b.Dy()) / 2
	r := math.Min(cx, cy) - 0.5

	out := image.NewNRGBA(b)
	// Fill transparent background
	draw.Draw(out, b, &image.Uniform{C: color.NRGBA{0, 0, 0, 0}}, image.Point{}, draw.Src)

	for y := b.Min.Y; y < b.Max.Y; y++ {
		for x := b.Min.X; x < b.Max.X; x++ {
			dx := float64(x-b.Min.X) + 0.5 - cx
			dy := float64(y-b.Min.Y) + 0.5 - cy
			if dx*dx+dy*dy <= r*r {
				// inside circle: copy pixel
				i := src.PixOffset(x, y)
				j := out.PixOffset(x, y)
				out.Pix[j+0] = src.Pix[i+0]
				out.Pix[j+1] = src.Pix[i+1]
				out.Pix[j+2] = src.Pix[i+2]
				out.Pix[j+3] = src.Pix[i+3]
			}
		}
	}
	return out
}

// toNRGBA converts arbitrary image to *image.NRGBA for easier pixel operations.
func toNRGBA(img image.Image) *image.NRGBA {
	b := img.Bounds()
	out := image.NewNRGBA(image.Rect(0, 0, b.Dx(), b.Dy()))
	draw.Draw(out, out.Bounds(), img, b.Min, draw.Src)
	return out
}
