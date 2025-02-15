use crate::{
    self as ast, Alias, Arguments, BoolOp, CmpOp, Comprehension, Constant, Decorator,
    ElifElseClause, ExceptHandler, Expr, Keyword, MatchCase, Mod, Operator, Parameter,
    ParameterWithDefault, Parameters, Pattern, Stmt, TypeParam, TypeParamTypeVar, TypeParams,
    UnaryOp, WithItem,
};

/// Visitor that traverses all nodes recursively in pre-order.
pub trait PreorderVisitor<'a> {
    fn visit_mod(&mut self, module: &'a Mod) {
        walk_module(self, module);
    }

    fn visit_stmt(&mut self, stmt: &'a Stmt) {
        walk_stmt(self, stmt);
    }

    fn visit_annotation(&mut self, expr: &'a Expr) {
        walk_annotation(self, expr);
    }

    fn visit_expr(&mut self, expr: &'a Expr) {
        walk_expr(self, expr);
    }

    fn visit_decorator(&mut self, decorator: &'a Decorator) {
        walk_decorator(self, decorator);
    }

    fn visit_constant(&mut self, _constant: &'a Constant) {}

    fn visit_bool_op(&mut self, bool_op: &'a BoolOp) {
        walk_bool_op(self, bool_op);
    }

    fn visit_operator(&mut self, operator: &'a Operator) {
        walk_operator(self, operator);
    }

    fn visit_unary_op(&mut self, unary_op: &'a UnaryOp) {
        walk_unary_op(self, unary_op);
    }

    fn visit_cmp_op(&mut self, cmp_op: &'a CmpOp) {
        walk_cmp_op(self, cmp_op);
    }

    fn visit_comprehension(&mut self, comprehension: &'a Comprehension) {
        walk_comprehension(self, comprehension);
    }

    fn visit_except_handler(&mut self, except_handler: &'a ExceptHandler) {
        walk_except_handler(self, except_handler);
    }

    fn visit_format_spec(&mut self, format_spec: &'a Expr) {
        walk_format_spec(self, format_spec);
    }

    fn visit_arguments(&mut self, arguments: &'a Arguments) {
        walk_arguments(self, arguments);
    }

    fn visit_parameters(&mut self, parameters: &'a Parameters) {
        walk_parameters(self, parameters);
    }

    fn visit_parameter(&mut self, arg: &'a Parameter) {
        walk_parameter(self, arg);
    }

    fn visit_parameter_with_default(&mut self, parameter_with_default: &'a ParameterWithDefault) {
        walk_parameter_with_default(self, parameter_with_default);
    }

    fn visit_keyword(&mut self, keyword: &'a Keyword) {
        walk_keyword(self, keyword);
    }

    fn visit_alias(&mut self, alias: &'a Alias) {
        walk_alias(self, alias);
    }

    fn visit_with_item(&mut self, with_item: &'a WithItem) {
        walk_with_item(self, with_item);
    }

    fn visit_type_params(&mut self, type_params: &'a TypeParams) {
        walk_type_params(self, type_params);
    }

    fn visit_type_param(&mut self, type_param: &'a TypeParam) {
        walk_type_param(self, type_param);
    }

    fn visit_match_case(&mut self, match_case: &'a MatchCase) {
        walk_match_case(self, match_case);
    }

    fn visit_pattern(&mut self, pattern: &'a Pattern) {
        walk_pattern(self, pattern);
    }

    fn visit_body(&mut self, body: &'a [Stmt]) {
        walk_body(self, body);
    }

    fn visit_elif_else_clause(&mut self, elif_else_clause: &'a ElifElseClause) {
        walk_elif_else_clause(self, elif_else_clause);
    }
}

pub fn walk_module<'a, V>(visitor: &mut V, module: &'a Mod)
where
    V: PreorderVisitor<'a> + ?Sized,
{
    match module {
        Mod::Module(ast::ModModule { body, range: _ }) => {
            visitor.visit_body(body);
        }
        Mod::Expression(ast::ModExpression { body, range: _ }) => visitor.visit_expr(body),
    }
}

