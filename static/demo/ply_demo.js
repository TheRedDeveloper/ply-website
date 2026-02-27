// Global: both the WASM plugin and CodeMirror need access.
// Don't overwrite if index.html already set DEFAULT_CODE.
if (window.demo_code === undefined) window.demo_code = "";

miniquad_add_plugin({
    register_plugin: function (imp) {
        imp.env.ply_demo_get_code = function (ptr, max_len) {
            var bytes = new TextEncoder().encode(window.demo_code);
            var len = Math.min(bytes.length, max_len);
            new Uint8Array(wasm_memory.buffer, ptr, len).set(
                bytes.subarray(0, len)
            );
            return len;
        };
    },
    on_init: function () {},
    version: 1,
    name: "ply_demo"
});

window.ply_demo_set_code = function (code) {
    window.demo_code = code;
};
