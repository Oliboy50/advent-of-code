package day10

import (
    "testing"

    "github.com/stretchr/testify/assert"
)

func TestPart1(t *testing.T) {
    input := []string{
        "89010123",
        "78121874",
        "87430965",
        "96549874",
        "45678903",
        "32019012",
        "01329801",
        "10456732",
    }

    result := Part1(input)

    assert.Equal(t, "36", result)
}

func Test_parseInput(t *testing.T) {
    input := []string{
        "89010123",
        "78121874",
        "87430965",
        "96549874",
        "45678903",
        "32019012",
        "01329801",
        "10456732",
    }

    result := parseInput(input)

    assert.Equal(t, [][]int{
        {8, 9, 0, 1, 0, 1, 2, 3},
        {7, 8, 1, 2, 1, 8, 7, 4},
        {8, 7, 4, 3, 0, 9, 6, 5},
        {9, 6, 5, 4, 9, 8, 7, 4},
        {4, 5, 6, 7, 8, 9, 0, 3},
        {3, 2, 0, 1, 9, 0, 1, 2},
        {0, 1, 3, 2, 9, 8, 0, 1},
        {1, 0, 4, 5, 6, 7, 3, 2},
    }, result)
}

func Test_findTrailheadPositions(t *testing.T) {
    input := [][]int{
        {8, 9, 0, 1, 0, 1, 2, 3},
        {7, 8, 1, 2, 1, 8, 7, 4},
        {8, 7, 4, 3, 0, 9, 6, 5},
        {9, 6, 5, 4, 9, 8, 7, 4},
        {4, 5, 6, 7, 8, 9, 0, 3},
        {3, 2, 0, 1, 9, 0, 1, 2},
        {0, 1, 3, 2, 9, 8, 0, 1},
        {1, 0, 4, 5, 6, 7, 3, 2},
    }

    result := findTrailheadPositions(input)

    assert.ElementsMatch(t, []Position{
        {X: 2, Y: 0},
        {X: 4, Y: 0},
        {X: 4, Y: 2},
        {X: 6, Y: 4},
        {X: 2, Y: 5},
        {X: 5, Y: 5},
        {X: 0, Y: 6},
        {X: 6, Y: 6},
        {X: 1, Y: 7},
    }, result)
}

func Test_findPossibleNextPositionsForCurrentPosition(t *testing.T) {
    input := [][]int{
        {8, 9, 0, 1, 0, 1, 2, 3},
        {7, 8, 1, 2, 1, 8, 7, 4},
        {8, 7, 4, 3, 0, 9, 6, 5},
        {9, 6, 5, 4, 9, 8, 7, 4},
        {4, 5, 6, 7, 8, 9, 0, 3},
        {3, 2, 0, 1, 9, 0, 1, 2},
        {0, 1, 3, 2, 9, 8, 0, 1},
        {1, 0, 4, 5, 6, 7, 3, 2},
    }

    currentPosition := Position{
        X: 2, Y: 0,
    }

    result := findPossibleNextPositionsForCurrentPosition(input, currentPosition)

    assert.ElementsMatch(t, []Position{
        {X: 3, Y: 0},
        {X: 2, Y: 1},
    }, result)
}

func Test_getNinePositionsForTrailhead_1(t *testing.T) {
    input := [][]int{
        {8, 9, 0, 1, 0, 1, 2, 3},
        {7, 8, 1, 2, 1, 8, 7, 4},
        {8, 7, 4, 3, 0, 9, 6, 5},
        {9, 6, 5, 4, 9, 8, 7, 4},
        {4, 5, 6, 7, 8, 9, 0, 3},
        {3, 2, 0, 1, 9, 0, 1, 2},
        {0, 1, 3, 2, 9, 8, 0, 1},
        {1, 0, 4, 5, 6, 7, 3, 2},
    }

    trailhead := Position{
        X: 2, Y: 0,
    }

    result := getNinePositionsForTrailhead(input, trailhead)

    assert.ElementsMatch(t, []Position{
        {X: 1, Y: 0},
        {X: 0, Y: 3},
        {X: 4, Y: 3},
        {X: 5, Y: 4},
        {X: 4, Y: 5},
    }, result)
}

