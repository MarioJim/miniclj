# miniclj

Clojure-like lisp interpreter written in Rust

![Continuous Integration](https://github.com/MarioJim/miniclj/workflows/Continuous%20Integration/badge.svg)
![Lines of code](https://tokei.rs/b1/github/MarioJim/miniclj?category=code)

## Avance 1

Por el momento he implementado el 90% del lexer/parser (me falta incorporar la definición de map y mejorar la de set).
También implementé casi 30 funciones que formarán parte de mi lenguaje (están listadas en src/scope.rs, pero algunas tienen como cuerpo un todo!()).
Me falta terminar de implementar unas 5 o 6 funciones, el mecanismo de evaluación de los valores y la transformación de SExprs a sus respectivos tipos de dato.

## Avance 2

El jueves me dí cuenta que en realidad el proyecto es hacer un compilador y no un intérprete, por lo que esta semana y la siguiente me dedicaré a separar la parte del compilador y la parte del intérprete, y para esta entrega moví todo lo que tengo a la parte del compilador, mientras diseño el formato de salida del compilador.
También implementé una interfaz de subcomandos para el ejecutable, e incluí 5 opciones por ahora:

- check, que imprime un error en caso de que el lexer/parser (y próximamente compilador) encuentren una parte de la entrada que no reconozcan
- ast, para imprimir el árbol de sintaxis de un archivo (si no tiene errores de sintaxis)
- build, para compilar un archivo (por ahora no implementado)
- exec, para ejecutar un archivo compilado (tampoco implementado)
- run, para compilar y ejecutar un archivo (por ahora corre el archivo en el intérprete)

Sobre la semántica básica de variables y el cubo semántico, por ahora sólo tengo un tipo de datos numérico (una fracción de enteros de 64bits), y las operaciones aritméticas no aceptan otros tipos.
