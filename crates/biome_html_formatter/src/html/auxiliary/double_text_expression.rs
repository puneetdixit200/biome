use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_html_syntax::{HtmlDoubleTextExpression, HtmlDoubleTextExpressionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlDoubleTextExpression;
impl FormatNodeRule<HtmlDoubleTextExpression> for FormatHtmlDoubleTextExpression {
    fn fmt_fields(
        &self,
        node: &HtmlDoubleTextExpression,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let HtmlDoubleTextExpressionFields {
            l_double_curly_token,
            expression,
            r_double_curly_token,
        } = node.as_fields();

        let expression = expression?;
        let is_multiline = expression.html_literal_token()?.text().contains('\n');

        if is_multiline {
            return write!(
                f,
                [
                    l_double_curly_token.format(),
                    indent(&format_args![hard_line_break(), expression.format()]),
                    hard_line_break(),
                    r_double_curly_token.format(),
                ]
            );
        }

        write!(
            f,
            [
                l_double_curly_token.format(),
                space(),
                expression.format(),
                space(),
                r_double_curly_token.format(),
            ]
        )
    }
}
