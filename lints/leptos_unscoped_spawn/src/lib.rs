#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;

use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::{HirId, Path};
use rustc_lint::{LateContext, LateLintPass};


dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Forbids the use of `leptos::task::spawn_local`, and recommends using
    /// `leptos::task::spawn_local_scoped` instead.  The purpose of this lint is
    /// to reduce the risk of missing context at run-time.
    ///
    /// ### Why is this bad?
    ///
    /// In Leptos, it's not always obvious when contexts are available. There are situations
    /// where they are not, like in unscoped spawns. These missing contexts can be difficult
    /// to detect and debug, and lead to run-time bugs.
    /// 
    /// For a in-depth explanation, please refer to [`leptos_unscoped_spawn`'s rationale].
    /// 
    /// [`leptos_unscoped_spawn`'s rationale]: https://github.com/leptos-rs/leptos-lints/blob/main/lints/leptos_unscoped_spawn/RATIONALE.md
    ///
    /// ### Example
    ///
    /// ```rust
    /// leptos::task::spawn_local(async {
    ///     // ...
    /// });
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// leptos::task::spawn_local_scoped(async {
    ///     // ...
    /// });
    /// ```
    ///
    /// A scoped spawn is less performant, so if you're sure that you're not using contexts
    /// in the spawn, allow the spawn to be unscoped.
    ///
    /// ```rust
    /// #[allow(leptos_unscoped_spawn)]
    /// leptos::task::spawn_local(async {
    ///     // ...
    /// });
    /// ```
    pub LEPTOS_UNSCOPED_SPAWN,
    Deny,
    "Forbids the use of `leptos::task::spawn_local`"
}

impl<'tcx> LateLintPass<'tcx> for LeptosUnscopedSpawn {
    fn check_path(&mut self, cx: &LateContext<'tcx>, path: &Path<'tcx>, _: HirId) {
        if let Some(segment) = path.segments.last() {
            let name = segment.ident.name.to_string();

            if name == "spawn_local" {
                span_lint_and_help(
                    cx,
                    LEPTOS_UNSCOPED_SPAWN,
                    segment.ident.span,
                    "use of `leptos::task::spawn_local`",
                    None,
                    "prefer `leptos::task::spawn_local_scoped`. For further information visit https://github.com/leptos-rs/leptos-lints/tree/main/lints/leptos_unscoped_spawn#readme",
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ui_examples() {
        dylint_testing::ui_test_examples(env!("CARGO_PKG_NAME"));
    }
}
