# Exercise 11 - Building a UI
Much like `spawn_local` and `Resource`, we have also added a primitive to make 
it easier to mutate data, the `Action`.

## Actions
The `Action`, much like the `Resource`, is designed to allow you to easily run 
an async function to mutate data. Unlike `Resource`, which runs on page load, 
an `Action` has to be dispatched, and features convenience methods to that 
effect. `ServerAction` further simplifies things for mutating server functions.

We can see an example of both in the below code.
```rust
async fn send_new_todo_to_api(task: String) -> usize {
    // do something...
    // return a task id
    42
}
let save_data = Action::new(|task: &String| {
  // `task` is given as `&String` because its value is available in `input`
  send_new_todo_to_api(task.clone())
});

// the argument currently running
let input = save_data.input();
// the most recent returned result
let result_of_call = save_data.value();
// whether the call is pending
let pending = save_data.pending();
// how many times the action has run
// useful for reactively updating something else in response to a `dispatch` and response
let version = save_data.version();

// before we do anything
assert_eq!(input.get(), None); // no argument yet
assert_eq!(pending.get(), false); // isn't pending a response
assert_eq!(result_of_call.get(), None); // there's no "last value"
assert_eq!(version.get(), 0);

// dispatch the action
save_data.dispatch("My todo".to_string());

// when we're making the call
assert_eq!(input.get(), Some("My todo".to_string()));
assert_eq!(pending.get(), true); // is pending
assert_eq!(result_of_call.get(), None); // has not yet gotten a response


// after call has resolved
assert_eq!(input.get(), None); // input clears out after resolved
assert_eq!(pending.get(), false); // no longer pending
assert_eq!(result_of_call.get(), Some(42));
assert_eq!(version.get(), 1);
```

## ActionForm
`ActionForm` is meant to empower you to use forms to provide progressive 
enhancement. If your app uses ActionForms, it should work without Javascript or 
Webassembly, depending a bit on the other choices you make in your app.
```rust

#[component]
fn SimpleComponent() -> impl IntoView {
    let submit = ServerAction::<Add>::new();

    view! {
      <ActionForm action=submit>
        <input type="number" name="num1"/>
        <input type="number" name="num2"/>
        <input type="submit"/>
      </ActionForm>
    }
}
```


## Exercise
Create a `/post/add` page and an `add_post()` server fn that allows you to add
a post to your blog!
