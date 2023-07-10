fn main() {
    eframe::run_native(
        "indrex wasm demo",
        Default::default(),
        Box::new(|_cc| Box::new(index_rs_wasm::DemoApp::default())),
    );
}
