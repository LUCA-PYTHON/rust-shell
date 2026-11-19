# Rust Shell

Hier soll ein Terminal in rust enstehen. Vegrleich zu [C-Shell](https://github.com/LUCA-PYTHON/C-Shell).

## Befehle
- `echo <text>` => Gibt den eingebenen Text in einer neuen Zeile zurück
- `pwd`         => Zeigt den Aktuellen Pfad an
- `cd <path>`   => Pfad ändern in dem man arbeiten möchte

## Andere Funktionen
- `""` Erkennung und Verarbeitung
    - `echo "te    st"` => Ausgabe: te    st
    - `echo te    st    => Ausgabe: te st

## Tests
- `parser_tests`    => Kleiner test des parsers mit `ls -l`
- `builtins_tests`  => Tests für alle Builtins, `echo`, `pwd`, `cd` und `is_empty`

## License
bald...