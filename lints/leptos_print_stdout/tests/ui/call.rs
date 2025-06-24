fn main() {
    leptos::logging::log!("foo");

    // This should not trigger the lint
    #[allow(unknown_lints)]
    #[expect(leptos_print_stdout)]
    {
        leptos::logging::log!("this should not trigger the lint");
    }

    ::leptos::logging::log!("bar");
}
