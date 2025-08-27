use leptos::prelude::*;

#[component]
pub fn Archive() -> impl IntoView {
    // rust variables defined in the function scope are
    // available inside the view macro
    let first_name = "Ben";
    let last_name = "Wishovich";
    view! {
        <h1>"Archive"</h1>
        <p>"My name is: "{first_name}" "{last_name}</p>
    }
}
