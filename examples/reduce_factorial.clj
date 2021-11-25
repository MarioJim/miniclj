(defn factorial [n]
  (if (< n 2)
    1
    (reduce * (range 1 (+ n 1)))))

(println "The factorial of" 15 "is" (factorial 15))