pub fn walk_body<'a, V>(visitor: &mut V, body: &'a [Stmt])
where
    V: PreorderVisitor<'a> + ?Sized,
{
    for stmt in body {
        visitor.visit_stmt(stmt);
    }
}

pub fn walk_stmt<'a, V>(visitor: &mut V, stmt: &'a Stmt)
where
    V: PreorderVisitor<'a> + ?Sized,
{
    match stmt {
        Stmt::Expr(ast::StmtExpr { value, range: _ }) => visitor.visit_expr(value),

        Stmt::FunctionDef(ast::StmtFunctionDef {
            parameters,
            body,
            decorator_list,
            returns,
            type_params,
            ..
        })
        | Stmt::AsyncFunctionDef(ast::StmtAsyncFunctionDef {
            parameters,
            body,
            decorator_list,
            returns,
            type_params,
            ..
        }) => {
            for decorator in decorator_list {
                visitor.visit_decorator(decorator);
            }

            if let Some(type_params) = type_params {
                visitor.visit_type_params(type_params);
            }

            visitor.visit_parameters(parameters);

            for expr in returns {
                visitor.visit_annotation(expr);
            }

            visitor.visit_body(body);
        }

        Stmt::ClassDef(ast::StmtClassDef {
            arguments,
            body,
            decorator_list,
            type_params,
            ..
        }) => {
            for decorator in decorator_list {
                visitor.visit_decorator(decorator);
            }

            if let Some(type_params) = type_params {
                visitor.visit_type_params(type_params);
            }

            if let Some(arguments) = arguments {
                visitor.visit_arguments(arguments);
            }

            visitor.visit_body(body);
        }

        Stmt::Return(ast::StmtReturn { value, range: _ }) => {
            if let Some(expr) = value {
                visitor.visit_expr(expr);
            }
        }

        Stmt::Delete(ast::StmtDelete { targets, range: _ }) => {
            for expr in targets {
                visitor.visit_expr(expr);
            }
        }

        Stmt::TypeAlias(ast::StmtTypeAlias {
            range: _,
            name,
            type_params,
            value,
        }) => {
            visitor.visit_expr(name);
            if let Some(type_params) = type_params {
                visitor.visit_type_params(type_params);
            }
            visitor.visit_expr(value);
        }

        Stmt::Assign(ast::StmtAssign {
            targets,
            value,
            range: _,
        }) => {
            for expr in targets {
                visitor.visit_expr(expr);
            }

            visitor.visit_expr(value);
        }

        Stmt::AugAssign(ast::StmtAugAssign {
            target,
            op,
            value,
            range: _,
        }) => {
            visitor.visit_expr(target);
            visitor.visit_operator(op);
            visitor.visit_expr(value);
        }

        Stmt::AnnAssign(ast::StmtAnnAssign {
            target,
            annotation,
            value,
            range: _,
            simple: _,
        }) => {
            visitor.visit_expr(target);
            visitor.visit_annotation(annotation);
            if let Some(expr) = value {
                visitor.visit_expr(expr);
            }
        }

        Stmt::For(ast::StmtFor {
            target,
            iter,
            body,
            orelse,
            ..
        })
        | Stmt::AsyncFor(ast::StmtAsyncFor {
            target,
            iter,
            body,
            orelse,
            ..
        }) => {
            visitor.visit_expr(target);
            visitor.visit_expr(iter);
            visitor.visit_body(body);
            visitor.visit_body(orelse);
        }

        Stmt::While(ast::StmtWhile {
            test,
            body,
            orelse,
            range: _,
        }) => {
            visitor.visit_expr(test);
            visitor.visit_body(body);
            visitor.visit_body(orelse);
        }

        Stmt::If(ast::StmtIf {
            test,
            body,
            elif_else_clauses,
            range: _,
        }) => {
            visitor.visit_expr(test);
            visitor.visit_body(body);
            for clause in elif_else_clauses {
                visitor.visit_elif_else_clause(clause);
            }
        }

        Stmt::With(ast::StmtWith {
            items,
            body,
            range: _,
        })
        | Stmt::AsyncWith(ast::StmtAsyncWith {
            items,
            body,
            range: _,
        }) => {
            for with_item in items {
                visitor.visit_with_item(with_item);
            }
            visitor.visit_body(body);
        }

        Stmt::Match(ast::StmtMatch {
            subject,
            cases,
            range: _,
        }) => {
            visitor.visit_expr(subject);
            for match_case in cases {
                visitor.visit_match_case(match_case);
            }
        }

        Stmt::Raise(ast::StmtRaise {
            exc,
            cause,
            range: _,
        }) => {
            if let Some(expr) = exc {
                visitor.visit_expr(expr);
            };
            if let Some(expr) = cause {
                visitor.visit_expr(expr);
            };
        }

        Stmt::Try(ast::StmtTry {
            body,
            handlers,
            orelse,
            finalbody,
            range: _,
        })
        | Stmt::TryStar(ast::StmtTryStar {
            body,
            handlers,
            orelse,
            finalbody,
            range: _,
        }) => {
            visitor.visit_body(body);
            for except_handler in handlers {
                visitor.visit_except_handler(except_handler);
            }
            visitor.visit_body(orelse);
            visitor.visit_body(finalbody);
        }

        Stmt::Assert(ast::StmtAssert {
            test,
            msg,
            range: _,
        }) => {
            visitor.visit_expr(test);
            if let Some(expr) = msg {
                visitor.visit_expr(expr);
            }
        }

        Stmt::Import(ast::StmtImport { names, range: _ }) => {
            for alias in names {
                visitor.visit_alias(alias);
            }
        }

        Stmt::ImportFrom(ast::StmtImportFrom {
            range: _,
            module: _,
            names,
            level: _,
        }) => {
            for alias in names {
                visitor.visit_alias(alias);
            }
        }

        Stmt::Pass(_)
        | Stmt::Break(_)
        | Stmt::Continue(_)
        | Stmt::Global(_)
        | Stmt::Nonlocal(_)
        | Stmt::LineMagic(_) => {}
    }
}

