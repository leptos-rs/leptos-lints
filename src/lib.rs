#![feature(rustc_private)]
#![warn(unused_extern_crates)]

dylint_linting::dylint_library!();

extern crate rustc_lint;
extern crate rustc_session;

#[expect(clippy::no_mangle_with_rust_abi)]
#[unsafe(no_mangle)]
pub fn register_lints(sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    leptos_print_stdout::register_lints(sess, lint_store);
    leptos_unscoped_spawn::register_lints(sess, lint_store);
    leptos_event_target::register_lints(sess, lint_store);
}
