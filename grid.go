package main

import (
	"bytes"
	"io"
	"math/rand"
	"strconv"

	"github.com/Nhanderu/tuyo/text"
)

type Grid struct {
	v  [][]uint8
	s  int
	sc int64
}

func (g *Grid) moveCell(cola, lina, colb, linb int) {
	cella, cellb := &g.v[cola][lina], &g.v[colb][linb]
	if *cella == *cellb {
		*cella += *cellb
		*cellb = 0
	} else if *cella == 0 {
		*cella = *cellb
		*cellb = 0
	}
}

func (g *Grid) MoveUp() {
	for i := 0; i < g.s; i++ {
		for j := 0; j < g.s; j++ {
			for n := i + 1; n < g.s; n++ {
				g.moveCell(i, j, n, j)
			}
		}
	}
}

func (g *Grid) MoveRight() {
	for i := 0; i < g.s; i++ {
		for j := 0; j < g.s; j++ {
			for n := j - 1; n > 0; n-- {
				g.moveCell(i, j, i, n)
			}
		}
	}
}

func (g *Grid) MoveDown() {
	for i := 0; i < g.s; i++ {
		for j := 0; j < g.s; j++ {
			for n := i - 1; n > 0; n-- {
				g.moveCell(i, j, n, j)
			}
		}
	}
}

func (g *Grid) MoveLeft() {
	for i := 0; i < g.s; i++ {
		for j := 0; j < g.s; j++ {
			for n := j + 1; n < g.s; n++ {
				g.moveCell(i, j, i, n)
			}
		}
	}
}

func (g Grid) WriteTo(w io.Writer) (int64, error) {
	var total int64
	var n int
	var err error
	n, err = w.Write([]byte("\n"))
	if total += int64(n); err != nil {
		return total, err
	}
	n, err = w.Write([]byte("Score: "))
	if total += int64(n); err != nil {
		return total, err
	}
	n, err = w.Write([]byte(strconv.FormatInt(g.sc, 10)))
	if total += int64(n); err != nil {
		return total, err
	}
	n, err = w.Write([]byte("\n\n"))
	if total += int64(n); err != nil {
		return total, err
	}
	for i := 0; i < g.s; i++ {
		for j := 0; j < g.s; j++ {
			if v := g.v[i][j]; v == 0 {
				n, err = w.Write([]byte("     -"))
			} else {
				n, err = w.Write([]byte(text.PadLeft(strconv.FormatUint(uint64(g.v[i][j]), 10), " ", 6)))
			}
			if total += int64(n); err != nil {
				return total, err
			}
		}
		n, err = w.Write([]byte("\n"))
		if total += int64(n); err != nil {
			return total, err
		}
	}
	return total, nil
}

func (g Grid) String() string {
	var buffer bytes.Buffer
	g.WriteTo(&buffer)
	return buffer.String()
}

func NewGrid(s int) *Grid {
	g := Grid{make([][]uint8, s), s, 0}
	for i := 0; i < s; i++ {
		g.v[i] = make([]uint8, s)
		r := rand.Intn(2)
		if r == 1 {
			g.v[i][rand.Intn(g.s-1)] = 1
		}
	}
	return &g
}
