use leptos::prelude;
use leptos::prelude::event_target;

fn main() {
    event_target::<u32>();

    let _a: Option<&str> = prelude::event_target();

    leptos::prelude::event_target::<String>();

    let _b: Option<Option<u8>> = ::leptos::prelude::event_target();
}
