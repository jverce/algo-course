package main

import (
	"bufio"
	"container/heap"
	util "course2/week3/util"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	// Initialize the heaps
	hl := &util.MaxHeap{}
	heap.Init(hl)
	hh := &util.MinHeap{}
	heap.Init(hh)

	// Open input file
	file, err := os.Open("./Median.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	medians := []int{}

	// Parse input file and stream it to heaps
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		num, err := strconv.Atoi(scanner.Text())
		if err != nil {
			log.Fatal(err)
		}

		if hl.Len() == 0 || num <= hl.Head() {
			heap.Push(hl, num)
		} else {
			heap.Push(hh, num)
		}

		// Rebalance heaps to maintain 50/50 split
		if hh.Len()-hl.Len() > 1 {
			head := heap.Pop(hh)
			heap.Push(hl, head)
		} else if hl.Len()-hh.Len() > 1 {
			head := heap.Pop(hl)
			heap.Push(hh, head)
		}

		var median int
		if hh.Len() > hl.Len() {
			median = hh.Head()
		} else {
			median = hl.Head()
		}
		medians = append(medians, median)
	}

	sum := 0
	for i := range medians {
		sum += medians[i]
	}
	result := sum % 10000
	fmt.Println(result)
}
