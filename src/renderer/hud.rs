use crate::renderer::Renderer;
use crate::types::color::ColorRGB;

pub struct Hud;

impl Hud {
    pub fn draw(
        renderer: &mut Renderer,
        current_fps: u32,
        draw_keybinds: bool,
        draw_grid: bool,
        draw_axis: bool,
        draw_lights: bool,
    ) {
        renderer.render_text("[ESC] Exit", 0, 0, ColorRGB::WHITE, 2);

        let keybind_hud_pos = (150, 0);

        renderer.render_text("Niko Tepe - 2025", 0, 980, ColorRGB::WHITE, 2);

        let fps_test = format!("FPS: {}", current_fps);

        renderer.render_text(fps_test.as_str(), 0, 960, ColorRGB::WHITE, 2);

        if !draw_keybinds {
            renderer.render_text(
                "[F1] Keybinds",
                keybind_hud_pos.0,
                keybind_hud_pos.1,
                ColorRGB::GRAY_MEDIUM,
                2,
            );
        } else {
            renderer.render_text(
                "[F1] Keybinds",
                keybind_hud_pos.0,
                keybind_hud_pos.1,
                ColorRGB::WHITE,
                2,
            );

            renderer.render_text(
                "|",
                keybind_hud_pos.0,
                keybind_hud_pos.1 + 20,
                ColorRGB::WHITE,
                2,
            );

            renderer.render_text(
                "|- [G] Grid",
                keybind_hud_pos.0,
                keybind_hud_pos.1 + 40,
                if draw_grid {
                    ColorRGB::WHITE
                } else {
                    ColorRGB::GRAY_MEDIUM
                },
                2,
            );

            renderer.render_text(
                "|- [H] Faces",
                keybind_hud_pos.0,
                keybind_hud_pos.1 + 60,
                if renderer.draw_faces {
                    ColorRGB::WHITE
                } else {
                    ColorRGB::GRAY_MEDIUM
                },
                2,
            );

            renderer.render_text(
                "|- [K] Axis",
                keybind_hud_pos.0,
                keybind_hud_pos.1 + 80,
                if draw_axis {
                    ColorRGB::WHITE
                } else {
                    ColorRGB::GRAY_MEDIUM
                },
                2,
            );

            renderer.render_text(
                "|- [L] Light Vectors",
                keybind_hud_pos.0,
                keybind_hud_pos.1 + 100,
                if draw_lights {
                    ColorRGB::WHITE
                } else {
                    ColorRGB::GRAY_MEDIUM
                },
                2,
            );

            renderer.render_text(
                "|",
                keybind_hud_pos.0,
                keybind_hud_pos.1 + 120,
                ColorRGB::WHITE,
                2,
            );

            renderer.render_text(
                "|- [Z] Z-Buffer",
                keybind_hud_pos.0,
                keybind_hud_pos.1 + 140,
                if renderer.draw_z_buffer {
                    ColorRGB::WHITE
                } else {
                    ColorRGB::GRAY_MEDIUM
                },
                2,
            );

            renderer.render_text(
                "|- [X] Wireframe",
                keybind_hud_pos.0,
                keybind_hud_pos.1 + 160,
                if renderer.draw_wireframe {
                    ColorRGB::WHITE
                } else {
                    ColorRGB::GRAY_MEDIUM
                },
                2,
            );

            renderer.render_text(
                "|- [C] Vertices",
                keybind_hud_pos.0,
                keybind_hud_pos.1 + 180,
                if renderer.draw_vertex {
                    ColorRGB::WHITE
                } else {
                    ColorRGB::GRAY_MEDIUM
                },
                2,
            );

            renderer.render_text(
                "|- [V] Vertex Normals",
                keybind_hud_pos.0,
                keybind_hud_pos.1 + 200,
                if renderer.draw_vertex_normals {
                    ColorRGB::WHITE
                } else {
                    ColorRGB::GRAY_MEDIUM
                },
                2,
            );

            renderer.render_text(
                "|- [B] Backface Culling",
                keybind_hud_pos.0,
                keybind_hud_pos.1 + 220,
                if renderer.backface_culling {
                    ColorRGB::WHITE
                } else {
                    ColorRGB::GRAY_MEDIUM
                },
                2,
            );

            renderer.render_text(
                "|",
                keybind_hud_pos.0,
                keybind_hud_pos.1 + 240,
                ColorRGB::WHITE,
                2,
            );

            renderer.render_text(
                "|- [Arrow Keys] Move Camera",
                keybind_hud_pos.0,
                keybind_hud_pos.1 + 260,
                ColorRGB::WHITE,
                2,
            );

            renderer.render_text(
                "|- [Mouse 1]    Move Camera",
                keybind_hud_pos.0,
                keybind_hud_pos.1 + 280,
                ColorRGB::WHITE,
                2,
            );

            renderer.render_text(
                "|- [W, A, S, D] Move Light Vectors",
                keybind_hud_pos.0,
                keybind_hud_pos.1 + 300,
                ColorRGB::WHITE,
                2,
            );

            renderer.render_text(
                "|",
                keybind_hud_pos.0,
                keybind_hud_pos.1 + 320,
                ColorRGB::WHITE,
                2,
            );

            renderer.render_text(
                "|- [O/P] Change FOV",
                keybind_hud_pos.0,
                keybind_hud_pos.1 + 340,
                ColorRGB::WHITE,
                2,
            );

            renderer.render_text(
                "|- [N/M] Change Scale",
                keybind_hud_pos.0,
                keybind_hud_pos.1 + 360,
                ColorRGB::WHITE,
                2,
            );
        }
    }
}