pub fn walk_annotation<'a, V: PreorderVisitor<'a> + ?Sized>(visitor: &mut V, expr: &'a Expr) {
    visitor.visit_expr(expr);
}

pub fn walk_decorator<'a, V>(visitor: &mut V, decorator: &'a Decorator)
where
    V: PreorderVisitor<'a> + ?Sized,
{
    visitor.visit_expr(&decorator.expression);
}

pub fn walk_expr<'a, V>(visitor: &mut V, expr: &'a Expr)
where
    V: PreorderVisitor<'a> + ?Sized,
{
    match expr {
        Expr::BoolOp(ast::ExprBoolOp {
            op,
            values,
            range: _,
        }) => match values.as_slice() {
            [left, rest @ ..] => {
                visitor.visit_expr(left);
                visitor.visit_bool_op(op);
                for expr in rest {
                    visitor.visit_expr(expr);
                }
            }
            [] => {
                visitor.visit_bool_op(op);
            }
        },

        Expr::NamedExpr(ast::ExprNamedExpr {
            target,
            value,
            range: _,
        }) => {
            visitor.visit_expr(target);
            visitor.visit_expr(value);
        }

        Expr::BinOp(ast::ExprBinOp {
            left,
            op,
            right,
            range: _,
        }) => {
            visitor.visit_expr(left);
            visitor.visit_operator(op);
            visitor.visit_expr(right);
        }

        Expr::UnaryOp(ast::ExprUnaryOp {
            op,
            operand,
            range: _,
        }) => {
            visitor.visit_unary_op(op);
            visitor.visit_expr(operand);
        }

        Expr::Lambda(ast::ExprLambda {
            parameters,
            body,
            range: _,
        }) => {
            visitor.visit_parameters(parameters);
            visitor.visit_expr(body);
        }

        Expr::IfExp(ast::ExprIfExp {
            test,
            body,
            orelse,
            range: _,
        }) => {
            // `body if test else orelse`
            visitor.visit_expr(body);
            visitor.visit_expr(test);
            visitor.visit_expr(orelse);
        }

        Expr::Dict(ast::ExprDict {
            keys,
            values,
            range: _,
        }) => {
            for (key, value) in keys.iter().zip(values) {
                if let Some(key) = key {
                    visitor.visit_expr(key);
                }
                visitor.visit_expr(value);
            }
        }

        Expr::Set(ast::ExprSet { elts, range: _ }) => {
            for expr in elts {
                visitor.visit_expr(expr);
            }
        }

        Expr::ListComp(ast::ExprListComp {
            elt,
            generators,
            range: _,
        }) => {
            visitor.visit_expr(elt);
            for comprehension in generators {
                visitor.visit_comprehension(comprehension);
            }
        }

        Expr::SetComp(ast::ExprSetComp {
            elt,
            generators,
            range: _,
        }) => {
            visitor.visit_expr(elt);
            for comprehension in generators {
                visitor.visit_comprehension(comprehension);
            }
        }

        Expr::DictComp(ast::ExprDictComp {
            key,
            value,
            generators,
            range: _,
        }) => {
            visitor.visit_expr(key);
            visitor.visit_expr(value);

            for comprehension in generators {
                visitor.visit_comprehension(comprehension);
            }
        }

        Expr::GeneratorExp(ast::ExprGeneratorExp {
            elt,
            generators,
            range: _,
        }) => {
            visitor.visit_expr(elt);
            for comprehension in generators {
                visitor.visit_comprehension(comprehension);
            }
        }

        Expr::Await(ast::ExprAwait { value, range: _ })
        | Expr::YieldFrom(ast::ExprYieldFrom { value, range: _ }) => visitor.visit_expr(value),

        Expr::Yield(ast::ExprYield { value, range: _ }) => {
            if let Some(expr) = value {
                visitor.visit_expr(expr);
            }
        }

        Expr::Compare(ast::ExprCompare {
            left,
            ops,
            comparators,
            range: _,
        }) => {
            visitor.visit_expr(left);

            for (op, comparator) in ops.iter().zip(comparators) {
                visitor.visit_cmp_op(op);
                visitor.visit_expr(comparator);
            }
        }

        Expr::Call(ast::ExprCall {
            func,
            arguments,
            range: _,
        }) => {
            visitor.visit_expr(func);
            visitor.visit_arguments(arguments);
        }

        Expr::FormattedValue(ast::ExprFormattedValue {
            value, format_spec, ..
        }) => {
            visitor.visit_expr(value);

            if let Some(expr) = format_spec {
                visitor.visit_format_spec(expr);
            }
        }

        Expr::JoinedStr(ast::ExprJoinedStr { values, range: _ }) => {
            for expr in values {
                visitor.visit_expr(expr);
            }
        }

        Expr::Constant(ast::ExprConstant {
            value,
            range: _,
            kind: _,
        }) => visitor.visit_constant(value),

        Expr::Attribute(ast::ExprAttribute {
            value,
            attr: _,
            ctx: _,
            range: _,
        }) => {
            visitor.visit_expr(value);
        }

        Expr::Subscript(ast::ExprSubscript {
            value,
            slice,
            ctx: _,
            range: _,
        }) => {
            visitor.visit_expr(value);
            visitor.visit_expr(slice);
        }
        Expr::Starred(ast::ExprStarred {
            value,
            ctx: _,
            range: _,
        }) => {
            visitor.visit_expr(value);
        }

        Expr::Name(ast::ExprName {
            id: _,
            ctx: _,
            range: _,
        }) => {}

        Expr::List(ast::ExprList {
            elts,
            ctx: _,
            range: _,
        }) => {
            for expr in elts {
                visitor.visit_expr(expr);
            }
        }
        Expr::Tuple(ast::ExprTuple {
            elts,
            ctx: _,
            range: _,
        }) => {
            for expr in elts {
                visitor.visit_expr(expr);
            }
        }

        Expr::Slice(ast::ExprSlice {
            lower,
            upper,
            step,
            range: _,
        }) => {
            if let Some(expr) = lower {
                visitor.visit_expr(expr);
            }
            if let Some(expr) = upper {
                visitor.visit_expr(expr);
            }
            if let Some(expr) = step {
                visitor.visit_expr(expr);
            }
        }
        Expr::LineMagic(_) => (),
    }
}

