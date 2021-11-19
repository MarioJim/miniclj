(def matrixA
  '('(3 6 7)
    '(5 -3 0)))

(def matrixB
  '('(1 1)
    '(2 1)
    '(3 -3)))

(defn matrix_mult [A B]
  "TODO")

(println "Matrix A:" matrixA)
(println "Matrix B:" matrixB)
(println "A x B:" (matrix_mult matrixA matrixB))
