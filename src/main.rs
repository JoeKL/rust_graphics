mod engine;
mod math;
mod renderer;
mod scene;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let _ = engine::EngineApp::start();
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect log messages to the browser console:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        use eframe::wasm_bindgen::JsCast;

        let document = web_sys::window()
            .and_then(|w| w.document())
            .expect("Failed to get window or document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id is not a canvas");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(engine::EngineApp::new(cc, 800, 600)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            loading_text.remove();
        }

        if let Err(e) = start_result {
            log::error!("Failed to start eframe: {:?}", e);
        }
    });
}
