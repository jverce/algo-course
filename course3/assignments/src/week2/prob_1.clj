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
   (u/data-loader "resources/week2/clustering1.txt")))

(defn initial-cluster
  [edges]
  (set (map
        #(identity #{%})
        (into nil (w1p3/vertices-of-edge-records edges)))))

(defn initial-map
  [edges]
  {:groups (initial-cluster edges) :distance 0})

(defn assoc-points
  [clusters e]
  (let [new-groups (apply union-uf (:groups clusters) (w1p3/vertices e))]
    {:groups new-groups
     :distance (w1p3/weight e)}))

(defn clusters
  [edges k]
  (first (drop-while
         #(>= (count (:groups %)) k)
         (reductions assoc-points (initial-map edges) edges))))
