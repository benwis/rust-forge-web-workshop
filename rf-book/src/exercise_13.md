# Exercise 13 - Auth

If we've made it to here, then things have gotten much further than I expected.
Below are two auth examples from the Leptos repo, one that uses third party 
social logins like Github or Google, and one that does everything in-app and 
uses sessions.

I'll let y'all decide which we should add to the app.
[SSO](https://github.com/leptos-rs/leptos/tree/main/projects/sso_auth_axum)
[Session Based Auth](https://github.com/leptos-rs/leptos/tree/main/projects/sso_auth_axum)

## ProtectedRoute
ProtectedRoute is a neat little utility that'll let us protect a route, say 
`/private` or `/add_post`. Note though that on first load, Resources run as 
regular rust functions, so you'll need to place a method inside them to check 
for permissions.

[ProtectedRoute](https://docs.rs/leptos_router/latest/leptos_router/components/fn.ProtectedRoute.html)

## Exercise
Add auth to your app(or don't, I'm not the boss of you).
