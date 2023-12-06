use crate::{args::AocAttribute, solution_fn::SolutionFunction};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{parse_macro_input, GenericParam, Generics, ImplItemType, Lifetime, LifetimeParam};

pub fn aoc(attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let answer = parse_macro_input!(attr as AocAttribute);
    let day = answer.day;
    let part = answer.part;
    let f_item = item.clone();
    let f = parse_macro_input!(f_item as SolutionFunction);
    let struct_name = format_ident!("Day{day}Part{part}Solution");

    let solution_struct = quote! {
        #[derive(Default)]
        pub struct #struct_name;
    };

    let year = answer.year;
    let fn_name = f.ident;
    let output = f.output_type;

    let lifetime = LifetimeParam::new(Lifetime::new("'a", Span::call_site()));
    let lifetime = GenericParam::Lifetime(lifetime);
    let mut input_generics = Generics::default();
    input_generics.params.push(lifetime);

    let ty = *f.input_pat.ty;

    let input = ImplItemType {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        defaultness: None,
        type_token: Default::default(),
        ident: format_ident!("Input"),
        generics: input_generics,
        eq_token: Default::default(),
        ty,
        semi_token: Default::default(),
    };

    let solution_impl = quote! {
        impl aoc_runner::PuzzleSolution for #struct_name {
            #input
            type Output = #output;

            fn year(&self) -> u16 {
                #year
            }

            fn day(&self) -> u8 {
                #day
            }

            fn part(&self) -> u8 {
                #part
            }

            fn solve(&self, input: Self::Input<'_>) -> Self::Output {
                #fn_name(input)
            }
        }
    };

    let solution_collection_struct = quote! {
        ::aoc_runner::inventory::submit! {
            &#struct_name as &(dyn ::aoc_runner::Solution + Sync + 'static)
        }
    };

    let expanded = quote! {
        #solution_struct
        #solution_impl
        #solution_collection_struct
    };

    item.extend(TokenStream::from(expanded));

    item
}
