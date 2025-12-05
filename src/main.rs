use crate::app::App;

mod app;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "guidn",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
    .expect("app error thing");
}
