use ruff_formatter::prelude::{
    format_args, format_with, group, soft_line_break_or_space, space, text,
};
use ruff_formatter::{write, Buffer, FormatResult};
use ruff_python_ast::node::AnyNodeRef;
use ruff_python_ast::ExprDictComp;

use crate::context::PyFormatContext;
use crate::expression::parentheses::{
    parenthesized_with_dangling_comments, NeedsParentheses, OptionalParentheses,
};
use crate::AsFormat;
use crate::{FormatNodeRule, FormattedIterExt, PyFormatter};

#[derive(Default)]
pub struct FormatExprDictComp;

impl FormatNodeRule<ExprDictComp> for FormatExprDictComp {
    fn fmt_fields(&self, item: &ExprDictComp, f: &mut PyFormatter) -> FormatResult<()> {
        let ExprDictComp {
            range: _,
            key,
            value,
            generators,
        } = item;

        let joined = format_with(|f| {
            f.join_with(soft_line_break_or_space())
                .entries(generators.iter().formatted())
                .finish()
        });

        let comments = f.context().comments().clone();
        let dangling = comments.dangling_comments(item);

        write!(
            f,
            [parenthesized_with_dangling_comments(
                "{",
                dangling,
                &group(&format_args!(
                    group(&key.format()),
                    text(":"),
                    space(),
                    value.format(),
                    soft_line_break_or_space(),
                    &joined
                )),
                "}"
            )]
        )
    }
}

impl NeedsParentheses for ExprDictComp {
    fn needs_parentheses(
        &self,
        _parent: AnyNodeRef,
        _context: &PyFormatContext,
    ) -> OptionalParentheses {
        OptionalParentheses::Never
    }
}
