# Yew + DaisyUI + Tailwind 4 + Vite (Single HTML file)

This is a minimal example of a Yew app compiled to WebAssembly and bundled
with Vite, using Tailwind CSS 4 and DaisyUI 5 **without any CDNs**.

The build uses `vite-plugin-singlefile` so the production output is a single
`frontend/dist/index.html` file with JS, CSS, and WASM all inlined.

## Structure

- `my-yew-app/` — Rust crate (Yew app built as `cdylib` via `wasm-pack`)
- `frontend/`   — Vite + Tailwind 4 + DaisyUI 5 + singlefile plugin

## Commands

```bash
# one-time setup
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

cd frontend
npm install

# build wasm
npm run wasm

# dev server
npm run dev

# production single-file build
npm run wasm
npm run build
```

After `npm run build`, open `frontend/dist/index.html` in a browser.
