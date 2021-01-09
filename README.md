# yew-app crash course code

This repo is the code for my yew crash course video [here](https://youtu.be/lmLiMozWNGA).

Some useful commands from the video:

```
cargo new --lib yew-app && cd yew-app

wasm-pack build --target web --out-name wasm --out-dir ./static

python -m http.server 8000 --directory static/
```
