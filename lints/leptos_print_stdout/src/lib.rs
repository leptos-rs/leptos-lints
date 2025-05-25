#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;

use clippy_utils::diagnostics::span_lint_and_help;
use rustc_lint::{EarlyContext, EarlyLintPass};

dylint_linting::declare_pre_expansion_lint! {
    /// ### What it does
    ///
    /// Check for calls to `leptos::logging::log!`. The purpose of this lint is
    /// to catch debugging remnants. Analog to `clippy::print_stdout` for Leptos.
    ///
    /// ### Why is this bad?
    ///
    /// People often print on stdout while debugging an application and might
    /// forget to remove those prints afterward.
    ///
    /// ### Known problems
    ///
    /// Only catches `leptos::logging::log!` calls.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// leptos::logging::log!("This is a log message");
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// #[allow(unknown_lints)]
    /// #[allow(leptos_print_stdout)]
    /// {
    ///     leptos::logging::log!("This is a log message");
    /// }
    /// ```
    pub LEPTOS_PRINT_STDOUT,
    Warn,
    "Check for Leptos logging usage."
}

impl EarlyLintPass for LeptosPrintStdout {
    fn check_mac(&mut self, cx: &EarlyContext, macro_call: &rustc_ast::MacCall) {
        let segments = &macro_call.path.segments;
        let is_leptos_logging_log_call = (
            // leptos::logging::log!("...")
            segments.len() == 3
                && segments[0].ident.name.as_str() == "leptos"
                && segments[1].ident.name.as_str() == "logging"
                && segments[2].ident.name.as_str() == "log"
        ) || (
            // ::leptos::logging::log!("...")
            segments.len() == 4
                && segments[0].ident.name.as_str() == "{{root}}"
                && segments[1].ident.name.as_str() == "leptos"
                && segments[2].ident.name.as_str() == "logging"
                && segments[3].ident.name.as_str() == "log"
        );
        if is_leptos_logging_log_call {
            span_lint_and_help(
                cx,
                LEPTOS_PRINT_STDOUT,
                macro_call.span(),
                "use of `leptos::logging::log!`",
                None,
                "for further information visit https://github.com/leptos-rs/leptos-lints/tree/main/lints/leptos_print_stdout#readme",
            );
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ui() {
        dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
    }
}
