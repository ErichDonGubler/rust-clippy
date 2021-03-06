use crate::consts::{constant, Constant};
use crate::utils::{is_trait_method, span_lint};
use rustc_hir as hir;
use rustc_lint::LateContext;
use rustc_span::sym;

use super::ITERATOR_STEP_BY_ZERO;

pub(super) fn check<'tcx>(cx: &LateContext<'tcx>, expr: &hir::Expr<'_>, args: &'tcx [hir::Expr<'_>]) {
    if is_trait_method(cx, expr, sym::Iterator) {
        if let Some((Constant::Int(0), _)) = constant(cx, cx.typeck_results(), &args[1]) {
            span_lint(
                cx,
                ITERATOR_STEP_BY_ZERO,
                expr.span,
                "`Iterator::step_by(0)` will panic at runtime",
            );
        }
    }
}
