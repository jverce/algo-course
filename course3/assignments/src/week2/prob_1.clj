(ns week2.prob-1)

(require '[common.utils :as u]
         '[week1.prob-3 :as w1p3])

;; Create the Union-Find structure for the input edges data.
(defn initial-cluster
  [edges]
  (u/create-uf (into nil (w1p3/vertices-of-edge-records edges))))

;; Creates the initial hash-map structure to use for
;; repetitive computations:
;; - `:groups` refers to the Union-Find structure outlining the
;;             clustering.
;; - `:distance` refers to the max distance for this particular
;;               clustering.
(defn initial-map
  [edges]
  {:groups (initial-cluster edges) :distance 0})

;; Given a cluster map `clusters` and a new edge, perform
;; the clustering computations for the vertices of such edge.
(defn assoc-points
  [clusters e]
  (let [new-groups (apply u/union-uf (:groups clusters) (w1p3/vertices e))]
    {:groups new-groups
     :distance (w1p3/weight e)}))

;; Compute the size of the clustering represented
;; by the Union-Find structure `uf`.
(defn cluster-size
  [uf]
  (count (distinct (vals uf))))

;; Compute the k-cluster of the list of edges `edges`.
;; This list must be sorted in ascending cost order for this
;; function to compute the max distance clustering.
(defn clusters
  [edges k]
  (first (drop-while
          #(>= (cluster-size (:groups %)) k)
          (reductions assoc-points (initial-map edges) edges))))

;; Load input data.
(defn edges-data []
  (w1p3/sort-by-weight
   (u/data-loader "resources/week2/clustering1.txt")))

(defn prob1 [] (:distance (clusters (edges-data) 4)))
