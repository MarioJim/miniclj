(def matrixA
  '('(3 6 7)
    '(5 -3 0)))

(def matrixB
  '('(1 1)
    '(2 1)
    '(3 -3)))

(defn inc [n] (+ n 1))

(defn pos_matrix_mult [A B idxA idxB len]
  (loop [result 0 idx 0]
    (if (= idx len)
      result
      (recur
        (+ result (* (nth (nth A idxA) idx) (nth (nth B idx) idxB)))
        (inc idx)))))

(defn matrix_mult [A B]
  (let [dA1 (count A) dA2 (count (first A))
        dB1 (count B) dB2 (count (first B))]
    (loop [idxA 0 idxB 0 result [] row []]
      (if (= idxA dA1)
        result
        (if (= idxB dB2)
          (recur (inc idxA) 0 (conj result row) [])
          (recur idxA (inc idxB) result
            (conj row (pos_matrix_mult A B idxA idxB dA2))))))))

(println "Matrix A:" matrixA)
(println "Matrix B:" matrixB)
(println "A x B:" (matrix_mult matrixA matrixB))
(println "B x A:" (matrix_mult matrixB matrixA))
