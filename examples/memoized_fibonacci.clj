(def fibonacci-memo [0 1])

(defn fibonacci [n]
  (if (< n (count fibonacci-memo))
    (get fibonacci-memo n)
    (do
      (def fibonacci-memo
        (let [min1 (fibonacci (- n 1)) min2 (fibonacci (- n 2))]
          (conj fibonacci-memo (+ min1 min2))))
      (get fibonacci-memo n))))

(def num 15)
(println "The fibonacci of" num "is" (fibonacci num))
