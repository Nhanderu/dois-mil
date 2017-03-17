package main

import (
	"os"
	"os/signal"

	"github.com/Nhanderu/tuyo/convert"
	"github.com/eiannone/keyboard"
	"golang.org/x/crypto/ssh/terminal"
)

const (
	clearLine = "\033[1A\033[2K"
	satan     = 666
)

var (
	stdoutFd = int(os.Stdout.Fd())

	stateBefore *terminal.State
	stdout      *terminal.Terminal
	grid        *Grid
)

func init() {
	// Get previous terminal state.
	var err error
	stateBefore, err = terminal.GetState(stdoutFd)
	if err != nil {
		os.Exit(1)
	}
	// Setup new terminal.
	stdout = terminal.NewTerminal(os.Stdout, "")
	stdout.AutoCompleteCallback = autoCompleteCallback
	// Setup grid.
	var count int
	if len(os.Args) != 1 {
		count, _ = convert.ToInt(os.Args[1])
	}
	if count <= 1 {
		count = 4
	}
	grid = NewGrid(count)
	// Setup input reading.
	if err := keyboard.Open(); err != nil {
		exit(1)
	}
	// Setup ^C handler.
	go func() {
		sc := make(chan os.Signal, 1)
		signal.Notify(sc, os.Interrupt)
		<-sc

		exit(2)
	}()
}

func main() {
	grid.WriteTo(stdout)
	for {
		_, key, err := keyboard.GetKey()
		if err != nil {
			exit(1)
		}
		switch key {
		case keyboard.KeyArrowUp:
			grid.MoveUp()
		case keyboard.KeyArrowRight:
			grid.MoveRight()
		case keyboard.KeyArrowDown:
			grid.MoveDown()
		case keyboard.KeyArrowLeft:
			grid.MoveLeft()
		case keyboard.KeyEsc:
			exit(0)
		default:
			continue
		}
		grid.WriteTo(stdout)
	}
}

func exit(n int) {
	keyboard.Close()
	terminal.Restore(stdoutFd, stateBefore)
	os.Exit(n)
}

func autoCompleteCallback(line string, pos int, key rune) (string, int, bool) {
	// ?
	return "", 0, true
}
