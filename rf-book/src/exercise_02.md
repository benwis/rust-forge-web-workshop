# Exercise 2 - Components

If you've used other frontend frameworks, the concept of "components" in leptos
should be familiar for you. A component is a useful way to define a reusable 
part of your web app. They can contain something as small as an svg tag or as
large as a page. Everything else is up to you!

## Components in Leptos
In Leptos, a component is a function that returns a type `View`, or much more
commonly, `impl IntoView`, meaning the framework can convert it into `View`. 
We also usually annotate our component function with a `#[component]` macro 
marker, which allows us some conveniences. You'll need it to use the `view!`
macro, which we'll be using in this tutorial, but if macros give you an ick 
feeling you can use the 
[builder syntax](https://book.leptos.dev/view/builder.html) and omit it.
```rust
// a simple component
#[component]
pub fn About()-> impl IntoView{
    // rust code goes here
    todo!();
}
```

## RSX and the view! macro
We've defined a component, but it still doesn't show anything. To do that, we'll
need to talk about the rsx/rstml DSL. RSX is a language designed to allow you
to combine html and rust code. The `view` macro wraps that DSL and let's us use 
it. Similar to jsx, it's basically regular HTML with Rust inserted between 
curly brackets`{}`.
```rust
#[component]
pub fn Archive() -> impl IntoView{
    view!{
        <h1>Archive</h1>
        <p> The first number is {1+2}</p>
    }
}
```
Easy right? It can get a bit more complicated looking, but that's the basics.

> You'll notice in the above example that the string Hello World doesn't have 
quotes. Quotes are no longer required for string content in almost all cases, 
but it will still work with double quotes aroound it. Which you choose is a
matter of personal preference.

Oh also, we do not require curly brackets around rust code in some places,
like below:
```rust
view!{
<h1 class=move || "foo".to_string()></h1>
}
```
Personally I prefer them though. We can configure leptosfmt to automatically
enforce that by adding a `leptosfmt.toml` to the root of your workspace and
adding `attr_value_brace_style="Always"`. Other options to configure can
be found on the leptosfmt [README](https://github.com/bram209/leptosfmt).

## Exercise
Expand your `Archive` function into a leptos component, and add a `view` macro
with some content. Play around with adding rust code in the `view` macro. 


