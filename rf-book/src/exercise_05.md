# Exercise 5 - Server Data Fetching
## Resources

While there's nothing wrong with `spawn_local`, leptos provides a fair number 
of components to streamline async data fetching and provide additional 
functionality. 

`Resource` is a reactive wrapper for async tasks, which allows you to run async
functions and handle futures. Being able to await a function in your app turns 
out to have a lot of uses. Let's take a look at one:
```rust
// this count is our synchronous, local state
let (count, set_count) = signal(0);

// our resource
let async_data = Resource::new(
    move || count.get(),
    // every time `count` changes, this will run
    |count| load_data(count) 
);
```
The first argument is the set of Signals that will cause the Resource to rerun, 
the second is the function to run.

To handle some of the additions in `Resource`, we usually recommend using 
`Suspense`. It lets you handle waiting for multiple resources to complete, 
render a fallback while it's loading, and together with Resource also provides
some significant performance improvements.

```rust
let (count, set_count) = signal(0);
let (count2, set_count2) = signal(0);
let a = Resource::new(count, |count| async move { load_a(count).await });
let b = Resource::new(count2, |count| async move { load_b(count).await });

view! {
    <h1>"My Data"</h1>
    <Suspense
        fallback=move || view! { <p>"Loading..."</p> }
    >
        <h2>"My Data"</h2>
        <h3>"A"</h3>
        {move || {
            a.get()
                .map(|a| view! { <ShowA a/> })
        }}
        <h3>"B"</h3>
        {move || {
            b.get()
                .map(|b| view! { <ShowB b/> })
        }}
    </Suspense>
}
```

This example from the book shows the basic lifecycle of a Resource, which wraps
your `Result` type in an `Option` to represent the network lifecycle. You can 
imagine that handling the nested types can be a bit painful, so we've created
`Suspend` to streamline that somehwat.
```rust
view! {
    <h1>"My Data"</h1>
    <Suspense
        fallback=move || view! { <p>"Loading..."</p> }
    >
        <h2>"My Data"</h2>
        {move || Suspend::new(async move {
            let a = a.await;
            let b = b.await;
            view! {
                <h3>"A"</h3>
                <ShowA a/>
                <h3>"B"</h3>
                <ShowB b/>
            }
        })}
    </Suspense>
}
```
And even simpler than that is `Await`, which simply waits for a `Future` to 
resolve before rendering.
```rust
async fn fetch_monkeys(monkey: i32) -> i32 {
    // maybe this didn't need to be async
    monkey * 2
}
view! {
    <Await
        // `future` provides the `Future` to be resolved
        future=fetch_monkeys(3)
        // the data is bound to whatever variable name you provide
        let:data
    >
        // you receive the data by reference and can use it in your view here
        <p>{*data} " little monkeys, jumping on the bed."</p>
    </Await>
}
```

## Exercise
Modify your `PostList` component to use a `Resource` instead of a `spawn_local` 
and using the async component primitive of your choice.


