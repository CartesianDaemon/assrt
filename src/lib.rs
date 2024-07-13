use std::iter;
use proc_macro;

use std::str::FromStr;

use proc_macro::*;
use syn::*;
use syn::visit::{self, Visit};
use quote::{ToTokens};

struct ExprVisitor {
    exprs: Vec<proc_macro2::TokenStream>,
}

impl ExprVisitor {
    fn default() -> ExprVisitor {
        ExprVisitor { exprs: Vec::new() }
    }

    fn visit_expr_nocapture(&mut self, expr: &Expr) {
        self.visit_expr_optionalcapture(expr, false);
    }

    fn visit_expr_optionalcapture(&mut self, expr: &Expr, docapture: bool) {
        match expr {
            Expr::Binary(exp) => {
                match exp.op {
                    BinOp::Eq(_) |
                    BinOp::Ne(_) |
                    BinOp::Lt(_) |
                    BinOp::Gt(_) |
                    BinOp::Le(_) |
                    BinOp::Ge(_) => {
                        self.visit_expr(&*exp.left);
                        self.visit_expr(&*exp.right);
                    }
                    _ => {
                        if docapture {
                            self.exprs.push(exp.to_token_stream());
                        }
                        self.visit_expr_nocapture(&*exp.left);
                        self.visit_expr_nocapture(&*exp.right);
                    }
                }
            }
            Expr::Paren(exp) => {
                if docapture {
                    self.exprs.push(exp.to_token_stream());
                }
                self.visit_expr_nocapture(&*exp.expr);
            }
            Expr::Unary(exp) => {
                if docapture {
                    self.exprs.push(exp.to_token_stream());
                }
                self.visit_expr_nocapture(&*exp.expr);
            }

            Expr::Call(exp) => {
                self.exprs.push(exp.to_token_stream());
                visit::visit_expr(self, expr);
            }
            Expr::Field(exp) => {
                self.exprs.push(exp.to_token_stream());
                visit::visit_expr(self, expr);
            }
            Expr::Index(exp) => {
                self.exprs.push(exp.to_token_stream());
                visit::visit_expr(self, expr);
            }
            Expr::MethodCall(exp) => {
                self.exprs.push(exp.to_token_stream());
                visit::visit_expr(self, expr);
            }
            Expr::Path(exp) => {
                self.exprs.push(exp.to_token_stream());
                visit::visit_expr(self, expr);
            }
            _ => {
                visit::visit_expr(self, expr);
            }
        }
    }
}

impl Visit<'_> for ExprVisitor {
    fn visit_expr(&mut self, expr: &Expr) {
        self.visit_expr_optionalcapture(expr, true);
    }
}

#[proc_macro]
pub fn rsst(tokens: TokenStream) -> TokenStream {
    /* Parse condition and find non-trivial expressions: */

    let condition = tokens.clone();
    let syntax_tree: Expr = parse_macro_input!(tokens as Expr);
    let mut visitor = ExprVisitor::default();
    visitor.visit_expr_nocapture(&syntax_tree);

    if false {
        println!("Exprs:");
        for e in &visitor.exprs {
            println!("{}", e.to_string());
            if false { println!("{:?}", e); }
        }
    }

    /* Construct replacement TokenStream to return: */

    let comma = TokenTree::Punct(Punct::new(',', Spacing::Alone));

    let mut msg = "Assertion failed: ".to_string();
    msg.push_str(&condition.to_string());
    for toks in &visitor.exprs {
        msg.push_str("\n  ");
        msg.push_str(&toks.to_string());
        msg.push_str(": {:?}");
    }

    /* Create args TokenStream to represent arguments we'll give to assert.
     */
    let mut args = TokenStream::new();
    args.extend(iter::once(condition.clone()));
    args.extend(iter::once(comma.clone()));
    args.extend(iter::once(TokenTree::Literal(Literal::string(&msg))));
    for toks in &visitor.exprs {
        args.extend(iter::once(comma.clone()));
        args.extend(iter::once(TokenStream::from(toks.clone())));
    }

    /* Construct a TokenStream to represent:
     *
     * assert! ( args )
     */
    let mut ret = TokenStream::from_str("assert!").unwrap();
    ret.extend(iter::once(TokenTree::Group(Group::new(Delimiter::Parenthesis, args))));
    ret
}

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
                    let comma = TokenTree::Punct(Punct::new(',', Spacing::Alone));

                    /* Create args TokenStream to represent:
                     *
                     * condition, "Assertion failed...", left-as-str, op-as-str, right-as-str, left-as-value
                     */
                    let mut args = TokenStream::new();
                    args.extend(iter::once(condition));
                    args.extend(iter::once(comma.clone()));
                    args.extend(iter::once(TokenTree::Literal(Literal::string("Assertion failed: needed '{}' {} '{}' but was '{}'.\n"))));
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
