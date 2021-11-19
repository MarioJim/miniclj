(defn find [val list_v]
  (loop [idx 0 list_v list_v]
    (if (= val (first list_v))
      idx
      (recur (+ idx 1) (rest list_v)))))

(def list_val '(2 6 8 4 3 5))
(println "List:" list_val)
(println "Found element" 3 "in position" (find 3 list_val))