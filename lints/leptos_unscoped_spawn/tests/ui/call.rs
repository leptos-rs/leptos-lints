use leptos::task;
use leptos::task::spawn_local;

fn main() {
    spawn_local(async {
        // ...
    });

    task::spawn_local(async {
        // ...
    });

    leptos::task::spawn_local(async {
        // ...
    });
}
