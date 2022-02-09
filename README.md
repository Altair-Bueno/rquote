# Rquote

Rquote is a SPA using Rust and WebAssembly. It fetches Anime quotes from the
[Animechan API](https://animechan.vercel.app/). You can check it out at
<https://rquote.vercel.app/>

## How to build

### Pre-requisites

- [rust's toolkit](https://www.rust-lang.org/learn/get-started)
- [trunk](https://trunkrs.dev)
- [wasm-bindgen-cli](https://rustwasm.github.io/wasm-bindgen/reference/cli.html)

### Build website

```bash
# To open a local development server
trunk serve --open

# Release build (destination `dist/` folder)
trunk build --release
```

## Technologies

- [Yew](https://yew.rs)
- [Rust](https://www.rust-lang.org)
- [Boostrap v5](https://www.rust-lang.org)

