#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;

use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::{Expr, ExprKind, QPath};
use rustc_lint::{LateContext, LateLintPass};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Warn about usages of `event_target`. The purpose of this lint is
    /// to make users know that the function may panic under certain circumstances.
    ///
    /// ### Why is this bad?
    ///
    /// The function `leptos::prelude::event_target` will panic at run-time if the
    /// event doesn't have a target (eg. `web_sys::FetchEvent`) or is manually
    /// created with `web_sys::Event::new` or `web_sys::MouseEvent::new`. This
    /// can introduce hard-to-debug runtime errors in Leptos applications
    /// that produce panics at run-time.
    ///
    /// To reproduce:
    ///
    /// ```rust,ignore
    /// let event = web_sys::Event::new("visibilitychange").unwrap();
    /// let target = leptos::prelude::event_target::<web_sys::Document>(&event);
    /// ```
    ///
    /// The underlying problem is that the MDN APIs don't guarantee at type level
    /// if `target` is null or not.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let target = leptos::prelude::event_target::<web_sys::Document>(&event);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let target = match event.target() {
    ///     Some(t) => t.dyn_into::<web_sys::Document>().ok(),
    ///     None => {
    ///        // handle the case where there is no target
    ///        todo!()
    ///     },
    /// };
    /// ```
    ///
    /// Or allow the lint if you're sure that the event has a target:
    ///
    /// ```rust,ignore
    /// #[allow(leptos_event_target)]
    /// let target = leptos::prelude::event_target::<web_sys::Document>(&event);
    /// ```
    pub LEPTOS_EVENT_TARGET,
    Warn,
    "Warn about usages of `leptos::prelude::event_target`"
}

impl<'tcx> LateLintPass<'tcx> for LeptosEventTarget {
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

        if cx.tcx.item_name(def_id).as_str() != "event_target" {
            return;
        }

        let QPath::Resolved(_, path) = qpath else {
            return;
        };

        span_lint_and_help(
            cx,
            LEPTOS_EVENT_TARGET,
            path.span,
            "use of `leptos::prelude::event_target`",
            None,
            "this function can panic at run-time if the target doesn't have an event. \
                  For further information visit \
                  https://github.com/leptos-rs/leptos-lints/tree/main/lints/leptos_event_target#readme",
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
