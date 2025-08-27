# Exercise 1 - Adding a New Route

## Directory Structure 
Let's get acquainted with the different crates in our template. Hopefully you 
should have a workspace that looks like this:
```bash
├── app
│   └── src
├── end2end
│   └── tests
├── frontend
│   └── src
├── public
├── server
│   └── src
└── style
```
### Crates
1. `app` - Contains the meat of your application, where your components and 
server functions live
2. `end2end` - A folder for end to end tests. We won't be using that today.
3. `frontend` - The frontend folder contains code only needed to run leptos's 
webassembly on the browser. Generally users do not need to edit this crate
4. `server` - The crate that contains the axum server, and code that'll be used
only on the server. Axum configuration happens here. 
5. `style` - Not a crate, but contains your CSS or SCSS. This can be configured
in cargo-leptos, should contain a `main.scss` file

## Start The Server
`cargo-leptos` tries to make working on SSR projects easy, and should be easy to
start in this case. Just run
```bash
cargo-leptos watch
```
Opening your browser to the listed IP and port, usually 'http://127.0.0.1:3000'
You should see the text "Welcome to Leptos", and a "Click Me" button with a 
counter on it. Clicking it should cause it to increment. If this doesn't happen,
 please let us know!

## Let's tweak it.
We're going to start this off simple, let's add a new page to the starter app.
Leptos has a router built in that's perfect for these cases. 
## Exercise
Add an About page to our site. It's fine if it's blank, we'll cover that in the
next one. Let's just get it to not return 404 on navigation