pub fn walk_comprehension<'a, V>(visitor: &mut V, comprehension: &'a Comprehension)
where
    V: PreorderVisitor<'a> + ?Sized,
{
    visitor.visit_expr(&comprehension.target);
    visitor.visit_expr(&comprehension.iter);

    for expr in &comprehension.ifs {
        visitor.visit_expr(expr);
    }
}

pub fn walk_elif_else_clause<'a, V>(visitor: &mut V, elif_else_clause: &'a ElifElseClause)
where
    V: PreorderVisitor<'a> + ?Sized,
{
    if let Some(test) = &elif_else_clause.test {
        visitor.visit_expr(test);
    }
    visitor.visit_body(&elif_else_clause.body);
}

pub fn walk_except_handler<'a, V>(visitor: &mut V, except_handler: &'a ExceptHandler)
where
    V: PreorderVisitor<'a> + ?Sized,
{
    match except_handler {
        ExceptHandler::ExceptHandler(ast::ExceptHandlerExceptHandler {
            range: _,
            type_,
            name: _,
            body,
        }) => {
            if let Some(expr) = type_ {
                visitor.visit_expr(expr);
            }
            visitor.visit_body(body);
        }
    }
}

pub fn walk_format_spec<'a, V: PreorderVisitor<'a> + ?Sized>(
    visitor: &mut V,
    format_spec: &'a Expr,
) {
    visitor.visit_expr(format_spec);
}

