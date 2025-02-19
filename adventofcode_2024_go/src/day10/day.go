package day10

import (
    "fmt"
    "slices"
    "strconv"
)

func Part1(lines []string) string {
    input := parseInput(lines)
    trailheadPositions := findTrailheadPositions(input)
    var trailSum int
    for _, trailheadPosition := range trailheadPositions {
        ninePositions := getNinePositionsForTrailhead(input, trailheadPosition)
        trailSum += len(ninePositions)
    }

    return fmt.Sprintf("%d", trailSum)
}

func Part2(lines []string) string {
    return ""
}

type Position struct {
    X int
    Y int
}

func parseInput(lines []string) [][]int {
    var result [][]int
    for _, line := range lines {
        var row []int
        for _, char := range line {
            num, err := strconv.Atoi(string(char))
            if err != nil {
                panic(err)
            }
            row = append(row, num)
        }
        result = append(result, row)
    }
    return result
}

func findTrailheadPositions(input [][]int) []Position {
    var result []Position
    for i, row := range input {
        for j, num := range row {
            if num == 0 {
                result = append(result, Position{X: j, Y: i})
            }
        }
    }
    return result
}

func getNinePositionsForTrailhead(input [][]int, currentPosition Position) []Position {
    ninePositions := make([]Position, 0, len(input[0])*len(input))

    // positions from 0 -> 1
    possibleNextPositions := findPossibleNextPositionsForCurrentPosition(input, currentPosition)
    if len(possibleNextPositions) == 0 {
        return ninePositions
    }

    // positions from 1 -> 2
    for _, nextPosition := range possibleNextPositions {
        possibleNextPositions = findPossibleNextPositionsForCurrentPosition(input, nextPosition)
        if len(possibleNextPositions) == 0 {
            continue
        }

        // positions from 2 -> 3
        for _, nextPosition := range possibleNextPositions {
            possibleNextPositions = findPossibleNextPositionsForCurrentPosition(input, nextPosition)
            if len(possibleNextPositions) == 0 {
                continue
            }

            // positions from 3 -> 4
            for _, nextPosition := range possibleNextPositions {
                possibleNextPositions = findPossibleNextPositionsForCurrentPosition(input, nextPosition)
                if len(possibleNextPositions) == 0 {
                    continue
                }

                // positions from 4 -> 5
                for _, nextPosition := range possibleNextPositions {
                    possibleNextPositions = findPossibleNextPositionsForCurrentPosition(input, nextPosition)
                    if len(possibleNextPositions) == 0 {
                        continue
                    }

                    // positions from 5 -> 6
                    for _, nextPosition := range possibleNextPositions {
                        possibleNextPositions = findPossibleNextPositionsForCurrentPosition(input, nextPosition)
                        if len(possibleNextPositions) == 0 {
                            continue
                        }

                        // positions from 6 -> 7
                        for _, nextPosition := range possibleNextPositions {
                            possibleNextPositions = findPossibleNextPositionsForCurrentPosition(input, nextPosition)
                            if len(possibleNextPositions) == 0 {
                                continue
                            }

                            // positions from 7 -> 8
                            for _, nextPosition := range possibleNextPositions {
                                possibleNextPositions = findPossibleNextPositionsForCurrentPosition(input, nextPosition)
                                if len(possibleNextPositions) == 0 {
                                    continue
                                }

                                // positions from 8 -> 9
                                for _, nextPosition := range possibleNextPositions {
                                    possibleNextPositions = findPossibleNextPositionsForCurrentPosition(input, nextPosition)
                                    if len(possibleNextPositions) == 0 {
                                        continue
                                    }

                                    // positions from 9 -> 10
                                    for _, nextPosition := range possibleNextPositions {
                                        if !slices.Contains(ninePositions, nextPosition) {
                                            ninePositions = append(ninePositions, nextPosition)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    return ninePositions
}

func findPossibleNextPositionsForCurrentPosition(input [][]int, currentPosition Position) []Position {
    var result []Position

    if currentPosition.X < len(input[currentPosition.Y])-1 &&
        input[currentPosition.Y][currentPosition.X]+1 == input[currentPosition.Y][currentPosition.X+1] {
        result = append(result, Position{X: currentPosition.X + 1, Y: currentPosition.Y})
    }

    if currentPosition.Y < len(input)-1 &&
        input[currentPosition.Y][currentPosition.X]+1 == input[currentPosition.Y+1][currentPosition.X] {
        result = append(result, Position{X: currentPosition.X, Y: currentPosition.Y + 1})
    }

    if currentPosition.X > 0 &&
        input[currentPosition.Y][currentPosition.X]+1 == input[currentPosition.Y][currentPosition.X-1] {
        result = append(result, Position{X: currentPosition.X - 1, Y: currentPosition.Y})
    }

    if currentPosition.Y > 0 &&
        input[currentPosition.Y][currentPosition.X]+1 == input[currentPosition.Y-1][currentPosition.X] {
        result = append(result, Position{X: currentPosition.X, Y: currentPosition.Y - 1})
    }

    return result
}
