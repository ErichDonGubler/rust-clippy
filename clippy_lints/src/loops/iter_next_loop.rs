use super::ITER_NEXT_LOOP;
use rustc_hir::Expr;
use rustc_lint::LateContext;
use rustc_span::sym;

use crate::utils::{is_trait_method, span_lint};

pub(super) fn check(cx: &LateContext<'_>, arg: &Expr<'_>, expr: &Expr<'_>) -> bool {
    if is_trait_method(cx, arg, sym::Iterator) {
        span_lint(
            cx,
            ITER_NEXT_LOOP,
            expr.span,
            "you are iterating over `Iterator::next()` which is an Option; this will compile but is \
            probably not what you want",
        );
        true
    } else {
        false
    }
}
