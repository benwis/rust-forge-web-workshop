# Exercise 3 - Hot Reloading and Styling

Before we get too much further, I thought it'd be a good idea to talk about
styling and how it affects hot reloading.

Hot reloading in Leptos tries to update the view in the browser in real time,
without waiting for rust to recompile first. This allows changes simple HTML/CSS
changes to appear in real time when you save the file. Try it out by adding the
`--hot-reloading` switch to `cargo-leptos`.
```bash
cargo leptos watch --hot-reload
```

Now, if you change the `h1` header on our About page and save, it should update
in your browser immediately.
```rust
view!{
    <h1>Archive</h1>
    //changed to
    <h1>My Archive</h1>
    // should be nearly instant
}
```
This should also work for adding/removing or editing pure HTML, CSS classes, 
style tags, and more!

## Tailwind and SCSS
Both Tailwind and SCSS have a compile step built into them. Tailwind scans your 
code and adds only necessary classes to your CSS. SCSS compiles itself to CSS.
Both of these can introduce delay into this loop, which may or may not be worth
it to you.

# Exercise
If you feel like it, setup tailwind or switch to regular CSS by editing the 
options in your root `Cargo.toml`. `cargo-leptos` uses the 
`[workspace.metadata.leptos]` to store options. The options for it can be found
in the README for `cargo-leptos` 
[here](https://github.com/leptos-rs/cargo-leptos#parameters-reference).

