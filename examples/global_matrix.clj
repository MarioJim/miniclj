(def matrixA
  '('(3 6 7)
    '(5 -3 0)))
(def dA1 (count matrixA))
(def dA2 (count (first matrixA)))

(def matrixB
  '('(1 1)
    '(2 1)
    '(3 -3)))
(def dB1 (count matrixB))
(def dB2 (count (first matrixB)))

(def matrix_mult
  (map
    (fn [idxAv]
      (do
        (def idxA idxAv)
        (map
          (fn [idxBv]
            (do
              (def idxB idxBv)
              (reduce
                +
                (map
                  #(*
                    (nth (nth matrixA idxA) %)
                    (nth (nth matrixB %) idxB))
                  (range dA2)))))
          (range dB2))))
    (range dA1)))

(println "Matrix A:" matrixA)
(println "Matrix B:" matrixB)
(println "A x B:" matrix_mult)
