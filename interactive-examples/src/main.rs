use ply_engine::prelude::*;

mod interpreter;

// --- WASM: read code from JS each frame ---

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn ply_demo_get_code(ptr: *mut u8, max_len: u32) -> u32;
}

#[cfg(target_arch = "wasm32")]
fn get_demo_code() -> String {
    let mut buf = [0u8; 16384]; // 16 KB max
    let len = unsafe { ply_demo_get_code(buf.as_mut_ptr(), buf.len() as u32) };
    String::from_utf8_lossy(&buf[..len as usize]).into_owned()
}

// --- Native: use the default code for `cargo run` testing ---

#[cfg(not(target_arch = "wasm32"))]
fn get_demo_code() -> String {
    include_str!("default_code.ply").to_owned()
}

fn window_conf() -> macroquad::conf::Conf {
    macroquad::conf::Conf {
        miniquad_conf: miniquad::conf::Conf {
            window_title: "Ply Interpreter Test".to_owned(),
            window_width: 800,
            window_height: 600,
            high_dpi: true,
            sample_count: 4,
            platform: miniquad::conf::Platform {
                webgl_version: miniquad::conf::WebGLVersion::WebGL2,
                ..Default::default()
            },
            ..Default::default()
        },
        draw_call_vertex_capacity: 100000,
        draw_call_index_capacity: 100000,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    static DEFAULT_FONT: FontAsset = FontAsset::Path("assets/fonts/lexend.ttf");
    let mut ply = Ply::<()>::new(&DEFAULT_FONT).await;

    loop {
        clear_background(MacroquadColor::new(0.0, 0.0, 0.0, 1.0));

        let code = get_demo_code();
        let mut ui = ply.begin();

        match interpreter::interpret(&code, &mut ui) {
            Ok(()) => {}
            Err(err) => interpreter::render_error(&mut ui, &err),
        }

        ui.show(|_| {}).await;

        next_frame().await;
    }
}
