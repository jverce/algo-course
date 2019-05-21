package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() { // Open input file
	file, err := os.Open("./algo1-programming_prob-2sum.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	table := map[int64]bool{}

	// Parse input file and stream it to heaps
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		num, err := strconv.ParseInt(scanner.Text(), 10, 64)
		if err != nil {
			log.Fatal(err)
		}
		table[num] = true
	}

	count := 0
	var t int64
	for t = -10000; t <= 10000; t++ {
		for k := range table {
			if t != 2*k {
				complement := t - k
				if table[complement] {
					count++
					fmt.Printf("%d + %d = %d\n", k, complement, t)
					break
				}
			}
		}
	}
	fmt.Println(count)
}
