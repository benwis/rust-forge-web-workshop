# Exercise 8 - Axum Integration

I think it's worth having a brief discussion about how Axum and Leptos fit 
together. I definitely get the impression that people think there's a lot more 
magic in the connection, but it's actually fairly simple. 
## What is Leptos in Regards to Axum?
Leptos lives as a set of routes and a few Axum handlers. We've created a trait 
called `LeptosRoutes` that scans the code and automatically adds regular 
Axum `.route()` calls to the Axum router. 

These routes call a couple handler functions in the leptos_axum crate called 
[`handle_server_fns`](https://docs.rs/leptos_axum/latest/leptos_axum/fn.handle_server_fns.html)
and [`render_route_with_context`](https://docs.rs/leptos_axum/latest/leptos_axum/fn.render_route_with_context.html).
```rust
#[tokio::main]
async fn main() {

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
```
This means that Leptos can be quite easily added to an existing Axum app to 
serve the frontend. The same general principal applies to Actix as well.

## Exercise
Add a health check route `/health_check` to Axum.

