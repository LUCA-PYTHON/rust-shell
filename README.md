# Rust Shell

Hier soll ein Terminal in rust enstehen. Vergleich zu [C-Shell](https://github.com/LUCA-PYTHON/C-Shell).

## Befehle
- `echo <text>`         => Gibt den eingebenen Text in einer neuen Zeile zurück
- `pwd`                 => Zeigt den Aktuellen Pfad an
- `cd <path>`           => Pfad ändern in dem man arbeiten möchte
- `export <NAME=value>` => Variablen in ShellState (env) setzen
- `env <NAME>`          => Alle/Einzelene Daten aus dem Envoierment holen
- `fm <path>`           => File Manager öffnen

## Andere Funktionen
- `""` Erkennung und Verarbeitung
    - `echo "te    st"` => Ausgabe: `te    st`
    - `echo te    st`   => Ausgabe: `te st`
- `File Manager` Dateien verwalten im Terminal
    - `CTRL + X`        => close file manager

## Tests
- `parser_tests`    => Kleiner test des parsers mit `ls -l`
- `builtins_tests`  => Tests für alle Builtins, `echo`, `pwd`, `cd` und `is_empty`

## License
bald...