# Exercise 6 - We Need a Data Store

Up until this point we've been returning static data from your server function.
Obviously with a blog we'll want to have a more elegant way to store and handle
posts. 

While it doesn't need to be a database, I'll use one here as an example to show 
what some of that setup looks like.

## Optional Dependencies
Leptos compiles a webassembly module for the browser to provide interactivity 
and routing for your app. Unfortunately that means all the dependencies used in 
components need to compile for webassembly.

Database crates like `sqlx`  often fail to do this, so we need to exclude them 
from this build. We'll mark these as `optional` in our `Cargo.toml` and only 
include them for the `ssr` feature. An example of this is below:
```toml
axum = { version = "0.8.1", optional = true }
[features]
default = ["csr"]
csr = ["leptos/csr"]
hydrate = ["leptos/hydrate"]
ssr = [
  "dep:axum",
  "dep:tower",
  "dep:http",
  "leptos/ssr",
  "leptos_axum",
  "leptos_meta/ssr",
  "leptos_router/ssr",
]
```

We only need to worry about this for the `app` crate, the `server` crate is only
 compiled during the server build and escapes this for workspaces.

## Add Sqlite DB
We're going to use `sqlite` as our DB, so we'll need to create the database file
 first. To do that you'll need the `sqlite3` cli tool. Feel free to install that
 in a way you seem fit. You'll want to put it in the public folder so it's 
included with your final app. We'll also want to install `sqlx-cli` with cargo 
install.

```bash
export DATABASE_URL="sqlite:public/rustforge.db"
sqlx db create
```

Next we'll choose a rust crate to access your DB, I usually use `sqlx`, although
 `diesel`, `seaorm` or `rusqlite` are all good options.
```toml
# Cargo.toml in app
sqlx = { version = "0.8", features = [ "runtime-tokio", "tls-native-tls", 
    "sqlite"], optional="true" }
```
We'll need to create a database pool in our `main.rs` in our server crate.
```rust
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
```
and then add it to Axum's State so that we can access it in server functions.


## Exercise
Add server functions `get_post()` and `add_post()`. Setup a posts table and a 
basic post type in your DB.
