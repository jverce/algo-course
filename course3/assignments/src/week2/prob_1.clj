(ns week2.prob-1)

(require '[common.utils :as u])
(require '[week1.prob-3 :as w1p3])

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
  (let [new-groups (apply u/union-uf (:groups clusters) (w1p3/vertices e))]
    {:groups new-groups
     :distance (w1p3/weight e)}))

(defn clusters
  [edges k]
  (first (drop-while
         #(>= (count (:groups %)) k)
         (reductions assoc-points (initial-map edges) edges))))
