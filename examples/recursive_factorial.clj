(defn factorial [n]
  (if (= n 0)
    1
    (* n (factorial (- n 1)))))

(println "The factorial of" 15 "is" (factorial 15))