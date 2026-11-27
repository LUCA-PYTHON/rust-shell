set -e
cargo build --release --bin rust-shell
mkdir -p "$HOME/.local/bin" "$HOME/.local/share/applications" "$HOME/.local/share/icons/hicolor/1024x1024/apps"
cp target/release/rust-shell "$HOME/.local/bin/rust-shell"
cp assets/icon.png "$HOME/.local/share/icons/hicolor/1024x1024/apps/rust-shell.png"
cp packaging/rust-shell.desktop "$HOME/.local/share/applications/rust-shell.desktop"
update-desktop-database "$HOME/.local/share/applications" 2>/dev/null || true
echo "Fertig. Starte mit 'rust-shell' oder ueber das Anwendungsmenue."
echo "Falls 'rust-shell' nicht gefunden wird: ~/.local/bin muss im PATH sein."