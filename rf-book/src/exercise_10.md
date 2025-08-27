# Exercise 10 - Client Side Interactivity

Overall, client side interactivity is the feature that makes a web framework like
 Leptos worthwhile over a server and a templating language, or a simple static 
site generator. I'd be remiss to do a workshop without talking about Signals.

Signals are the fundamental building block of the reactive system, which allows 
us to update specific bits of our UI in response to data changes. 

A Signal holds a value that may be updated, and that we may want to notify 
others that it was updated.
```rust
// Returns a ReadSignal, which lets you get the value, and a WriteSignal, which
// let's you set the value
let (count, set_count) = signal(0);
#[component]
pub fn CountComponent()-> impl IntoView{
    view!{
        <p>"The count is " {move || count.get()}</p>
        <button on:click={move||set_count.set(count.get()+1)}
            "Add 1"
        </button>
    }
}
```
When the button is pressed in the above example, only the closure that calls 
`count.get()` is rerendered, not the whole component. This is fine grained 
interactivity.
## Event Handlers
The eagle eye abouve you may have noticed the `on:click` attribute on the button
 in the above example. This is an example of Leptos can register event handlers 
on different parts of the UI. You should be able to use any of the common events
 in this way to run rust code on your html.

## Effects
The second extremely useful part of the reactive system is an Effect. An Effect 
is a wrapper around a bit of code that depends on a Signal. 
```rust
let (a, set_a) = signal(0);
let (b, set_b) = signal(0);

Effect::new(move |_| {
  // immediately prints "Value: 0" and subscribes to `a`
  logging::log!("Value: {}", a.get());
});
```

Effects automatically subscribe to changes in any signal whose value is 
accessed inside them, like the `a.get()` n the above example.

In fact, there are a lot of "hidden" signals that you've already seem. For 
example, the p tag in the above example
```rust
<p>"The count is " {count.get()}</p>
```
is actually automatically wrapped in an effect by the renderer to allow it to run.

## Memos
A Memo is the most complex of the reactive elements to implement, but is very 
similar in behavior to an Effect. Unlike an Effect, which runs anytime the 
subscribed signals is touched, a Memo only runs when one of the inputs changes.
This has a minimal additional runtime cost but is worth it for expensive 
calculations.


## Exercise
Add a +1 Button to the post page with an on:submit handler that increases the 
total +1s. Store those in the DB. Display the number of +1s per post on the 
index and post pages.

