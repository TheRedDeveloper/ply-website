// Cache WASM binaries and demo JS assets.
// WASM uses versioned filenames (app-v1.wasm, app-v2.wasm) for cache busting.
// JS files are cached by URL and updated when the SW is reinstalled.

var CACHE = 'ply-demo-v1';

self.addEventListener('install', function (event) {
    self.skipWaiting();
});

self.addEventListener('activate', function (event) {
    // Purge old cache versions
    event.waitUntil(
        caches.keys().then(function (names) {
            return Promise.all(
                names.filter(function (n) { return n !== CACHE; })
                     .map(function (n) { return caches.delete(n); })
            );
        }).then(function () { return self.clients.claim(); })
    );
});

self.addEventListener('fetch', function (event) {
    var url = event.request.url;

    // Only cache heavy demo assets: WASM and CodeMirror bundle
    var isDemoAsset = url.endsWith('.wasm') ||
        url.indexOf('codemirror-bundle.js') !== -1;

    if (!isDemoAsset) return;

    event.respondWith(
        caches.open(CACHE).then(function (cache) {
            return cache.match(event.request).then(function (cached) {
                if (cached) return cached;

                return fetch(event.request).then(function (response) {
                    cache.put(event.request, response.clone());

                    // Purge old WASM versions when a new one is cached
                    if (url.endsWith('.wasm')) {
                        cache.keys().then(function (keys) {
                            keys.forEach(function (key) {
                                if (key.url !== url && key.url.endsWith('.wasm')) {
                                    cache.delete(key);
                                }
                            });
                        });
                    }

                    return response;
                });
            });
        })
    );
});
