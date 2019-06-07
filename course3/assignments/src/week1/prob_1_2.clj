(ns week1.prob-1-2)

(require '[common.utils :as u])

;; Easy indexers.
(defn weight [x] (first x))
(defn length [x] (nth x 1))

(defn wl-ratio
  [x]
  (/ (weight x) (length x)))

(defn wl-diff
  [x]
  (- (weight x) (length x)))

(defn wl-prod
  [x]
  (* (weight x) (length x)))


;; Take 2 processes `a` and `b` and add
;; their corresponding lenghts.
(defn add-lengths
  [a b]
  (list
   (weight b)
   (+ (length a) (length b))))

;; Time `t` when a process is actually fully executed.
;; It is computed as the exec time of the previous process
;; plus the current process length.
(defn exec-time
  [procs]
  (reductions add-lengths procs))

;; Map-reduce to compute sum of weighted lengths.
(defn weighted-sum
  [procs]
  (apply + (map wl-prod procs)))

;; Sort processes by a comparator function `fn`.
;; In case of ties, sort them by their weight in descendent order.
(defn sorted-procs
  [procs fn]
  (sort-by fn > (sort-by weight > procs)))

(defn procs [] (u/data-loader "resources/week1/jobs.txt"))
(defn prob1 [] (weighted-sum (exec-time (sorted-procs (procs) wl-diff))))
(defn prob2 [] (weighted-sum (exec-time (sorted-procs (procs) wl-ratio))))
