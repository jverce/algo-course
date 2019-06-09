(ns week2.prob-2)

(require
 '[clojure.math.combinatorics :as combo]
 '[common.utils :as u])

;; Transforms a list of bits (represented as `"1"` or `"0"`)
;; into its corresponding base 10 integer.
(defn binary-list-to-dec
  [binlist]
  (Integer/parseInt (clojure.string/join binlist) 2))

;; Takes the input list of implicit edge costs (provided in the input)
;; and converts it to their base 10 integer representation.
(defn edges-data-as-ints
  [edges]
  (map binary-list-to-dec edges))

;; Transform an input list of edge costs into
;; a map that associates an edge cost with its
;; corresponding vertex ID (vertices ID's are assumed
;; to be determined by the position in the input list).
(defn edge-costs-to-vertices-assoc
  [edges]
  (let [enumerated-entries (map-indexed #(identity {%2 %1}) edges)]
    (reduce #(conj %1 %2) {} enumerated-entries)))

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
        combs (filter #(apply distinct? %)
                      (apply combo/cartesian-product (repeat d masks)))]
    (distinct (map #(apply bit-xor (cons 0 %)) combs))))

;; Load input data
(defn edges-data []
  (u/data-loader "resources/week2/clustering_big.txt"))

(defn prob2 [] true)
