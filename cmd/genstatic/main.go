//go:build ignore

package main

import (
	"flag"
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"regexp"
	"runtime"
	"strings"
)

const (
	userAgentHdr          = "Go-Generate-Assets/1.0"
	htmxURL               = "https://unpkg.com/htmx.org@1.9.12/dist/htmx.min.js"
	tailwindCDNURL        = "https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"
	materialCSSURL        = "https://fonts.googleapis.com/icon?family=Material+Icons"
	simpleIconsFontCSSURL = "https://unpkg.com/simple-icons-font@v15/font/simple-icons.min.css"
)

var (
	outBase   string
	urlPrefix string
)

func main() {
	flag.StringVar(&outBase, "out", "static", "Basisverzeichnis für Assets")
	flag.StringVar(&urlPrefix, "prefix", "/static", "URL-Prefix unter dem die Assets später serviert werden")
	flag.Parse()

	fmt.Printf("asset generator (%s/%s) → %s\n", runtime.GOOS, runtime.GOARCH, outBase)

	outDirCSS := filepath.Join(outBase, "css")
	outDirJS := filepath.Join(outBase, "js")
	outDirFonts := filepath.Join(outBase, "fonts")

	must(ensureDirs(outDirCSS, outDirJS, outDirFonts))

	must(downloadHTMX(outDirJS))
	must(downloadTailwindCDN(outDirJS))
	must(downloadMaterialIcons(outDirCSS, outDirFonts))
	must(downloadSimpleIconsFont(outDirCSS, outDirFonts))

	fmt.Println("✅ Done.")
}

func must(err error) {
	if err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		os.Exit(1)
	}
}

func ensureDirs(dirs ...string) error {
	for _, d := range dirs {
		if err := os.MkdirAll(d, 0o755); err != nil {
			return err
		}
	}
	return nil
}

func httpGet(url string) ([]byte, error) {
	req, _ := http.NewRequest("GET", url, nil)
	req.Header.Set("User-Agent", userAgentHdr)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()
	if resp.StatusCode/100 != 2 {
		return nil, fmt.Errorf("GET %s: %s", url, resp.Status)
	}
	return io.ReadAll(resp.Body)
}

func writeFile(path string, data []byte) error {
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return err
	}
	return os.WriteFile(path, data, 0o644)
}

func downloadHTMX(outDirJS string) error {
	fmt.Println("→ Downloading htmx…")
	b, err := httpGet(htmxURL)
	if err != nil {
		return err
	}
	return writeFile(filepath.Join(outDirJS, "htmx.min.js"), b)
}

func downloadTailwindCDN(outDirJS string) error {
	fmt.Println("→ Downloading Tailwind CDN script…")
	b, err := httpGet(tailwindCDNURL)
	if err != nil {
		return err
	}
	return writeFile(filepath.Join(outDirJS, "tailwind.min.js"), b)
}