pub fn walk_arguments<'a, V>(visitor: &mut V, arguments: &'a Arguments)
where
    V: PreorderVisitor<'a> + ?Sized,
{
    for arg in &arguments.args {
        visitor.visit_expr(arg);
    }

    for keyword in &arguments.keywords {
        visitor.visit_keyword(keyword);
    }
}

pub fn walk_parameters<'a, V>(visitor: &mut V, parameters: &'a Parameters)
where
    V: PreorderVisitor<'a> + ?Sized,
{
    for arg in parameters.posonlyargs.iter().chain(&parameters.args) {
        visitor.visit_parameter_with_default(arg);
    }

    if let Some(arg) = &parameters.vararg {
        visitor.visit_parameter(arg);
    }

    for arg in &parameters.kwonlyargs {
        visitor.visit_parameter_with_default(arg);
    }

    if let Some(arg) = &parameters.kwarg {
        visitor.visit_parameter(arg);
    }
}

pub fn walk_parameter<'a, V>(visitor: &mut V, parameter: &'a Parameter)
where
    V: PreorderVisitor<'a> + ?Sized,
{
    if let Some(expr) = &parameter.annotation {
        visitor.visit_annotation(expr);
    }
}

pub fn walk_parameter_with_default<'a, V>(
    visitor: &mut V,
    parameter_with_default: &'a ParameterWithDefault,
) where
    V: PreorderVisitor<'a> + ?Sized,
{
    visitor.visit_parameter(&parameter_with_default.parameter);
    if let Some(expr) = &parameter_with_default.default {
        visitor.visit_expr(expr);
    }
}

#[inline]
pub fn walk_keyword<'a, V>(visitor: &mut V, keyword: &'a Keyword)
where
    V: PreorderVisitor<'a> + ?Sized,
{
    visitor.visit_expr(&keyword.value);
}

pub fn walk_with_item<'a, V>(visitor: &mut V, with_item: &'a WithItem)
where
    V: PreorderVisitor<'a> + ?Sized,
{
    visitor.visit_expr(&with_item.context_expr);

    if let Some(expr) = &with_item.optional_vars {
        visitor.visit_expr(expr);
    }
}

