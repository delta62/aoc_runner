use syn::{
    parse::{Parse, ParseStream},
    FnArg, Ident, ItemFn, PatType, Result, ReturnType, Type,
};

pub struct SolutionFunction {
    pub ident: Ident,
    pub input_pat: PatType,
    pub output_type: Box<Type>,
}

impl Parse for SolutionFunction {
    fn parse(input: ParseStream) -> Result<Self> {
        let f: ItemFn = input.parse()?;

        let ident = f.sig.ident;

        let output_type = match f.sig.output {
            ReturnType::Type(_, t) => t,
            _ => todo!(),
        };

        let input_arg = f.sig.inputs.into_iter().next().unwrap();
        let input_pat = match input_arg {
            FnArg::Typed(t) => t,
            _ => todo!(),
        };

        Ok(Self {
            ident,
            input_pat,
            output_type,
        })
    }
}
