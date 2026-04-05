use ply_engine::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

static SEEDED: AtomicBool = AtomicBool::new(false);

const PLAYGROUND_SHADER: ShaderAsset = ShaderAsset::Stored("playground");

const DEFAULT_GLSL: &str = "\
#version 100
precision highp float;

varying lowp vec2 uv;
uniform sampler2D Texture;
uniform float u_time;
uniform vec2 u_resolution;

void main() {
  vec2 p = uv * 2.0 - 1.0;
  p.x *= u_resolution.x / u_resolution.y;
  float d = length(p) - 0.4;
  float ring = smoothstep(0.1, 0.0, abs(d));
  float hue = atan(p.y, p.x) / 6.2832 + u_time * 0.2;
  vec3 col = 0.5 + 0.5 * cos(6.2832 * (hue + vec3(0.0, 0.33, 0.67)));
  vec4 bg = texture2D(Texture, uv);
  gl_FragColor = mix(bg, vec4(col, 1.0), ring);
}";

const GLSL_KEYWORDS: &[&str] = &[
  "void", "float", "int", "bool", "vec2", "vec3", "vec4",
  "mat2", "mat3", "mat4", "sampler2D", "samplerCube",
  "if", "else", "for", "while", "do", "return", "break", "continue",
  "in", "out", "inout", "uniform", "varying", "attribute",
  "precision", "highp", "mediump", "lowp",
  "const", "struct",
];

const GLSL_BUILTINS: &[&str] = &[
  "texture2D", "textureCube", "mix", "clamp", "smoothstep", "step",
  "length", "distance", "dot", "cross", "normalize", "reflect",
  "refract", "sin", "cos", "tan", "asin", "acos", "atan",
  "pow", "exp", "log", "sqrt", "abs", "sign", "floor", "ceil",
  "fract", "mod", "min", "max", "radians", "degrees",
  "gl_FragColor", "gl_FragCoord",
];

fn highlight_glsl(plain: &str) -> String {
  let mut result = String::with_capacity(plain.len() * 2);

  for line in plain.split('\n') {
    if !result.is_empty() {
      result.push('\n');
    }

    let trimmed = line.trim_start();

    if trimmed.starts_with('#') {
      result.push_str("{color=#e06c75|");
      result.push_str(&styling::escape_str(line));
      result.push('}');
      continue;
    }

    if let Some(comment_pos) = line.find("//") {
      let (before, comment) = line.split_at(comment_pos);
      highlight_tokens(before, &mut result);
      result.push_str("{color=#7d8799|");
      result.push_str(&styling::escape_str(comment));
      result.push('}');
      continue;
    }

    highlight_tokens(line, &mut result);
  }

  result
}

fn highlight_tokens(line: &str, out: &mut String) {
  let mut chars = line.char_indices().peekable();

  while let Some(&(i, ch)) = chars.peek() {
    // Numbers (including floats like 0.5, .5, 1e3)
    if ch.is_ascii_digit() || (
      ch == '.' &&
      chars.clone().nth(1).map_or(false, |(_, c)| c.is_ascii_digit())
    ) {
      let start = i;
      while chars.peek().map_or(false, |&(_, c)|
        c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E'
      ) { chars.next(); }
      let end = chars.peek().map_or(line.len(), |&(idx, _)| idx);
      let num = &line[start..end]; // not inclusive
      out.push_str("{color=#fab387|");
      out.push_str(&styling::escape_str(num));
      out.push('}');
      continue;
    }

    // Identifiers / Keywords
    if ch.is_ascii_alphabetic() || ch == '_' {
      let start = i;
      while chars.peek().map_or(false, |&(_, c)|
        c.is_ascii_alphanumeric() || c == '_'
      ) { chars.next(); }
      let end = chars.peek().map_or(line.len(), |&(idx, _)| idx);
      let word = &line[start..end]; // not inclusive
      let escaped = styling::escape_str(word);

      if GLSL_KEYWORDS.contains(&word) {
        out.push_str("{color=#e06c75|");
        out.push_str(&escaped);
        out.push('}');
      } else if GLSL_BUILTINS.contains(&word) {
        out.push_str("{color=#89b4fa|");
        out.push_str(&escaped);
        out.push('}');
      } else {
        out.push_str(&escaped);
      }
      continue;
    }

    // Everything else: operators, brackets, whitespace
    let escaped = styling::escape_str(&ch.to_string());
    if "=+-*/<>!&|%^~?:".contains(ch) {
      out.push_str("{color=#94e2d5|");
      out.push_str(&escaped);
      out.push('}');
    } else if "()[]{}.,;".contains(ch) {
      out.push_str("{color=#9399b2|");
      out.push_str(&escaped);
      out.push('}');
    } else {
      out.push_str(&escaped);
    }
    chars.next();
  }
}

pub fn run(ui: &mut Ui<'_, ()>) {
  static CODE_FONT: FontAsset = FontAsset::Bytes {
    file_name: "geistmono.ttf",
    data: include_bytes!("../../assets/fonts/geistmono.ttf"),
  };

  // Seed the editor with default GLSL exactly once
  if !SEEDED.swap(true, Ordering::Relaxed) {
    let highlighted = highlight_glsl(DEFAULT_GLSL);
    ui.set_text_value("glsl_editor", &highlighted);
  }

  let raw = ui.get_text_value("glsl_editor").to_string();
  let plain = styling::strip_styling(&raw);
  let highlighted = highlight_glsl(&plain);

  if raw != highlighted {
    let cursor = ui.get_cursor_pos("glsl_editor");
    let content_pos = styling::cursor_to_content(&raw, cursor);
    ui.set_text_value("glsl_editor", &highlighted);
    let new_cursor = styling::content_to_cursor(&highlighted, content_pos, false);
    ui.set_cursor_pos("glsl_editor", new_cursor);
  }
  set_shader_source("playground", &plain);

  ui.element()
    .width(grow!())
    .height(grow!())
    .background_color(0x1E1B1B)
    .layout(|l| l.direction(LeftToRight))
    .children(|ui| {
      // Editor panel (padding wrapper around the text input)
      ui.element()
        .width(percent!(0.45))
        .height(grow!())
        .background_color(0x1A1111)
        .layout(|l| l.padding(12))
        .children(|ui| {
          // Text input for GLSL code
          ui.element()
            .id("glsl_editor")
            .width(grow!())
            .height(grow!())
            .text_input(|t| t
              .multiline()
              .font(&CODE_FONT)
              .font_size(13)
              .line_height(18)
              .text_color(0xE8E0DC)
              .cursor_color(0xFF654D)
              .selection_color((255u8, 101u8, 77u8, 51u8))
              .no_styles_movement()
            )
            .empty();
        });

      // Preview panel
      ui.element()
        .width(grow!())
        .height(grow!())
        .background_color(0x111111)
        .effect(&PLAYGROUND_SHADER, |s| {
          s.uniform("u_time", get_time() as f32);
        })
        .empty();
    });
}
