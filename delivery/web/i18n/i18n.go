package i18n

import (
	"embed"
	"fmt"
	"time"

	"github.com/invopop/ctxi18n"
)

//go:embed locales/*.yaml
var locales embed.FS

// Load initializes the global locale registry from the embedded YAML files.
func Load() error {
	return ctxi18n.LoadWithDefault(locales, "en")
}

var germanWeekdays = [...]string{
	"Sonntag", "Montag", "Dienstag", "Mittwoch", "Donnerstag", "Freitag", "Samstag",
}

var germanMonths = [...]string{
	"", "Januar", "Februar", "März", "April", "Mai", "Juni",
	"Juli", "August", "September", "Oktober", "November", "Dezember",
}

// FormatDate formats a time value according to the locale's conventions.
// Go's time.Format only outputs English day/month names, so German needs manual formatting.
func FormatDate(t time.Time, lang string) string {
	if lang == "de" {
		return fmt.Sprintf("%s, %d. %s %d",
			germanWeekdays[t.Weekday()],
			t.Day(),
			germanMonths[t.Month()],
			t.Year(),
		)
	}
	return t.Format("Monday, 2. January 2006")
}
