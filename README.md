#                                                      AJEDREZ
## Consigna
Dada la posicion de dos piezas (una negra y una blanca), y solo dos piezas, determinar si:

- La pieza negra puede capturar a la blanca en su siguiente movimiento.
- La pieza blanca puede capturar a la negra en su siguiente movimiento.
- Ambas piezas pueden capturar a la otra en su siguiente movimiento.
- Ninguna de las piezas puede capturar a la otra.
Aclaración: es indistinto qué color se movera en el siguiente turno.

Para identificar cada pieza se utilizaran letras: Rey [R], Dama [D], Alfil [A], Caballo [C], Torre [T], Peon [P].
Las blancas se identificaran con letras minúsculas, mientras que las negras con letras MAYÚSCULAS. 
Cualquier casilla vacia sera representada con guion bajo [_]. Para cada fila del tablero, cada casillero se encuentra separado por un espacio [ ].

## Formato del output
El output sera un caracter impreso por terminal:

- B: indica que solo la pieza blanca pueden capturar.
- N: indica que solo la pieza negra pueden capturar.
- E: indica que ambas piezas pueden capturar.
- P: indica que ninguna pieza puede capturar.
En caso de que un error ocurriese, se deberá imprimir un mensaje con el siguiente formato: ERROR: [descripcion_del_error].

## Consideraciones
La orientación del tablero dispondrá a las piezas blancas en la parte de abajo, y a las piezas negras en la parte de arriba 
Esto quiere decir que los peones blancos se moverán y capturarán hacia "arriba", mientras que los peones negros hacia "abajo".

## Formato del input

El input es un archivo en el filesystem con el formato de entrada del tablero. 
En la invocación del programa se debe proveer la ruta a ese archivo. Solo deberá ingresarse este argumento.

*cargo run -- table.txt*
