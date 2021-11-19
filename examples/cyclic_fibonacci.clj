(defn fibonacci [n]
  (if (<= n 1)
    n
    (loop [a 0 b 1 idx 2]
      (if (= idx n)
        (+ a b)
        (recur b (+ a b) (+ idx 1))))))

(println "The Fibonacci number" 15 "is" (fibonacci 15))