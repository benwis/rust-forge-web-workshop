# Exercise 4 - Server Functions

One of the hardest parts of writing a full-stack framework is how to handle
running code on the server and using it in the browser. Gone are the simple days
 where either everything was done on the server(PHP?) or on the client(React). 
(Un)Fortunately we're not writing one today, nor are we doing it the traditional
way of writing business logic on the server, setting up a REST or GraphQL API, 
and then writing fetching code on the frontend. Server functions abstract that 
all away while still providing the flexibility we need.

In Leptos, a server function is just a Rust function, annotated with the 
`#[server]` macro, that takes in inputs from the client and returns a `Result`.
```rust
// Easy right
#[server]
pub async fn add(num1: u32, num2: u32) -> Result<u32, ServerFnError>{
    num1+num2
}
```
By editing the parameters passed to the server macro, we can change how the code
generation is handled. We can change it's name, endpoint, input encoding, output
 encoding, and [more](https://docs.rs/leptos/latest/leptos/attr.server.html). 
```rust
// Look ma, cursed API
#[server(input = Rkyv, output = Toml)]
pub async fn add(num1: u32, num2: u32) -> Result<u32, ServerFnError>{
    num1+num2
}
```

In our components we can call the server fn as a regular function. Here's one 
way to do it(not the best way, but it's good to simplify for this).
```rust
#[component]
pub fn AddButton() -> impl IntoView {
    view! {
        <button on:click=move |_| {
            spawn_local(async {
                add(2, 2).await;
            });
        }>
            "Add 2+2"
        </button>
    }
}
```

Server functions are run as regular rust functions when a user navigates to a 
page, and then on subsequent navigations they're called as a separate request 
during client side navigation. Server functions create *public* API endpoints, 
which is great for creating an API and terrible if you forget and expose 
sensitive data without additional checks.

## Exercise
Since we're building a blog, let's create a server function called `get_posts()`
 that returns a `Vec` of post content. For now, feel free to use static 
hardcoded content, we'll change that later. In the same file, create a component
 called `PostList`that'll display the posts.





