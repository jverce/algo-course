(ns week3.core
  (:gen-class))

(require '[week3.prob-1-2 :as p1p2]
         '[week3.prob-3 :as p3])

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println p1p2/prob1)
  (println p1p2/prob2)
  (println p3/prob3))
