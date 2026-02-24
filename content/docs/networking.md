+++
title = "Networking"
weight = 15
+++

Ply apps can make HTTP requests and open WebSocket connections using `quad-net`.
We have a fork of `quad-net`, because the original crate is badly maintained.
<!-- TODO: Integrate quad-net into ply_engine and the prelude --> 

## HTTP requests

```rust
let request = Request::get("https://api.example.com/data");
```

Requests are non-blocking. Check for a response each frame:

```rust
if let Some(response) = request.try_recv() {
    match response {
        Ok(data) => println!("Response: {}", data),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```
<!-- TODO: Make them async!! -->

### POST requests

```rust
let request = Request::post(
    "https://api.example.com/submit",
    "{\"name\":\"ply\"}",
);
```

## WebSocket

```rust
let mut ws = WebSocket::connect("wss://echo.websocket.org").unwrap();
```

Send and receive:

```rust
// Send a message
ws.send_text("hello");

// Check for incoming messages each frame
while let Some(msg) = ws.try_recv() {
    println!("Received: {}", msg);
}
```
<!-- Make it async! -->

## WASM bundle

When building for web with `plyx web`, the JavaScript bundle
(`ply_bundle.js`) includes the quad-net bridge automatically. No extra
configuration needed.

## Chat example

```rust
let mut ws = WebSocket::connect("wss://chat.example.com").unwrap();
let mut messages: Vec<String> = Vec::new();

while let Some(msg) = ws.try_recv() {
    messages.push(msg);
}

ui.element()
    .width(grow!())
    .height(grow!())
    .overflow(|o| o.scroll_y())
    .layout(|l| l.direction(TopToBottom).gap(4).padding(8u16))
    .children(|ui| {
        for msg in &messages {
            ui.text(msg, |t| t.font_size(14).color(0xE8E0DC));
        }
    });
```

## Next steps

→ [Cross-Platform Builds](/docs/cross-platform/)
