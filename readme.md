# gnobbler
<sup>gnobble gnobble</sup>

<img width="768" height="424" alt="screenshot" src="https://github.com/user-attachments/assets/ed072e2b-f38c-4208-b60b-de4515e0808e" />

**you are the gnobbler. your job? gnobble.**

a retro platformer inspired by the original super mario bros games! if you dont know what a gnobbler is, thats fine, neither do i.

written in RUST (i love you rust).

made for hackclub's [siege](https://siege.hackclub.com/), for the **final week!!!**

## building

you'll need rust installed. to run, simply do `cargo run`.

to build for web, you need to compile for wasm and move that wasm file to the web dir. you'll also need to serve the file with a software of your choosing.

if you're serving with for instance `basic-http-server`, that would be:
```bash
cargo build --release --target wasm32-unknown-unknown && cp target/wasm32-unknown-unknown/release/gnobbler.wasm web/ && basic-http-server web/
```
