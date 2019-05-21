package week1

import (
	"bufio"
	util "course2/week1/util"
	"fmt"
	"log"
	"os"
	"regexp"
	"sort"
)

func Exercise() {
	// Create graph vars
	g := util.Graph{
		Edges: map[util.Vertex][]*util.Edge{},
	}
	gt := util.Graph{
		Edges: map[util.Vertex][]*util.Edge{},
	}

	// Open input file
	file, err := os.Open("./SCC.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	// Create regex to parse input file
	pattern := "(\\d+)\\s(\\d+)"
	re, err := regexp.Compile(pattern)
	if err != nil {
		log.Fatal(err)
	}

	// Parse input file and provision graph vars
	scanner := bufio.NewScanner(file)
	matched := make([]string, 2)
	var src string
	var dst string
	for scanner.Scan() {
		matched = re.FindStringSubmatch(scanner.Text())
		src, dst = matched[1], matched[2]
		g.AddEdge(src, dst, 1.)
		gt.AddEdge(dst, src, 1.)
	}

	// Compute the partial topological ordering of the graph
	// as a pre-processing step for the SCC algorithm, then
	// reverse the result.
	orders := g.Dfs()
	n := len(orders)
	for i := 0; i <= n/2; i++ {
		orders[i], orders[n-i-1] = orders[n-i-1], orders[i]
	}

	// Compute the SCC's by providing the previously computed
	// topological order.
	scc := gt.Scc(orders)

	// Computs the sizes of the SCC's, and output them to the
	// console.
	sizes := make([]int, len(scc))
	for i, x := range scc {
		sizes[i] = len(x)
	}
	sort.Ints(sizes)
	fmt.Println(sizes)
}
