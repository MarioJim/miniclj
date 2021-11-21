(defn frequencies [l]
  (loop [l l result {}]
    (if (empty? l)
      result
      (recur
        (rest l)
        (let [val (first l) n (get result val)]
          (if n
            (conj result [val (+ n 1)])
            (conj result [val 1])))))))

(defn cmp-entry [a b]
  (if (> (first a) (first b))
    a b))

(defn sort-list [l]
  (let [freq-map (frequencies l)]
    (loop [freqs freq-map result '()]
      (if (empty? freqs)
        result
        (let [max-entry (reduce cmp-entry freqs)
              val (first max-entry)
              freq (first (rest max-entry))]
          (recur
            (if (= freq 1)
              (del freqs val)
              (conj freqs [val (- freq 1)]))
            (cons val result)))))))

(def l '(3 6 1 7 8 2 7))

(println "List:" l)
(println "Sorted list:" (sort-list l))
