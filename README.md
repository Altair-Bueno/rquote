# Rquote

Rquote is a web application built using Rust and WebAssembly. It fetches Anime
quotes from the [Animechan API](https://animechan.vercel.app/). You can check it
out at <https://rquote.vercel.app/>

## How to build

### Pre-requisites

- [rust's toolkit](https://www.rust-lang.org/learn/get-started)
- [trunk](https://trunkrs.dev)
- [wasm-bindgen-cli](https://rustwasm.github.io/wasm-bindgen/reference/cli.html)

### Build website

1. Install [python3](https://www.python.org/downloads/) or later
2. Install [chevron](https://github.com/noahmorrison/chevron)
3. Run `python build.py` on the root directory to compile the `index.html`
   template

Now you can use `Trunk` to build `Rquote`

```bash
# To open a local development server
trunk serve --open

# Release build (destination `dist/` folder)
trunk build --release
```

> Debug builds are faster to compile, but they are significantly slower.

## Technologies

- [Yew](https://yew.rs)
- [Rust](https://www.rust-lang.org)
- [Boostrap v5](https://getbootstrap.com)

## More info

- [Source repository](https://github.com/Altair-Bueno/rquote)

