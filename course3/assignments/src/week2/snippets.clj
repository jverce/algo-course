(defn find-uf
  [sets x]
  (first (filter #(contains? % x) sets)))

(defn union-uf
  [sets a b]
  (set
   (clojure.set/union
    (filter #(and (not (contains? % a)) (not (contains? % b))) sets)
    (list (clojure.set/union (find-uf sets a) (find-uf sets b))))))
