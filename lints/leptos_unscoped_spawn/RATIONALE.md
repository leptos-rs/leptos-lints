# `leptos_unscoped_spawn`'s rationale

## The scope of contexts in Leptos

Leptos uses a model for scoping contexts known as the *reactive graph*. This graph
is not the same as the component tree. It is composed of a tree of reactive nodes,
each of which has a parent node and zero or more child nodes. These include
components, signals, effects, memos, resources, callbacks, and so on.

Contexts are scoped to the reactive graph. This means that a context set in a node
is available to its child nodes, but not to its parent or sibling nodes.

## A bit of history

Traditionally, Leptos provided limited access to contexts. For example, event listeners
and `Action`s were not considered part of the reactive graph until Leptos v0.8.9, so
contexts were not available there. Since Leptos v0.8.9, contexts are accessible in
event listeners and `Action`s as well.

## Expecting a context that does not exist

When a context is expected to be available but is not, the application will panic at
runtime. This is very inconvenient, since the panic will probably happen in production,
and it is not always easy to debug what is happening.

The absence of a context can be difficult to detect during development, as the developer’s
mental model may not match the actual scope of contexts in Leptos.

### Example

```rust
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    provide_context("Hi!");

    let my_fn = async move || {
        let ctx = expect_context::<&str>();  // Panics at runtime: no context found
    };

    view! {
        <button on:click=move |_| leptos::task::spawn_local(my_fn())>"Click me"</button>
    }
}
```

## The solution: scoped spawns

To spawn an asynchronous function with the parent scope included, Leptos provides
the function `leptos::task::spawn_local_scoped`. This creates a new reactive node
as a child of the current node, so contexts will be available inside.

### Example

```rust
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    provide_context("Hi!");

    let my_fn = async move || {
        let ctx = expect_context::<&str>();  // The context is found correctly
        assert_eq!(ctx, "Hi!");
    };

    view! {
        <button on:click=move |_| leptos::task::spawn_local_scoped(my_fn())>"Click me"</button>
    }
}
```

### The downside

Scoped spawns have a small overhead compared to unscoped spawns, which impacts performance
and memory usage. If you are sure that you do not need the parent scope, you can use
`leptos::task::spawn_local` instead and allow the `leptos_unscoped_spawn` lint to be ignored in
that specific case.

```rust
#[allow(leptos_unscoped_spawn)]
leptos::task::spawn_local(async {
    // ...
});
```
