mod engine;
mod input;
mod math;
mod renderer;
mod scene;
mod utils;

const WINDOW_WIDTH: usize = 1920;
const WINDOW_HEIGHT: usize = 1008;

fn main() {
    let _ = engine::EngineApp::start(WINDOW_HEIGHT as u32, WINDOW_WIDTH as u32);
}
