(ns common.utils)

(require 'clojure.set)

(defn parse-int
  [s]
  (Integer. (re-find #"-?\d+" s)))

;; Read file _lazylessly_ and return a coll,
;; with each item being a line in the input file.
(defn read-lines
  [filename]
  (with-open [r (clojure.java.io/reader filename)]
    (doall (line-seq r))))

(defn split-line-by-space-as-int
  [line]
  (map parse-int (clojure.string/split line #" ")))

;; Load all the data except for the 1st line (header).
;; Each line is split by space (`\s`) in order to separate
;; weights from lengths.
;; These are then converted to integers.
(defn data-loader
  [filename]
  (map split-line-by-space-as-int (rest (read-lines filename))))

;;
;; UNION-FIND functions
;;

;; Find the root element of the input Union-Find structure `uf`
;; that contains the element `x`.
(defn find-uf
  [uf x]
  (let [elem (get (:sets uf) x)
        parent (:p elem)]
    (if (= parent x) elem (find-uf uf parent))))

;; Function that performs the link of 2 sets of the
;; Union-Find structure `uf` for the input elements
;; `x` and `y`. The function assumes that `x` and `y`
;; are distinct root elements.
(defn link-uf
  [uf x y]
  (let [x-rank (:rank x)
        y-rank (:rank y)
        x-val (:p x)
        y-val (:p y)
        sets (:sets uf)
        size (:size uf)]
    (assoc uf
           :sets (cond
                   (= x-rank y-rank) (assoc sets
                                            x-val {:p y-val :rank x-rank}
                                            y-val {:p y-val :rank (inc y-rank)})
                   (> x-rank y-rank) (assoc sets
                                            x-val {:p x-val :rank x-rank}
                                            y-val {:p x-val :rank y-rank})
                   (< x-rank y-rank) (assoc sets
                                            x-val {:p y-val :rank x-rank}
                                            y-val {:p y-val :rank y-rank}))
           :size (dec size))))

;; Basic implementation of the UNION function that takes
;; 2 input elements `x` and `y` and performs the union
;; in the Union-Find structure `uf`.
(defn union-uf-basic
  [uf x y]
  (let [x-root (find-uf uf x)
        y-root (find-uf uf y)]
    (if (= x-root y-root) uf (link-uf uf x-root y-root))))

;; Perform a union of the subsets of `uf` that contain
;; the elements in `xs`.
(defn union-uf
  [uf & xs]
  (let [pivot (first xs)]
    (reduce #(union-uf-basic %1 %2 pivot) uf xs)))

;; Creates a Union-Find structure for a given list
;; of elements `xs`.
(defn create-uf
  [xs]
  {:sets (reduce #(assoc %1 %2 {:p %2 :rank 0}) {} xs)
   :size (count xs)})

;; Computes the size of the Union-Find structure `uf`.
(defn size-uf
  [uf]
  (:size uf))
