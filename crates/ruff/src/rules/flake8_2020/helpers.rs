use rustpython_parser::ast::Expr;

use ruff_python_semantic::SemanticModel;

pub(super) fn is_sys(model: &SemanticModel, expr: &Expr, target: &str) -> bool {
    model
        .resolve_call_path(expr)
        .map_or(false, |call_path| call_path.as_slice() == ["sys", target])
}
