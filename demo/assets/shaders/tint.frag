#version 100
precision highp float;

varying lowp vec2 uv;

uniform sampler2D Texture;
uniform float u_time;
uniform vec4 u_tint;

void main() {
    vec4 col = texture2D(Texture, uv);
    // Pulse the tint strength over time
    float strength = u_tint.a * (0.5 + 0.5 * sin(u_time * 3.0));
    gl_FragColor = mix(col, vec4(u_tint.rgb, col.a), strength);
}
