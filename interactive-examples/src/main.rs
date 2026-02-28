use ply_engine::prelude::*;

mod examples;
mod interpreter;

// ---------------------------------------------------------------------------
// WASM FFI — read strings from JS plugins
// ---------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
extern "C" {
    // Home page demo: read code from CodeMirror
    fn ply_demo_get_code(ptr: *mut u8, max_len: u32) -> u32;
    // Doc examples: read example ID and params from URL
    fn ply_example_get_id(ptr: *mut u8, max_len: u32) -> u32;
    fn ply_example_get_params(ptr: *mut u8, max_len: u32) -> u32;
    // Doc examples: log a message to the console panel
    fn ply_example_log(ptr: *const u8, len: u32);
}

/// Log a message to the in-page console panel (WASM only, no-op native).
#[allow(unused_variables)]
pub fn log_to_console(msg: &str) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        ply_example_log(msg.as_ptr(), msg.len() as u32);
    }
}

#[cfg(target_arch = "wasm32")]
fn read_wasm_str(f: unsafe extern "C" fn(*mut u8, u32) -> u32, cap: usize) -> String {
    let mut buf = vec![0u8; cap];
    let len = unsafe { f(buf.as_mut_ptr(), buf.len() as u32) } as usize;
    String::from_utf8_lossy(&buf[..len]).into_owned()
}

#[cfg(target_arch = "wasm32")]
fn get_demo_code() -> String {
    read_wasm_str(ply_demo_get_code, 16384)
}

#[cfg(target_arch = "wasm32")]
fn get_example_id() -> String {
    read_wasm_str(ply_example_get_id, 1024)
}

#[cfg(target_arch = "wasm32")]
fn get_example_params_raw() -> String {
    read_wasm_str(ply_example_get_params, 16384)
}

// ---------------------------------------------------------------------------
// Native fallback (cargo run)
// ---------------------------------------------------------------------------

#[cfg(not(target_arch = "wasm32"))]
fn get_demo_code() -> String {
    include_str!("default_code.ply").to_owned()
}

#[cfg(not(target_arch = "wasm32"))]
fn get_example_id() -> String {
    String::new()
}

#[cfg(not(target_arch = "wasm32"))]
fn get_example_params_raw() -> String {
    String::new()
}

// ---------------------------------------------------------------------------

fn window_conf() -> macroquad::conf::Conf {
    macroquad::conf::Conf {
        miniquad_conf: miniquad::conf::Conf {
            window_title: "Ply Interactive Examples".to_owned(),
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
    static DEFAULT_FONT: FontAsset = FontAsset::Bytes {
        file_name: "lexend.ttf",
        data: include_bytes!("../assets/fonts/lexend.ttf"),
    };
    let mut ply = Ply::<()>::new(&DEFAULT_FONT).await;

    loop {
        clear_background(MacroquadColor::new(0.102, 0.067, 0.067, 1.0));

        let example_id = get_example_id();
        let mut ui = ply.begin();

        if !example_id.is_empty() {
            // Doc example mode: run the compiled example function
            let raw = get_example_params_raw();
            let params = examples::parse_params(&raw);
            examples::run(&example_id, &params, &mut ui);
        } else {
            // Home page demo: run the interpreter
            let code = get_demo_code();
            match interpreter::interpret(&code, &mut ui) {
                Ok(()) => {}
                Err(err) => interpreter::render_error(&mut ui, &err),
            }
        }

        ui.show(|_| {}).await;

        next_frame().await;
    }
}