pub fn walk_type_params<'a, V>(visitor: &mut V, type_params: &'a TypeParams)
where
    V: PreorderVisitor<'a> + ?Sized,
{
    for type_param in &type_params.type_params {
        visitor.visit_type_param(type_param);
    }
}

pub fn walk_type_param<'a, V>(visitor: &mut V, type_param: &'a TypeParam)
where
    V: PreorderVisitor<'a> + ?Sized,
{
    match type_param {
        TypeParam::TypeVar(TypeParamTypeVar {
            bound,
            name: _,
            range: _,
        }) => {
            if let Some(expr) = bound {
                visitor.visit_expr(expr);
            }
        }
        TypeParam::TypeVarTuple(_) | TypeParam::ParamSpec(_) => {}
    }
}

pub fn walk_match_case<'a, V>(visitor: &mut V, match_case: &'a MatchCase)
where
    V: PreorderVisitor<'a> + ?Sized,
{
    visitor.visit_pattern(&match_case.pattern);
    if let Some(expr) = &match_case.guard {
        visitor.visit_expr(expr);
    }
    visitor.visit_body(&match_case.body);
}

pub fn walk_pattern<'a, V>(visitor: &mut V, pattern: &'a Pattern)
where
    V: PreorderVisitor<'a> + ?Sized,
{
    match pattern {
        Pattern::MatchValue(ast::PatternMatchValue { value, range: _ }) => {
            visitor.visit_expr(value);
        }

        Pattern::MatchSingleton(ast::PatternMatchSingleton { value, range: _ }) => {
            visitor.visit_constant(value);
        }

        Pattern::MatchSequence(ast::PatternMatchSequence { patterns, range: _ }) => {
            for pattern in patterns {
                visitor.visit_pattern(pattern);
            }
        }

        Pattern::MatchMapping(ast::PatternMatchMapping {
            keys,
            patterns,
            range: _,
            rest: _,
        }) => {
            for (key, pattern) in keys.iter().zip(patterns) {
                visitor.visit_expr(key);
                visitor.visit_pattern(pattern);
            }
        }

        Pattern::MatchClass(ast::PatternMatchClass {
            cls,
            patterns,
            kwd_attrs: _,
            kwd_patterns,
            range: _,
        }) => {
            visitor.visit_expr(cls);
            for pattern in patterns {
                visitor.visit_pattern(pattern);
            }

            for pattern in kwd_patterns {
                visitor.visit_pattern(pattern);
            }
        }

        Pattern::MatchStar(_) => {}

        Pattern::MatchAs(ast::PatternMatchAs {
            pattern,
            range: _,
            name: _,
        }) => {
            if let Some(pattern) = pattern {
                visitor.visit_pattern(pattern);
            }
        }

        Pattern::MatchOr(ast::PatternMatchOr { patterns, range: _ }) => {
            for pattern in patterns {
                visitor.visit_pattern(pattern);
            }
        }
    }
}

pub fn walk_bool_op<'a, V>(_visitor: &mut V, _bool_op: &'a BoolOp)
where
    V: PreorderVisitor<'a> + ?Sized,
{
}

#[inline]
pub fn walk_operator<'a, V>(_visitor: &mut V, _operator: &'a Operator)
where
    V: PreorderVisitor<'a> + ?Sized,
{
}

#[inline]
pub fn walk_unary_op<'a, V>(_visitor: &mut V, _unary_op: &'a UnaryOp)
where
    V: PreorderVisitor<'a> + ?Sized,
{
}

#[inline]
pub fn walk_cmp_op<'a, V>(_visitor: &mut V, _cmp_op: &'a CmpOp)
where
    V: PreorderVisitor<'a> + ?Sized,
{
}

#[inline]
pub fn walk_alias<'a, V>(_visitor: &mut V, _alias: &'a Alias)
where
    V: PreorderVisitor<'a> + ?Sized,
{
}
