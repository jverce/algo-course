(ns week1.core
  (:gen-class))

(require '[week1.prob_3 :as p3])

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (p3/mst
   (conj (set nil) (p3/sort-by-weight p3/edges-data))
   (p3/sort-by-weight p3/edges-data)))
