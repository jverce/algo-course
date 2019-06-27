(ns week3.prob-1)

(require '[common.utils :as u]
         '[clojure.data.priority-map :as pm])

(def data (u/data-loader "resources/week3/huffman.txt"))

;; Takes in a list of symbol frequencies `data`
;; (each one specified as a single-element list)
;; and maps them to their corresponding symbol
;; (symbol being implicit as a function of each frequency
;; position in the input list).
;; The result is a priority-map data structure where the
;; keys are the symbols and the values are their corresponding
;; frequencies.
(defn symbol-freq-map
  [data]
  (let [enumerated-pairs (map-indexed #(into [] [%1 (first %2)]) data)
        flattened-pairs (reduce concat enumerated-pairs)]
    (apply pm/priority-map flattened-pairs)))

;; Takes a symbol-frequency priority-map `sfm` as an argument,
;; takes the 2 symbols with the smallest frequencies and merges
;; them together according to Huffman's algorithm.
;; The result is the same priority-map without the smallest
;; frequencies symbols, these being replaced by the result of
;; merging them together.
(defn merge-least-freq
  [sfm]
  (let [rest (-> sfm pop pop)
        a (first sfm)
        b (first (pop sfm))
        new-symbol (into [] '((nth a 0) (nth b 0)))
        new-freq (+ (nth a 1) (nth b 1))]
    (assoc rest new-symbol new-freq)))

;; Reduce the input symbol-frequencies priority-map `sfm`
;; by continuingly merging the least frequent symbols
;; into the map, until there are no more symbols to merge.
(defn merge-reducer
  [sfm]
  (println sfm)
  (if (= 1 (count sfm)) sfm (merge-reducer (merge-least-freq sfm))))
