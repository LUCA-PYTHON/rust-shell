# Rust Shell

Ein Terminal, das mit **Rust** entwickelt wird. Das Projekt orientiert sich teilweise an meiner [C-Shell](https://github.com/LUCA-PYTHON/C-Shell).

## Befehle

| Befehl | Beschreibung |
|---|---|
| `echo <text>` | Gibt den eingegebenen Text in einer neuen Zeile aus. |
| `pwd` | Zeigt den aktuellen Pfad an. |
| `cd <path>` | Ändert das Arbeitsverzeichnis. |
| `export <NAME=value>` | Setzt Variablen im `ShellState` (Environment). |
| `env <NAME>` | Gibt alle oder einzelne Environment-Variablen aus. |
| `fm <path>` | Öffnet den File Manager. |

## Andere Funktionen

### String-Verarbeitung

- Erkennung und Verarbeitung von doppelten Anführungszeichen (`""`).
- Mehrere Leerzeichen innerhalb von Anführungszeichen bleiben erhalten.

Beispiele:

```bash
echo "te    st"
# Ausgabe: te    st

echo te    st
# Ausgabe: te st
```

### File Manager

Dateien direkt im Terminal verwalten.

- Existiert der angegebene Pfad bzw. die Datei nicht, wird eine neue Datei erstellt.
- `Pfeiltasten` – Navigation innerhalb der Datei
- `CTRL + D` – Datei löschen
- `CTRL + L` – Zeile löschen
- `CTRL + X` – File Manager schließen
- `CTRL + S` – Datei speichern
- `CTRL + K` - Zeile ausschneiden
- `CTRL + U` - Ausgeschnittene Zeile einfügen
- `Home` – Zum Anfang der Zeile springen
- `End` – Zum Ende der Zeile springen
- `Bild hoch` – Zum Anfang des Fensters springen
- `Bild runter` – Zum Ende des Fensters springen

## Projektstruktur

```text
.
├── README.md
├── Cargo.lock
├── Cargo.toml
├── .gitignore
├── src/
│   ├── file_managment
│   │   ├── command.rs
│   │   ├── fm.rs
│   │   └── mod.rs
│   ├── builtins.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── parser.rs
│   └── state.rs
└── tests/
    ├── builtins_tests.rs
    └── parser_tests.rs
```

## Tests

| Testdatei | Beschreibung |
|---|---|
| `parser_tests` | Testet den Parser unter anderem mit `ls -l`. |
| `builtins_tests` | Testet alle Builtins: `echo`, `pwd`, `cd`, `export`, `env` und `is_empty`. |

## License

Coming soon...