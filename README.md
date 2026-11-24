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
    - Path nicht gefunden/Datei nicht gefunden wird eine erstellt
    - `pfeiltasten`     => Navigation in der Datei
    - `CTRL + D`        => Datei löschen
    - `CTRL + L`        => Zeile löschen
    - `CTRL + X`        => close file manager
    - `CTRL + S`        => Speichern der datei
    - `Home`            => Ganz links in eine Zeile
    - `End`             => Ganz rechts in eine Zeile
    - `Bild hoch`       => Ganz an Anfang vom Fenster springen
    - `Bild runter`     => Ganz ans Ende vom Fenster springen

## Projektstruktur
── README.md
── Cargo.lock
── Cargo.toml
── .gitignore
── src/
    ├── builtins.rs
    ├── fm.rs
    ├── lib.rs
    ├── main.rs
    ├── parser.rs
    └── state.rs
── tests/
    ├── builtins_tests.rs
    └── parser_tests.rs

## Tests
- `parser_tests`    => Kleiner test des parsers mit `ls -l`
- `builtins_tests`  => Tests für alle Builtins, `echo`, `pwd`, `cd`, `export`, `env`, und `is_empty`

## License
bald...