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
