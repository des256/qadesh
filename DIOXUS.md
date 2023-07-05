# DUMP

- Dioxus supports web via wasm, like flutter web target

- Dioxus supports SSR? low-level control over rendering process

- Dioxus supports a way to run an app on a server and only render in the browser, like old X, some trouble rendering GL, etc.

- Dioxus desktop (Windows, Linux, MacOS) support via Tauri, get familiar with Tauri

- Dioxus supports mobile via WebView or blitz, poor support

- Dioxus supports some sort of terminal interface

- some hot reloading, using rsx! macro dependency

- rendering possible via virtualDOM-like construction, maybe even really inside HTML

- Dioxus generally renders to HTML/SVG tree, which is what web browsers are good at, this happens at the cost of native performance (even though SVG is implemented really fast in modern browsers, it is not nearly as performant as native/GPU)

- VirtualDom

- Dioxus currently not threadsafe

- rsx! interprets JSX DSL (JavaScript inside Rust??)

- 