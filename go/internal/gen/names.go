package gen

import (
	"strings"
	"unicode"
)

// snakeCase converts a Go identifier to snake_case, treating runs of capitals
// as one word: HelloWorld -> hello_world, UserID -> user_id, A -> a.
func snakeCase(s string) string {
	runes := []rune(s)
	var b strings.Builder
	for i, r := range runes {
		if unicode.IsUpper(r) {
			if i > 0 && runes[i-1] != '_' {
				prevLower := unicode.IsLower(runes[i-1]) || unicode.IsDigit(runes[i-1])
				nextLower := i+1 < len(runes) && unicode.IsLower(runes[i+1])
				if prevLower || (nextLower && unicode.IsUpper(runes[i-1])) {
					b.WriteByte('_')
				}
			}
			b.WriteRune(unicode.ToLower(r))
		} else {
			b.WriteRune(r)
		}
	}
	return b.String()
}

// crateName sanitizes a name the way Cargo derives a Rust crate name from a
// package name.
func crateName(s string) string {
	var b strings.Builder
	for _, r := range s {
		switch {
		case r == '-' || r == '.':
			b.WriteByte('_')
		case unicode.IsLetter(r) || unicode.IsDigit(r) || r == '_':
			b.WriteRune(r)
		}
	}
	return b.String()
}
