+++
title = "Networking"
weight = 14
+++

Ply has built-in HTTP and WebSocket support. Enable it with the `net` feature:

```toml
ply-engine = { ..., features = ["net"] }
```

Everything lives in the `net` module, available through the prelude. It works on all platforms. It is optimized to work across the application loop without you needing to store or manage anything, unlike typical networking libraries.

## HTTP requests

Fire a request with `net::get`, `net::post`, `net::put`, or `net::delete`.
Each takes a string ID, a URL, and a configuration closure:

```rust
// GET request
net::get("users", "https://api.example.com/users", |r| r
    .header("Authorization", "Bearer token123")
);

// POST with body
net::post("submit", "https://api.example.com/submit", |r| r
    .header("Content-Type", "application/json")
    .body("{\"name\":\"ply\"}")
);
```

Requests are idempotent: calling `net::get("users", ...)` again while a request with ID `"users"` already exists does nothing.

### Checking the response

Use `net::request()` to get a handle and poll for the response:

```rust
if let Some(req) = net::request("users") {
    match req.response() {
        None => {
            // Show a loading indicator
        }
        Some(Ok(resp)) => {
            let status = resp.status();   // u16
            let body = resp.text();       // &str
            let raw = resp.bytes();       // &[u8]
        }
        Some(Err(e)) => {
            // Request failed (network error, DNS, etc.)
        }
    }
}
```

`response()` returns `Arc<Response>`, calling it every frame costs nothing.

### Cancelling

```rust
if let Some(req) = net::request("users") {
    req.cancel();  // removes the request immediately and consumes the handle
}
```

### JSON deserialization

Enable the `net-json` feature to add `.json()`:

```toml
ply-engine = { ..., features = ["net-json"] }
```

```rust
#[derive(serde::Deserialize)]
struct User { name: String, email: String }

if let Some(req) = net::request("users") {
    if let Some(Ok(resp)) = req.response() {
        let users: Vec<User> = resp.json().unwrap();
    }
}
```

## WebSocket

### Connecting

```rust
net::ws_connect("chat", "wss://chat.example.com", |w| w
    .header("Authorization", "Bearer abc")
);
```

Like HTTP, `ws_connect` is idempotent.

For dev servers with self-signed certificates:

```rust
net::ws_connect("local", "wss://localhost:8443", |w| w.insecure());
```

`.insecure()` doesn't do anything on WASM because browsers don't allow it.

### Sending and receiving

```rust
if let Some(ws) = net::ws("chat") {
    ws.send_text("hello");
    ws.send(b"binary data");

    while let Some(msg) = ws.recv() {
        match msg {
            WsMessage::Connected => {
                // Connection established
            }
            WsMessage::Text(s) => {
                // Text frame
            }
            WsMessage::Binary(data) => {
                // Binary frame
            }
            WsMessage::Error(e) => {
                // Something went wrong
            }
            WsMessage::Closed => {
                // Server closed the connection
            }
        }
    }
}
```

### Closing

```rust
if let Some(ws) = net::ws("chat") {
    ws.close();  // graceful close, removes immediately and consumes the handle
}
```

## Why polling?

Awaiting a request would freeze the entire UI until it completes.
Internally, native builds use background threads (HTTP) and
a tokio runtime (WebSocket). WASM builds use the browser's
`XMLHttpRequest` and `WebSocket` APIs through the bridge.

Requests and WebSockets are tracked by the global `NET_MANAGER`, you don't need to manage anything. It cleans up your completed requests if you don't access them for 60 frames.

## Next steps

→ [Sound](/docs/sound/)
