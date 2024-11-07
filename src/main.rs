use std::process;

use ckms_gui::gui_main;

#[tokio::main]
async fn main() {
    if let Some(err) = gui_main().await.err() {
        eprintln!("ERROR: {err}");
        process::exit(1);
    }
}
