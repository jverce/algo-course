(defn acc
  [x]
  (if (= x 1)
    1
    (+ (acc (- x 1)) x)))

(defn acc-opt
  [x]
  (/ (* x (+ x 1)) 2))
