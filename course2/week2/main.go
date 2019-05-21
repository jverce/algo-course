package main

import (
	"bufio"
	util "course2/week1/util"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
)

func main() {
	// Create graph var
	g := util.Graph{
		Edges: map[util.Vertex][]*util.Edge{},
	}

	// Open input file
	file, err := os.Open("./dijkstraData.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	// Create regex to parse input file
	pattern := `^(\d+)|\t(\d+),(\d+)`
	re, err := regexp.Compile(pattern)
	if err != nil {
		log.Fatal(err)
	}

	// Parse input file and provision graph vars
	scanner := bufio.NewScanner(file)
	var src string
	var dst string
	var w float64
	for scanner.Scan() {
		line := scanner.Text()
		components := re.FindAllStringSubmatch(line, -1)
		// Iterate over each of the edges connected to the source vertex
		// and add an edge in the graph to those destinations.
		src = components[0][0]
		for _, matched := range components[1:len(components)] {
			dst = matched[2]
			w, err = strconv.ParseFloat(matched[3], 32)
			if err != nil {
				log.Fatal(err)
			}
			g.AddEdge(src, dst, float32(w))
		}
	}

	s := util.Vertex{Data: "1"}
	dist := g.Dijkstra(s)
	targets := []int{7, 37, 59, 82, 99, 115, 133, 165, 188, 197}
	for _, t := range targets {
		v := util.Vertex{Data: strconv.Itoa(t)}
		fmt.Printf("%v,", dist[v])
	}
	fmt.Println("")
}
