package utils

import "strings"

func CapitalizeFirst(s string) string {
	return strings.ToUpper(string(s[0])) + string(s[1:])
}
