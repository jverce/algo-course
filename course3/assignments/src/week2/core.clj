(ns week2.core
  (:gen-class))

(require '[week2.prob-1 :as p1])
(require '[week2.prob-2 :as p2])

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println (p1/prob1))
  (println (p2/prob2)))
