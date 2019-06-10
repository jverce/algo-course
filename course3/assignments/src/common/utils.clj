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

;; Find the subset of the input union-find `uf`
;; that contains the element `x`.
(defn find-uf
  [uf x]
  (first
   (filter #(contains? % x) uf)))

;; Perform a union of the subsets of `uf` that contain
;; the elements in `xs`.
(defn union-uf
  [uf & xs]
  (let [grouped-sets (group-by #(empty? (clojure.set/intersection (set xs) %)) uf)]
    (conj
     (set (get grouped-sets true))
     (apply clojure.set/union (get grouped-sets false)))))

;; Creates a Union-Find structure for a given list
;; of elements `xs`.
(defn create-uf
  [xs]
  (set (map #(identity #{%}) xs)))
