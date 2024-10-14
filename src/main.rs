#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod demos;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "egui presentation",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            Ok(Box::new(egui_presentation::Presentation::new(cc)))
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast;
    use eframe::web_sys;
    // Redirect `log` messages to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        // Get a reference to the canvas element by its ID
        let canvas_id = "the_canvas_id"; // Hardcoded canvas ID
        let canvas_element = web_sys::window()
            .expect("no global `window` exists") // Ensure there's a window
            .document()
            .expect("should have a document")
            .get_element_by_id(canvas_id) // Get element by ID
            .expect("document should have the canvas element") // Ensure it exists
            .dyn_into::<web_sys::HtmlCanvasElement>() // Cast to HtmlCanvasElement
            .expect("element should be of type HtmlCanvasElement"); // Handle potential error

        eframe::WebRunner::new()
            .start(
                canvas_element, // Pass the canvas element
                web_options,
                Box::new(|cc| {
                    cc.egui_ctx.set_visuals(egui::Visuals::light()); // Force light theme
                    Ok(Box::new(egui_presentation::Presentation::new(cc))) // Return Result
                }),
            )
            .await
            .expect("failed to start eframe");
    });
}
