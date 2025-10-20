#![feature(rustc_private)]

extern crate rustc_hir;

use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::{ExprKind, Expr, QPath};
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
    Warn,
    "Forbids the use of `leptos::task::spawn_local`"
}

impl<'tcx> LateLintPass<'tcx> for LeptosUnscopedSpawn {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        let ExprKind::Call(func, _) = expr.kind else {
            return;
        };

        let ExprKind::Path(ref qpath) = func.kind else {
            return;
        };

        let Some(def_id) = cx.qpath_res(qpath, expr.hir_id).opt_def_id() else {
            return;
        };

        if cx.tcx.item_name(def_id).as_str() != "spawn_local" {
            return;
        }

        let QPath::Resolved(_, path) = qpath else {
            return;
        };

        span_lint_and_help(
            cx,
            LEPTOS_UNSCOPED_SPAWN,
            path.span,
            "use of `leptos::task::spawn_local`",
            None,
            "prefer `leptos::task::spawn_local_scoped`. For further information visit https://github.com/leptos-rs/leptos-lints/tree/main/lints/leptos_unscoped_spawn#readme",
        );
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ui_examples() {
        dylint_testing::ui_test_examples(env!("CARGO_PKG_NAME"));
    }
}
