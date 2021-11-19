(defn factorial [n]
  (loop [x n result 1]
    (if (= x 0)
      result
      (recur (- x 1) (* result x)))))

(println "The factorial of" 15 "is" (factorial 15))