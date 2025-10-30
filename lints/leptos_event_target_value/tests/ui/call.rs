use leptos::prelude;
use leptos::prelude::event_target_value;

fn main() {
    event_target_value::<u32>();

    let _a: Option<&str> = prelude::event_target_value();

    leptos::prelude::event_target_value::<String>();

    let _b: Option<Option<u8>> = ::leptos::prelude::event_target_value();
}
