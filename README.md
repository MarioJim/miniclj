# miniclj

Clojure-like lisp compiler and VM written in Rust

![Continuous Integration](https://github.com/MarioJim/miniclj/workflows/Continuous%20Integration/badge.svg)
![Continuous Deployment](https://github.com/MarioJim/miniclj/workflows/Continuous%20Deployment/badge.svg)
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

## Avance 3

Sigo trabajando en separar el compilador y la máquina virtual del intérprete. En esta entrega empecé a definir el estado del compilador y de los espacios en memoria para así definir una función `State::compile` que reciba una expresión y añada al estado del compilador las expresiones descompuestas de la expresión padre.
Todavía tengo algunas dudas sobre cómo será la estructura de los datos en la tabla de símbolos (qué tengo que guardar y cómo) pero en eso avanzaré la siguiente semana.

## Avance 4

Durante esta semana no avancé tanto como me hubiera gustado, pero definí cómo voy a hacer referencias a la memoria durante la ejecución, y estoy empezando a escribir las partes del compilador que imprimen los cuádruplos. Estoy pensando en hacer el compilador sin tipos, y checar eso en la máquina virtual

## Avance 5

Durante la semana i y la semana pasada avancé hasta casi terminar el proyecto: ya compila y ejecuta funciones, condicionales y ciclos. Por ahora tengo un par de ideas "extras", aunque debería empezar con la documentación:

- Añadir funciones como:
  - spit/slurp (recibe el nombre de un archivo y lo escribe/lee como string)
  - inc/dec (incrementan o decrementan un número por uno)
  - mod (módulo de una división entre dos números)
  - rand/rand-int (devuelven un número decimal o entero aleatorio)
  - range (recibe uno, dos o tres números, como la función de Python regresa una lista de números)
  - repeat (repite un valor n veces)
  - sort/sort-by (ordenan una lista por su valor o por el valor regresado por una función)
  - pow (número elevado a otro número)
  - apply (recibe una función y una lista, llama a la función con los elementos de la lista como argumentos)
  - split (para strings, parte una string por un patrón)
  - min/max (encuentra el mínimo y máximo entre dos números)
  - drop/take (tira o toma los primeros n elementos de una lista)
  - drop-while/take-while (tira o toma los elementos de una lista hasta que la condición se vuelva falsa)
  - into (castea una collección a otro tipo de collección)
  - -> y ->> (reciben una lista de funciones parciales y las encadenan usando el resultado de la anterior como el primer o último argumento de la siguiente llamada)
- Compilar el proyecto en wasm y hacer una página web "playground" en la que de un lado se pueda escribir el código, y del otro poder ver el árbol de sintaxis, o el bytecode del compilador, o directamente el output de ejecutar el código
- Implementar más tests para las funciones del compilador (sólo +,-,\*,/,=,!=,<,>,<=,>= tienen tests unitarios)
