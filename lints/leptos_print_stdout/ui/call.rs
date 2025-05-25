// Is not possible to use third party dependencies in dylint tests,
// calling them without starting by a root path (`::`), like using
// `leptos::logging::log!` instead of `::leptos::logging::log!`.
// So we mock the `leptos::logging::log!` macro to test the lint.
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        println!($($arg)*);
    }
}

mod leptos {
    pub(crate) mod logging {
        pub(crate) use super::super::log;
    }
}

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