func Test_getNinePositionsForTrailhead_2(t *testing.T) {
    input := [][]int{
        {8, 9, 0, 1, 0, 1, 2, 3},
        {7, 8, 1, 2, 1, 8, 7, 4},
        {8, 7, 4, 3, 0, 9, 6, 5},
        {9, 6, 5, 4, 9, 8, 7, 4},
        {4, 5, 6, 7, 8, 9, 0, 3},
        {3, 2, 0, 1, 9, 0, 1, 2},
        {0, 1, 3, 2, 9, 8, 0, 1},
        {1, 0, 4, 5, 6, 7, 3, 2},
    }

    trailhead := Position{
        X: 4, Y: 0,
    }

    result := getNinePositionsForTrailhead(input, trailhead)

    assert.ElementsMatch(t, []Position{
        {X: 1, Y: 0},
        {X: 5, Y: 2},
        {X: 0, Y: 3},
        {X: 4, Y: 3},
        {X: 5, Y: 4},
        {X: 4, Y: 5},
    }, result)
}

func Test_getNinePositionsForTrailhead_3(t *testing.T) {
    input := [][]int{
        {8, 9, 0, 1, 0, 1, 2, 3},
        {7, 8, 1, 2, 1, 8, 7, 4},
        {8, 7, 4, 3, 0, 9, 6, 5},
        {9, 6, 5, 4, 9, 8, 7, 4},
        {4, 5, 6, 7, 8, 9, 0, 3},
        {3, 2, 0, 1, 9, 0, 1, 2},
        {0, 1, 3, 2, 9, 8, 0, 1},
        {1, 0, 4, 5, 6, 7, 3, 2},
    }

    trailhead := Position{
        X: 4, Y: 2,
    }

    result := getNinePositionsForTrailhead(input, trailhead)

    assert.ElementsMatch(t, []Position{
        {X: 1, Y: 0},
        {X: 0, Y: 3},
        {X: 4, Y: 3},
        {X: 5, Y: 4},
        {X: 4, Y: 5},
    }, result)
}

func Test_getNinePositionsForTrailhead_4(t *testing.T) {
    input := [][]int{
        {8, 9, 0, 1, 0, 1, 2, 3},
        {7, 8, 1, 2, 1, 8, 7, 4},
        {8, 7, 4, 3, 0, 9, 6, 5},
        {9, 6, 5, 4, 9, 8, 7, 4},
        {4, 5, 6, 7, 8, 9, 0, 3},
        {3, 2, 0, 1, 9, 0, 1, 2},
        {0, 1, 3, 2, 9, 8, 0, 1},
        {1, 0, 4, 5, 6, 7, 3, 2},
    }

    trailhead := Position{
        X: 6, Y: 4,
    }

    result := getNinePositionsForTrailhead(input, trailhead)

    assert.ElementsMatch(t, []Position{
        {X: 5, Y: 2},
        {X: 4, Y: 3},
        {X: 5, Y: 4},
    }, result)
}

func Test_getNinePositionsForTrailhead_5(t *testing.T) {
    input := [][]int{
        {8, 9, 0, 1, 0, 1, 2, 3},
        {7, 8, 1, 2, 1, 8, 7, 4},
        {8, 7, 4, 3, 0, 9, 6, 5},
        {9, 6, 5, 4, 9, 8, 7, 4},
        {4, 5, 6, 7, 8, 9, 0, 3},
        {3, 2, 0, 1, 9, 0, 1, 2},
        {0, 1, 3, 2, 9, 8, 0, 1},
        {1, 0, 4, 5, 6, 7, 3, 2},
    }

    trailhead := Position{
        X: 2, Y: 5,
    }

    result := getNinePositionsForTrailhead(input, trailhead)

    assert.ElementsMatch(t, []Position{
        {X: 4, Y: 6},
    }, result)
}

