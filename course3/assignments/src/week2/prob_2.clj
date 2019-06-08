(ns week2.prob-2)

(require '[common.utils :as u]
         '[week1.prob-3 :as w1p3]
         '[week2.prob-1 :as w2p1])

;; Load input data
(def edges-data
  (u/data-loader "resources/week2/clustering_big.txt"))

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
;; a map that associates each vertex ID with its
;; corresponding edge cost (vertices ID's are assumed
;; to be determined by the position in the input list).
(defn edge-costs-to-vertices-assoc
  [edges]
  (let [enumerated-entries (map-indexed #(identity {%1 %2}) edges)]
    (reduce #(conj %1 %2) {} enumerated-entries)))

;; Create a Union-Find for the list of vertices corresponding
;; to the input list `edges` (assumed implicitly based on their
;; position in the list).
(defn create-uf-for-vertices
  [edges]
  (u/create-uf (range 0 (count edges))))
