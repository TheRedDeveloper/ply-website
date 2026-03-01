// ply_example.js — miniquad plugin for doc embedded examples.
// Reads the example ID from the URL query string and params from the URL hash.
// Both are pushed to WASM each frame via FFI.

window.example_id = window.example_id || "";
window.example_params = window.example_params || "";

miniquad_add_plugin({
    register_plugin: function (imp) {
        imp.env.ply_example_get_id = function (ptr, max_len) {
            var bytes = new TextEncoder().encode(window.example_id);
            var len = Math.min(bytes.length, max_len);
            new Uint8Array(wasm_memory.buffer, ptr, len).set(
                bytes.subarray(0, len)
            );
            return len;
        };
        imp.env.ply_example_get_params = function (ptr, max_len) {
            var bytes = new TextEncoder().encode(window.example_params);
            var len = Math.min(bytes.length, max_len);
            new Uint8Array(wasm_memory.buffer, ptr, len).set(
                bytes.subarray(0, len)
            );
            return len;
        };
        imp.env.ply_example_log = function (ptr, len) {
            var bytes = new Uint8Array(wasm_memory.buffer, ptr, len);
            var msg = new TextDecoder().decode(bytes);
            var panel = document.getElementById('console-panel');
            if (panel) {
                panel.style.display = 'block';
                panel.textContent += msg + '\n';
                panel.scrollTop = panel.scrollHeight;
            }
        };
    },
    on_init: function () {},
    version: 1,
    name: "ply_example"
});
