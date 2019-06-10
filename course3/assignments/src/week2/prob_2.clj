(ns week2.prob-2)

(require
 '[clojure.math.combinatorics :as combo]
 '[common.utils :as u])

;; Transforms a list of bits (represented as `"1"` or `"0"`)
;; into its corresponding base 10 integer.
(defn binary-list-to-dec
  [binlist]
  (apply + (map-indexed #(bit-shift-left %2 %1) (reverse binlist))))

;; Takes the input list of implicit edge costs (provided in the input)
;; and converts it to their base 10 integer representation.
(defn edges-data-as-ints
  [edges]
  (map binary-list-to-dec edges))

;; Joins key-value pairs, where values are sets.
(defn join-kv-sets
  [m kv]
  (let [key (first (keys kv))
        value (get kv key)
        existing-values (get m key)]
    (assoc m key (clojure.set/union value existing-values))))

;; Transform an input list of edge costs into
;; a map that associates an edge cost with its
;; corresponding vertex ID (vertices ID's are assumed
;; to be determined by the position in the input list).
(defn edge-costs-to-vertices-assoc
  [edges]
  (let [enumerated-entries (map-indexed #(identity {%2 #{%1}}) edges)]
    (reduce #(join-kv-sets %1 %2) {} enumerated-entries)))

;; Create a Union-Find for the list of vertices corresponding
;; to the input list `edges` (assumed implicitly based on their
;; position in the list).
(defn create-uf-for-vertices
  [edges]
  (u/create-uf (range 0 (count edges))))

;; Generate `n`-bit masks to compute Hamilton
;; differences equal to 1.
(defn generate-hdiff-masks-1bit
  [n]
  (map #(bit-shift-left 1 %) (range 0 n)))

;; Generate `n`-bit masks to compute Hamilton
;; differences equal to `d`.
(defn generate-hdiff-masks
  [n d]
  (let [masks (generate-hdiff-masks-1bit n)
        combs (apply combo/cartesian-product (repeat d masks))]
    (map #(apply bit-xor (cons 0 %)) combs)))

(defn possible-near-vertices
  [e masks vxmap]
  (mapcat #(get vxmap (bit-xor e %)) masks))

;; This function takes a vertex with edge cost `e` and a Hamiltonian diff
;; bit mask `mask`, looks for a near vertex in the vertex-to-edge map `vxmap`
;; and merges them together in the Union-Find `uf` (if the near edge exists).
(defn merge-near-vertices
  [e masks vxmap uf]
  (let [vertices (into nil (get vxmap e))
        near-vertices (possible-near-vertices e masks vxmap)]
    (apply u/union-uf uf (concat vertices near-vertices))))

;; Load input data
(defn edges-data []
  (u/data-loader "resources/week2/clustering_big.txt"))

(defn prob2 []
  (let [edge-costs (edges-data-as-ints (edges-data))
        masks (distinct
               (concat
                (repeat 24 0)
                (generate-hdiff-masks 24 1)
                (generate-hdiff-masks 24 2)))
        vxmap (edge-costs-to-vertices-assoc edge-costs)
        uf (create-uf-for-vertices edge-costs)]
    (count (reduce #(merge-near-vertices %2 masks vxmap %1) uf edge-costs))))
