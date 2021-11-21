(defn fibonacci [n]
  (if (<= n 1)
    n
    (+ (fibonacci (- n 1)) (fibonacci (- n 2)))))

(println "The Fibonacci number" 15 "is" (fibonacci 15))
