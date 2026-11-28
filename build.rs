fn main() {
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("ProductName", "Rust Shell");
        res.set("FileDescription", "Rust Shell");
        res.compile()
            .expect("Windows-Icon konnte nicht eingebettet werden");
    }
}
