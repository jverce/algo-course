(ns week2.prob_1)

(require '[common.utils :as u])
(require '[week1.prob-3 :as w1p3])

(defn find-uf
  [sets x]
  (first
   (filter #(contains? % x) sets)))

(defn find-uf-complement
  [sets & xs]
  (clojure.set/difference sets (set (map #(find-uf sets %) xs))))

(defn union-uf
  [sets & xs]
  (conj
   (apply find-uf-complement sets xs)
   (apply clojure.set/union
          (map #(find-uf sets %) xs))))

(def edges-data
  (w1p3/sort-by-weight
   (u/data-loader "resources/week2/input_completeRandom_10_32.txt")))

(defn assoc-points
  [clusters e]
  union-uf (:groups clusters) )

(defn clusters
  [k]
  ())
