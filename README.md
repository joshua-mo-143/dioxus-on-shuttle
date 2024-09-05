# Dioxus on Shuttle
This is an example of running Dioxus on Shuttle without being forced to compile to SPA and using CSR.

## How does it work?
- We use a separate binary, `build_wasm.rs` for frontend asset compilation. A new justfile command has been created (`just build`) to make the process a bit easier.
  - If you already use Dioxus, you may notice we delete the binary file. We don't need it and it also brings the archive size way over the limit.
- We also use a `build.rs` script to additionally move the assets to either the debug folder (next to the executable) or the sub-folder of the shuttle executable folder in deployment.

## How to use
- Make sure you have Dioxus CLI (the git version).
- Use `just build` if you have `just` installed to autobuild the assets, or follow the command chain manually.
- Use `cargo shuttle run` to run the application locally.
- Use `cargo shuttle deploy` to deploy the application to Shuttle!

## Limitations
- The `build.rs` file currently doesn't support moving assets to the release folder locally yet, but this is relatively easy to fix yourself.
