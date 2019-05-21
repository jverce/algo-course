(ns week1.prob_3)

(require '[common.utils :as u])

(def edges-data
  (eliminate-self-loops
   (sort-by-weight
    (u/data-loader "resources/edges.txt"))))

(defn vertices [e] (set (take 2 e)))
(defn weight [e] (nth e 2))

;; Sort all the edge records by their weight
;; in ascending order.
(defn sort-by-weight
  [es]
  (sort-by weight es))

;; Calculates the total weight of the graph `es`.
(defn total-weight
  [es]
  (reduce + (map weight es)))

;; Returns the set of vertices present in the
;; collection of edge records `es`.
(defn vertices-of-edge-records
  [es]
  (reduce clojure.set/union (map vertices es)))

;; Check if the edge record `e`
;; crosses the partition `vs`.
(defn edge-crosses-partition?
  [e vs]
  (= 1 (count (clojure.set/intersection (vertices e) vs))))

;; Extracts the next eligible element from the list of
;; edge records `es`, and returns a tuple containing the
;; extracted element as well as the rest of the list.
(defn find-and-extract
  [es vs]
  (let [index (first (keep-indexed #(if (edge-crosses-partition? %2 vs) %1) es))]
    (if (nil? index)
      nil
      {:item (nth es index)
       :rest (concat (subvec (vec es) 0 index) (subvec (vec es) (inc index)))})))

;; MST algorithm, implemented in a recursive manner
;; by sorting the edges by weight (in ascending order) and
;; selecting only edges that add new vertices to the MST.
(defn mst-computation
  [st es]
  (let [next-edge (find-and-extract es (vertices-of-edge-records st))]
    (if (nil? next-edge)
      st
      (mst-computation (conj st (:item next-edge)) (:rest next-edge)))))

;; Eliminate self-loops from graph `es`.
(defn eliminate-self-loops
  [es]
  (filter #(> (count (vertices %)) 1) es))

(def prob3
  (total-weight
   (mst-computation #{(first edges-data)} (rest edges-data))))
