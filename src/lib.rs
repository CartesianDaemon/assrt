use std::iter;
use proc_macro;

use std::str::FromStr;

use proc_macro::*;
use syn::*;
use quote::{quote, ToTokens};

#[proc_macro]
pub fn csst(condition: TokenStream) -> TokenStream {
    let input_stream = condition.clone();
    let expr = parse_macro_input!(input_stream as Expr);
    // Assume an assertion condition like "left op right" where left and right are expressions
    // and op is a comparison operator like "==" or "!=".
    // The message is chosen to make sense if left is an expression and right is a literal.
    match expr {
        Expr::Binary(binexpr) => {
            // Do things
            match binexpr.op {
                BinOp::Eq(_) |
                BinOp::Ne(_) |
                BinOp::Lt(_) |
                BinOp::Gt(_) |
                BinOp::Le(_) |
                BinOp::Ge(_) => {
                    let _xxx = Into::<TokenStream>::into(quote!{
                        6 * 9 == 42, "Expected 6 * 9 == 42, but..."
                    });

                    let comma = TokenTree::Punct(Punct::new(',', Spacing::Alone));

                    /* Create args TokenStream to represent:
                     *
                     * condition, "Assertion failed...", left-as-str, op-as-str, right-as-str, left-as-value
                     */
                    let mut args = TokenStream::new();
                    args.extend(iter::once(condition.clone()));
                    args.extend(iter::once(comma.clone()));
                    args.extend(iter::once(TokenTree::Literal(Literal::string("Assertion failed. Needed '{}' {} '{}' but was '{}'.\n"))));
                    args.extend(iter::once(comma.clone()));
                    args.extend(iter::once(TokenTree::Literal(Literal::string(&TokenStream::from(binexpr.left.to_token_stream()).to_string()))));
                    args.extend(iter::once(comma.clone()));
                    args.extend(iter::once(TokenTree::Literal(Literal::string(&TokenStream::from(binexpr.op.to_token_stream()).to_string()))));
                    args.extend(iter::once(comma.clone()));
                    args.extend(iter::once(TokenTree::Literal(Literal::string(&TokenStream::from(binexpr.right.to_token_stream()).to_string()))));
                    args.extend(iter::once(comma.clone()));
                    args.extend(iter::once(TokenStream::from(binexpr.left.to_token_stream())));

                    /* Construct a TokenStream to represent:
                     *
                     * assert! ( args )
                     */
                    let mut ret = TokenStream::from_str("assert!").unwrap();
                    ret.extend(iter::once(TokenTree::Group(Group::new(Delimiter::Parenthesis, args))));
                    ret
                }
                _ => panic!("Argument to csst must be a comparison expression."), // FIXME: Print tokens.
            }
        }
        _ => panic!("Argument to csst must be a binary expression."), // FIXME: Print tokens.
    }
}
