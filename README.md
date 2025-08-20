# rust-forge-web-workshop

## Prerequisites
The only thing you'll need for the workshop is a computer that can comfortably run and compile Rust projects. If it gets through the software installs below, it'll *probably* be fine.
- Mac, Windows, or Linux Laptop
- Your Blog Ideas(optional)
### Install Software
### 1. Rust
You'll want to install Rust(if you haven't already) through `rustup`. You can find the link to the instructions [here](https://www.rust-lang.org/tools/install).

Mac Users: Please don't use Homebrew to install rust. It messes with our target switching and causes bugs. If you are using an arm based mac, ou may also want to run `rustcc -Vv` in a terminal to make sure it isn't trying to run rust in "rosetta". 

We'll be using stable Rust in this workshop, but we do support Nightly, which offers a few small ergonomic benefits. I'll leave installation of that to the reader as a do it at your own risk thing, but there isn't a huge difference.

1. Add wasm32-unknown-unknown target
```bash
rustup target add wasm32-unknown-unknown
rustup component add rust-analyzer #optional, I like to manage RA through rustup
```
### 2. Cargo Extensions
```bash
cargo install cargo-leptos --locked # compiles leptos SSR projects and provides some DX features
cargo install cargo-generate #optional, but let's you use templates
```

If you're on Windows, installing `cargo-leptos` from scratch may error out and tell you you need to download `perl`. If you don't want to do that, then I'd recommend installing `cargo-binstall` and installing one of our binary builds
```bash
cargo install cargo-binstall
cargo binstall cargo-leptos
```
### 3. IDE Setup
It's nice to setup our editor with a few DX improvements to make using Leptos easier. [This page](https://book.leptos.dev/getting_started/leptos_dx.html) in our book covers the general process, however for the best experience I recommend either Neovim or Zed. VsCode, RustRover or all work fine, but you may not get as nice syntax highlighting or autocompletions.

- **Code Formatter**: leptosfmt can automatically format code inside of our view! macros, where we combine HTML with Rust code in a very similar way to JSX. This can then be piped into rustfmt to highlight all Rust code. Their [README](https://github.com/bram209/leptosfmt) has setup instructions. I personally use Option 2: Editor specific config, but feel free to install it as you see fit.
```bash
cargo install leptosfmt
```
* **Enable Features**: Leptos uses different features to gate code that compiles for different targets(server v browser)/(native v wasm), which can cause rust-analyzer to ignore currently non-enabled features. Check the book link [here](https://book.leptos.dev/getting_started/leptos_dx.html#3-enable-features-in-rust-analyzer-for-your-editor-optional)  on how to enable those.
## Editor Specific Config
### Neovim
For neovim/lazyvim/astrovim/etc. we have both a treesitter grammar plugin and a plugin to autoclose brackets provided by rayliwell, which can be found [here](https://github.com/rayliwell/tree-sitter-rstml) and [here](https://github.com/rayliwell/nvim-ts-autotag)

I wrote a blog post on how to setup Lazyvim with these plugins [here](https://benw.is/posts/easy-leptos-editor), the process should be similar for other neovim distros. 
### Zed
Zed is a newcomer to this trio, but they now have a plugin that should allow both syntax highlighting, type completion, and autoclosing brackets. You can find that [here](https://zed.dev/extensions/leptos)
### RustRover
There isn't too much to modify here, unforunately there's been little to no traction on getting support for the Leptos DSL in RustRover. You can see some of the open issues [here](https://youtrack.jetbrains.com/issue/RUST-17560/Feature-request-leptosfmt-formatter-for-the-leptos-view-macro) and [here](https://youtrack.jetbrains.com/issue/RUST-11857/RustRover-Failing-to-Find-Methods-Autocompletions-in-Leptos-RSX-fileRust-with-HTML-inside-macros). Feel free to comment on those to bring them to their attention
### VsCode
VsCode works fairly well, however I know of no movement to add plugins/grammars as there was for Neovim and Zed.

### Conclusions
Feel free to reach out with any questions, suggestions, or comments on the #rust-web channel in the Rust Forge Discord, the Leptos [discord](https://discord.gg/x8NhWWYTV2) or personally at ben@benw.is

Wishing you all safe travels!
-Ben