func Test_getNinePositionsForTrailhead_6(t *testing.T) {
    input := [][]int{
        {8, 9, 0, 1, 0, 1, 2, 3},
        {7, 8, 1, 2, 1, 8, 7, 4},
        {8, 7, 4, 3, 0, 9, 6, 5},
        {9, 6, 5, 4, 9, 8, 7, 4},
        {4, 5, 6, 7, 8, 9, 0, 3},
        {3, 2, 0, 1, 9, 0, 1, 2},
        {0, 1, 3, 2, 9, 8, 0, 1},
        {1, 0, 4, 5, 6, 7, 3, 2},
    }

    trailhead := Position{
        X: 5, Y: 5,
    }

    result := getNinePositionsForTrailhead(input, trailhead)

    assert.ElementsMatch(t, []Position{
        {X: 5, Y: 2},
        {X: 4, Y: 3},
        {X: 5, Y: 4},
    }, result)
}

func Test_getNinePositionsForTrailhead_7(t *testing.T) {
    input := [][]int{
        {8, 9, 0, 1, 0, 1, 2, 3},
        {7, 8, 1, 2, 1, 8, 7, 4},
        {8, 7, 4, 3, 0, 9, 6, 5},
        {9, 6, 5, 4, 9, 8, 7, 4},
        {4, 5, 6, 7, 8, 9, 0, 3},
        {3, 2, 0, 1, 9, 0, 1, 2},
        {0, 1, 3, 2, 9, 8, 0, 1},
        {1, 0, 4, 5, 6, 7, 3, 2},
    }

    trailhead := Position{
        X: 0, Y: 6,
    }

    result := getNinePositionsForTrailhead(input, trailhead)

    assert.ElementsMatch(t, []Position{
        {X: 1, Y: 0},
        {X: 0, Y: 3},
        {X: 4, Y: 3},
        {X: 5, Y: 4},
        {X: 4, Y: 5},
    }, result)
}

func Test_getNinePositionsForTrailhead_8(t *testing.T) {
    input := [][]int{
        {8, 9, 0, 1, 0, 1, 2, 3},
        {7, 8, 1, 2, 1, 8, 7, 4},
        {8, 7, 4, 3, 0, 9, 6, 5},
        {9, 6, 5, 4, 9, 8, 7, 4},
        {4, 5, 6, 7, 8, 9, 0, 3},
        {3, 2, 0, 1, 9, 0, 1, 2},
        {0, 1, 3, 2, 9, 8, 0, 1},
        {1, 0, 4, 5, 6, 7, 3, 2},
    }

    trailhead := Position{
        X: 6, Y: 6,
    }

    result := getNinePositionsForTrailhead(input, trailhead)

    assert.ElementsMatch(t, []Position{
        {X: 5, Y: 2},
        {X: 4, Y: 3},
        {X: 5, Y: 4},
    }, result)
}

func Test_getNinePositionsForTrailhead_9(t *testing.T) {
    input := [][]int{
        {8, 9, 0, 1, 0, 1, 2, 3},
        {7, 8, 1, 2, 1, 8, 7, 4},
        {8, 7, 4, 3, 0, 9, 6, 5},
        {9, 6, 5, 4, 9, 8, 7, 4},
        {4, 5, 6, 7, 8, 9, 0, 3},
        {3, 2, 0, 1, 9, 0, 1, 2},
        {0, 1, 3, 2, 9, 8, 0, 1},
        {1, 0, 4, 5, 6, 7, 3, 2},
    }

    trailhead := Position{
        X: 1, Y: 7,
    }

    result := getNinePositionsForTrailhead(input, trailhead)

    assert.ElementsMatch(t, []Position{
        {X: 1, Y: 0},
        {X: 0, Y: 3},
        {X: 4, Y: 3},
        {X: 5, Y: 4},
        {X: 4, Y: 5},
    }, result)
}
