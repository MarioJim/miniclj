# miniclj

Clojure-like lisp interpreter written in Rust

![Continuous Integration](https://github.com/MarioJim/miniclj/workflows/Continuous%20Integration/badge.svg)
![Lines of code](https://tokei.rs/b1/github/MarioJim/miniclj?category=code)

## Avance 1

Por el momento he implementado el 90% del lexer/parser (me falta incorporar la definición de map y mejorar la de set).
También implementé casi 30 funciones que formarán parte de mi lenguaje (están listadas en src/scope.rs, pero algunas tienen como cuerpo un todo!()).
Me falta terminar de implementar unas 5 o 6 funciones, el mecanismo de evaluación de los valores y la transformación de SExprs a sus respectivos tipos de dato.
