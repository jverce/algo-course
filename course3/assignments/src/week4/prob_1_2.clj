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

(defn item-value
  [item]
  (first item))

(defn item-weight
  [item]
  (last item))

(def x-range
  (memoize
   (fn [data]
     (range 0 (inc (knapsack-weight data))))))

(defn mask-first-n
  [xs n]
  (let [size (count xs)
        take-size (- size n)]
    (concat (replicate n 0) (take take-size xs))))

(defn knapsack
  [data]
  (loop [items (knapsack-items data)
         accum (replicate (knapsack-weight data) 0)]
    (if (empty? items)
      accum
      (let [xs (x-range data)
            item (first items)
            vi (item-value item)
            wi (item-weight item)
            accum-with-new-item (map #(+ % vi) accum)
            accum-shifted (mask-first-n accum-with-new-item wi)]
        (recur (rest items)
               (map max accum accum-shifted))))))

(defn knapsack-value
  [data]
  (last (knapsack data)))

(defn prob1 []
  (knapsack-value (data "resources/week4/knapsack1.txt")))
(defn prob2 []
  (knapsack-value (data "resources/week4/knapsack_big.txt")))
