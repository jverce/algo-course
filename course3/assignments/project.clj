(defproject assignments "0.1.0-SNAPSHOT"
  :description "Algo course 3"
  :url "http://example.com/FIXME"
  :license {:name "EPL-2.0 OR GPL-2.0-or-later WITH Classpath-exception-2.0"
            :url "https://www.eclipse.org/legal/epl-2.0/"}
  :dependencies [[org.clojure/clojure "1.9.0"]
                 [org.clojure/math.combinatorics "0.1.5"]]
  :main ^:skip-aot week2.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all}})
