# Exercise 7 - "Fancy" Routing

Because we're building a blog, we need to do a little bit of fancy routing. We 
can use path params or query params to store the post id or slug of the post
we want to display.

There are two ways you can do this:

1. named route params like id in /blog/post/:id
2. named route queries like q in /blog/post/?id=Foo

Accessing params and queries is pretty simple with a couple of hooks:

1. use_query or use_query_map
2. use_params or use_params_map

The first set are typed, meaning we can define what to expect. The second two 
put all found values into a map. I'm going to do the path params option
```rust
use leptos::Params;
use leptos_router::params::Params;

#[derive(Params, PartialEq)]
struct PostParams {
    id: Option<usize>,
}
```
Then inside my component, I can access the values.
```rust
use leptos_router::hooks::{use_params, use_query};

let params = use_params::<ContactParams>();
let query = use_query::<ContactSearch>();

// id: || -> usize
let id = move || {
    params
        .read()
        .as_ref()
        .ok()
        .and_then(|params| params.id)
        .unwrap_or_default()
};
```

## Exercise
Create a `/blog/:postid` route and read that value in our Post page. Modify the
`get_post` server function to take that id and find the post in the db.


