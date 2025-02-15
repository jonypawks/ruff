use ruff_python_ast::{self as ast, Expr, Parameters, Stmt};

use ruff_python_ast::cast;
use ruff_python_semantic::analyze::visibility;
use ruff_python_semantic::{Definition, Member, MemberKind, SemanticModel};

pub(super) fn match_function_def(
    stmt: &Stmt,
) -> (&str, &Parameters, Option<&Expr>, &[Stmt], &[ast::Decorator]) {
    match stmt {
        Stmt::FunctionDef(ast::StmtFunctionDef {
            name,
            parameters,
            returns,
            body,
            decorator_list,
            ..
        })
        | Stmt::AsyncFunctionDef(ast::StmtAsyncFunctionDef {
            name,
            parameters,
            returns,
            body,
            decorator_list,
            ..
        }) => (
            name,
            parameters,
            returns.as_ref().map(AsRef::as_ref),
            body,
            decorator_list,
        ),
        _ => panic!("Found non-FunctionDef in match_function_def"),
    }
}

/// Return the name of the function, if it's overloaded.
pub(crate) fn overloaded_name(definition: &Definition, semantic: &SemanticModel) -> Option<String> {
    if let Definition::Member(Member {
        kind: MemberKind::Function | MemberKind::NestedFunction | MemberKind::Method,
        stmt,
        ..
    }) = definition
    {
        if visibility::is_overload(cast::decorator_list(stmt), semantic) {
            let (name, ..) = match_function_def(stmt);
            Some(name.to_string())
        } else {
            None
        }
    } else {
        None
    }
}

/// Return `true` if the definition is the implementation for an overloaded
/// function.
pub(crate) fn is_overload_impl(
    definition: &Definition,
    overloaded_name: &str,
    semantic: &SemanticModel,
) -> bool {
    if let Definition::Member(Member {
        kind: MemberKind::Function | MemberKind::NestedFunction | MemberKind::Method,
        stmt,
        ..
    }) = definition
    {
        if visibility::is_overload(cast::decorator_list(stmt), semantic) {
            false
        } else {
            let (name, ..) = match_function_def(stmt);
            name == overloaded_name
        }
    } else {
        false
    }
}