func downloadMaterialIcons(outDirCSS, outDirFonts string) error {
	fmt.Println("→ Downloading Material Icons CSS…")

	variantSuffixes := []string{
		"", // base
		"+Outlined",
		"+Round",
		"+Sharp",
		"+Two+Tone",
	}
	var urls []string
	for _, suf := range variantSuffixes {
		if suf == "" {
			urls = append(urls, materialCSSURL)
		} else {
			u := strings.Replace(materialCSSURL, "Material+Icons", "Material+Icons"+suf, 1)
			urls = append(urls, u)
		}
	}

	re := regexp.MustCompile(`url\((?:"([^"]+)"|'([^']+)'|([^)]*))\)`)
	seenFonts := map[string]bool{}
	var combinedCSS strings.Builder

	for _, u := range urls {
		cssBytes, err := httpGet(u)
		if err != nil {
			return err
		}
		css := string(cssBytes)

		matches := re.FindAllStringSubmatch(css, -1)
		for _, m := range matches {
			var remote string
			for i := 1; i <= 3; i++ {
				if m[i] != "" {
					remote = m[i]
					break
				}
			}
			if remote == "" || !strings.HasPrefix(remote, "http") {
				continue
			}

			lower := strings.ToLower(remote)
			if !(strings.Contains(lower, ".woff2") ||
				strings.Contains(lower, ".woff") ||
				strings.Contains(lower, ".ttf") ||
				strings.Contains(lower, ".otf")) {
				continue
			}
			if !seenFonts[remote] {
				seenFonts[remote] = true

				name := remote[strings.LastIndex(remote, "/")+1:]
				if q := strings.IndexByte(name, '?'); q >= 0 {
					name = name[:q]
				}
				if name == "" {
					continue
				}

				fmt.Println("   ↳ font:", remote, "→", name)
				b, err := httpGet(remote)
				if err != nil {
					return fmt.Errorf("downloading font %s: %w", remote, err)
				}
				if err := writeFile(filepath.Join(outDirFonts, name), b); err != nil {
					return err
				}
				served := strings.TrimRight(urlPrefix, "/") + "/fonts/" + name
				css = strings.ReplaceAll(css, remote, served)
			}
		}

		if !strings.Contains(css, "font-display") {
			css = strings.ReplaceAll(css, "font-weight: 400;", "font-weight: 400;\n  font-display: swap;")
		}

		combinedCSS.WriteString(css)
		combinedCSS.WriteString("\n")
	}

	minComments := regexp.MustCompile(`(?s)/\*.*?\*/`)
	minWS := regexp.MustCompile(`\s+`)
	minDelims := regexp.MustCompile(`\s*([{}:;,])\s*`)

	minified := combinedCSS.String()
	minified = minComments.ReplaceAllString(minified, "")
	minified = strings.TrimSpace(minified)
	minified = minWS.ReplaceAllString(minified, " ")
	minified = minDelims.ReplaceAllString(minified, "$1")
	minified = strings.ReplaceAll(minified, ";}", "}")

	return writeFile(filepath.Join(outDirCSS, "material-icons.min.css"), []byte(minified))
}

func downloadSimpleIconsFont(outDirCSS, outDirFonts string) error {
	fmt.Println("→ Downloading Simple Icons Font CSS…")
	cssBytes, err := httpGet(simpleIconsFontCSSURL)
	if err != nil {
		return err
	}
	css := string(cssBytes)

	re := regexp.MustCompile(`url\((?:"([^"]+)"|'([^']+)'|([^)]*))\)`)
	matches := re.FindAllStringSubmatch(css, -1)

	seen := map[string]bool{}
	for _, m := range matches {
		var originalRef string
		for i := 1; i <= 3; i++ {
			if m[i] != "" {
				originalRef = m[i]
				break
			}
		}
		if originalRef == "" {
			continue
		}

		resolved := originalRef
		if !strings.HasPrefix(resolved, "http") {
			base := simpleIconsFontCSSURL[:strings.LastIndex(simpleIconsFontCSSURL, "/")+1]
			resolved = base + strings.TrimPrefix(originalRef, "./")
		}

		lower := strings.ToLower(resolved)
		if !(strings.Contains(lower, ".woff2") ||
			strings.Contains(lower, ".woff") ||
			strings.Contains(lower, ".ttf") ||
			strings.Contains(lower, ".otf")) {
			continue
		}
		if seen[resolved] {
			continue
		}
		seen[resolved] = true

		name := resolved[strings.LastIndex(resolved, "/")+1:]
		if q := strings.IndexByte(name, '?'); q >= 0 {
			name = name[:q]
		}
		if name == "" {
			continue
		}

		fmt.Println("   ↳ font:", resolved, "→", name)
		b, err := httpGet(resolved)
		if err != nil {
			return fmt.Errorf("downloading font %s: %w", resolved, err)
		}
		if err := writeFile(filepath.Join(outDirFonts, name), b); err != nil {
			return err
		}
		served := strings.TrimRight(urlPrefix, "/") + "/fonts/" + name
		css = strings.ReplaceAll(css, originalRef, served)
	}

	return writeFile(filepath.Join(outDirCSS, "simple-icons.min.css"), []byte(css))
}
