(ns week3.prob-3)

(require '[common.utils :as u])

(def data (u/data-loader "resources/week3/mwis.txt"))

;; This function takes a list of vertex weights `data`
;; and computes its Maximum-Weight Independent Set
;; iteratively. The result is a list of all the
;; intermediate calculations used to compute the final result,
;; which is the last element of such list.
(defn compute-wis
  [data]
  (loop [accum [0 (first data)]
         data (rest data)]
    (if (empty? data)
      accum
      (recur
       (conj
        accum
        (max
         (+ (first data) (last (drop-last accum)))
         (last accum)))
       (rest data)))))

;; Given a WIS list of computations (as returned by
;; `compute-wis`), determine wether the last vertex
;; belongs to the WIS.
(defn vertex-selected?
  [wis]
  (let [ultimate (last wis)
        penultimate (last (drop-last wis))]
    (not= ultimate penultimate)))

;; Given a WIS list of computations (as returned by
;; `compute-wis`), compute the *set* of vertices that
;; conform such WIS.
(defn vertices-in-wis
  [wis]
  (loop [accum wis
         idx (dec (count wis))
         idxs #{}]
    (if (empty? accum)
      idxs
      (if (vertex-selected? accum)
        ;; If the vertex `idx` is selected, then we add it to the set
        ;; of vertices `idxs` and we proceed to check the vertex after
        ;; its neighbour.
        (recur (drop-last (drop-last accum)) (dec (dec idx)) (conj idxs idx))
        ;; If the vertex `idx` is *not* selected, then we proceed to check
        ;; its neighbour.
        (recur (drop-last accum) (dec idx) idxs)))))

;; Function that determines whether the list of elements `es`
;; is part of the set `data`.
(defn list-contained
  [es data]
  (map #(contains? data %) es))

;; Function that takes a list of booleans `data`
;; and converts it to their corresponding int value.
(defn bool-to-int
  [data]
  (map #(if % 1 0) data))

;; Static list of indices for which the final result should be expressed.
(def indices '(1 2 3 4 17 117 517 997))

(def prob3
  (->> data
       flatten
       compute-wis
       vertices-in-wis
       (list-contained indices)
       bool-to-int
       (#(map str %))
       clojure.string/join))
