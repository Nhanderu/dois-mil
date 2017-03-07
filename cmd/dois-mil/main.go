package main

import (
	"os"
	"strconv"
	"strings"

	"github.com/Nhanderu/tuyo/convert"
	"github.com/Nhanderu/tuyo/text"
	"github.com/eiannone/keyboard"
)

const (
	clearLine = "\033[1A\033[2K"
	satan     = 666
)

func main() {
	var count, num int
	if len(os.Args) != 1 {
		count, _ = convert.ToInt(os.Args[1])
	}
	if count <= 1 {
		count = 4
	}
	for i := 0; i < count; i++ {
		os.Stdout.WriteString(makeLine(num, count))
		os.Stdout.WriteString("\n")
	}
	if keyboard.Open() != nil {
		os.Exit(satan)
	}
	defer keyboard.Close()
	for {
		_, key, err := keyboard.GetSingleKey()
		if err != nil {
			os.Exit(satan)
		}
		switch key {
		case keyboard.KeyArrowUp:
			num++ // Temporary.
		case keyboard.KeyArrowRight:
			num++ // Temporary.
		case keyboard.KeyArrowDown:
			num-- // Temporary.
		case keyboard.KeyArrowLeft:
			num-- // Temporary.
		case keyboard.KeyEsc:
			break
		default:
			continue
		}
		for i := 0; i < count; i++ {
			os.Stdout.WriteString(clearLine)
		}
		for i := 0; i < count; i++ {
			os.Stdout.WriteString(makeLine(num, count))
			os.Stdout.WriteString("\n")
		}
	}
}

func makeLine(num, count int) string {
	return strings.Repeat(text.PadLeft(strconv.FormatInt(int64(num), 10), " ", 4), count)
}
