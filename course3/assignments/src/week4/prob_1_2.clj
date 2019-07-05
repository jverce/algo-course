(ns week4.prob-1-2)

(require '[common.utils :as u])

(defn data
  [filename]
  (u/data-loader filename true))

;; Extract the knapsack total weight/size
;; from the input `data`.
(defn knapsack-weight
  [data]
  (first (first data)))

;; Extract the items to be stored in the
;; knapsack from the input `data`.
(defn knapsack-items
  [data]
  (rest data))

;; Given an item `item` as extracted from a Knapsasck
;; input file, return its value.
(defn item-value
  [item]
  (first item))

;; Given an item `item` as extracted from a Knapsasck
;; input file, return its weight.
(defn item-weight
  [item]
  (last item))

;; Return a list of consecutive weights, from 0
;; to the total weight of the Knapsack. `data`
;; represents the whole input for the Knapsack problem.
(def x-range
  (memoize
   (fn [data]
     (range 0 (inc (knapsack-weight data))))))

;; This function takes a list `xs` and generates
;; a new list of the same size, with its first `n`
;; elements _shifted_ to the right. The first `n`
;; elements of the new list are set to 0.
(defn mask-first-n
  [xs n]
  (let [size (count xs)
        take-size (- size n)]
    (concat (repeat n 0) (take take-size xs))))

;; Return the optimal value of the Knapsack problem
;; for the input `data`.
(defn knapsack
  [data]
  (loop [items (knapsack-items data)
         accum (repeat (knapsack-weight data) 0)]
    (if (empty? items)
      (last accum)
      (let [xs (x-range data)
            item (first items)
            vi (item-value item)
            wi (item-weight item)
            accum-with-new-item (map #(+ % vi) accum)
            accum-shifted (mask-first-n accum-with-new-item wi)]
        (recur (rest items)
               (map max accum accum-shifted))))))

(defn prob1 []
  (knapsack (data "resources/week4/knapsack1.txt")))
(defn prob2 []
  (knapsack (data "resources/week4/knapsack_big.txt")))
