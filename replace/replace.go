package replace

import (
	"log"
	"strings"
)

func SearchAndReplaceInBetweenTokens(leftToken, rightToken, s, old, new string) string {
	leftTokenCount := strings.Count(s, leftToken)
	rightTokenCount := strings.Count(s, rightToken)

	if leftTokenCount != rightTokenCount || leftTokenCount == 0 || leftToken == "" || rightToken == "" {
		return s
	}

	newString := ""

	leftTokenSplit := strings.Split(s, leftToken)
	for i, s2 := range leftTokenSplit {
		isOnLastItem := len(leftTokenSplit) == i+1

		newS2 := searchAndReplaceToTheLeftOfToken(rightToken, s2, old, new)

		if isOnLastItem {
			newString += newS2
			continue
		}

		newString += newS2 + leftToken

	}

	return newString
}

func searchAndReplaceToTheLeftOfToken(token, s, old, new string) string {
	split := strings.Split(s, token)

	if len(split) == 1 || !strings.ContainsAny(s, token) {
		return s
	}

	if len(split) != 2 {
		log.Fatalln("Unexpected token count")
	}

	return strings.ReplaceAll(split[0], old, new) + token + split[1]
}
