extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Dot3, Fn, Paren, Unsafe};
use syn::{
    parenthesized, parse_macro_input, Abi, BareFnArg, BoundLifetimes,
    ExprBlock, Ident, Result, ReturnType,
};

struct TypeFnWithExprBlock {
    type_fn: TypeFn,
    expr_block: ExprBlock,
}

#[derive(Clone)]
struct TypeFn {
    lifetimes: Option<BoundLifetimes>,
    unsafety: Option<Unsafe>,
    abi: Option<Abi>,
    fn_token: Fn,
    fn_ident: Ident,
    paren_token: Paren,
    inputs: Punctuated<BareFnArg, Comma>,
    variadic: Option<Dot3>,
    output: ReturnType,
}

impl Parse for TypeFnWithExprBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(TypeFnWithExprBlock {
            type_fn: input.parse()?,
            expr_block: input.parse()?,
        })
    }
}

impl ToTokens for TypeFnWithExprBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.type_fn.to_token_stream());
        tokens.extend(self.expr_block.to_token_stream());
    }
}

impl Parse for TypeFn {
    fn parse(input: ParseStream) -> Result<Self> {
        let args_content;

        #[allow(clippy::eval_order_dependence)]
        Ok(TypeFn {
            lifetimes: input.parse()?,
            unsafety: input.parse()?,
            abi: input.parse()?,
            fn_token: input.parse()?,
            fn_ident: input.parse()?,
            paren_token: parenthesized!(args_content in input),
            inputs: args_content.parse_terminated(BareFnArg::parse)?,
            variadic: input.parse()?,
            output: input.parse()?,
        })
    }
}

impl ToTokens for TypeFn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.lifetimes.to_token_stream());
        tokens.extend(self.unsafety.to_token_stream());
        tokens.extend(self.abi.to_token_stream());
        tokens.extend(self.fn_token.to_token_stream());
        tokens.extend(self.fn_ident.to_token_stream());
        self.paren_token.surround(tokens, |tokens| {
            tokens.extend(self.inputs.to_token_stream());
        });
        tokens.extend(self.variadic.to_token_stream());
        tokens.extend(self.output.to_token_stream());
    }
}

#[proc_macro_attribute]
pub fn decorator(attr: TokenStream, input: TokenStream) -> TokenStream {
    let decor_ident = parse_macro_input!(attr as Ident);
    let target_fn = parse_macro_input!(input as TypeFnWithExprBlock);
    let type_fn = target_fn.type_fn.clone();

    let inner_ident = type_fn.fn_ident.clone();
    let inner_inputs = type_fn.inputs.clone();
    let inner_input_idents: Punctuated<Ident, Comma> = {
        let mut ret = Punctuated::new();
        for t in inner_inputs.iter() {
            ret.push_value(t.name.as_ref().unwrap().0.clone());
        }
        ret
    };

    let decorated_fn = quote! {
        #type_fn {
            #target_fn
            #decor_ident(#inner_ident, #inner_input_idents)
        }
    };

    decorated_fn.into()
}
