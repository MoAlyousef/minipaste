#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Ident, Span, TokenStream as TS2, TokenTree};

#[proc_macro]
pub fn paste(input: TokenStream) -> TokenStream {
    fn expand(ts: TS2) -> TS2 {
        let mut out = TS2::new();
        for tt in ts {
            match tt {
                TokenTree::Group(g) => {
                    let del = g.delimiter();
                    if del == Delimiter::Bracket {
                        let mut inner = g.stream().into_iter().peekable();
                        if matches!(inner.peek(), Some(TokenTree::Punct(p)) if p.as_char() == '<') {
                            let _lt = inner.next();
                            let mut name = String::new();
                            let mut closed = false;
                            for t in inner {
                                match t {
                                    TokenTree::Punct(p) if p.as_char() == '>' => {
                                        closed = true;
                                        break;
                                    }
                                    TokenTree::Ident(id) => name.push_str(&id.to_string()),
                                    TokenTree::Punct(p) if p.as_char() == '_' => name.push('_'),
                                    TokenTree::Literal(lit) => {
                                        let s = lit.to_string();
                                        let trimmed = s
                                            .strip_prefix('"')
                                            .and_then(|x| x.strip_suffix('"'))
                                            .unwrap_or(&s);
                                        name.push_str(trimmed);
                                    }
                                    TokenTree::Punct(p) if p.as_char().is_ascii_alphanumeric() => {
                                        name.push(p.as_char());
                                    }
                                    _ => {}
                                }
                            }
                            if closed && !name.is_empty() {
                                out.extend([TokenTree::Ident(Ident::new(
                                    &name,
                                    Span::call_site(),
                                ))]);
                                continue;
                            }
                        }
                    }
                    let inner_expanded = expand(g.stream());
                    out.extend([TokenTree::Group(Group::new(del, inner_expanded))]);
                }
                other => out.extend([other]),
            }
        }
        out
    }
    expand(input.into()).into()
}
