(def roman-nums
  '('(100 "C")
    '(90 "XC")
    '(50 "L")
    '(40 "XL")
    '(10 "X")
    '(9 "IX")
    '(5 "V")
    '(4 "IV")
    '(1 "I")))

(defn convert-to-roman [n]
  (loop [numv n result ""]
    (if (= numv 0)
      result
      (do
        (def num numv)
        (let [next-tup (first (filter #(>= num (first %)) roman-nums))]
          (recur
            (- numv (first next-tup))
            (str result (first (rest next-tup)))))))))

(println "Roman notation of" 2 "is" (convert-to-roman 2))
(println "Roman notation of" 3 "is" (convert-to-roman 3))
(println "Roman notation of" 9 "is" (convert-to-roman 9))
(println "Roman notation of" 14 "is" (convert-to-roman 14))
(println "Roman notation of" 44 "is" (convert-to-roman 44))
(println "Roman notation of" 99 "is" (convert-to-roman 99))
