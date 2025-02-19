package main

import (
	"adventofcode_2024/src/day10"
	"bufio"
	"fmt"
	"os"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Please provide the name of the day to execute (e.g. day10)")
		os.Exit(1)
	}

	filepath := fmt.Sprintf("src/%s/input", os.Args[1])

	if os.Args[1] == "day10" {
		fmt.Println("Part 1:", day10.Part1(getLinesFrom(filepath)))
		fmt.Println("Part 2:", day10.Part2(getLinesFrom(filepath)))
	}
}

func getLinesFrom(filepath string) []string {
	file, err := os.Open(filepath)
	if err != nil {
		fmt.Println("Error opening file")
		os.Exit(1)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var lines []string
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines
}
