use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    LitInt, Result, Token,
};

mod kw {
    syn::custom_keyword!(year);
    syn::custom_keyword!(day);
    syn::custom_keyword!(part);
}

enum Arg {
    Year(LitInt),
    Day(LitInt),
    Part(LitInt),
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::year) {
            input.parse::<kw::year>()?;
            input.parse::<Token![=]>()?;
            Ok(Self::Year(input.parse()?))
        } else if lookahead.peek(kw::day) {
            input.parse::<kw::day>()?;
            input.parse::<Token![=]>()?;
            Ok(Self::Day(input.parse()?))
        } else if lookahead.peek(kw::part) {
            input.parse::<kw::part>()?;
            input.parse::<Token![=]>()?;
            Ok(Self::Part(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

pub struct AocAttribute {
    pub year: u16,
    pub day: u8,
    pub part: u8,
}

impl Parse for AocAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let args = Punctuated::<Arg, Token![,]>::parse_terminated(input)?;

        let mut day = None;
        let mut part = None;
        let mut year = None;

        for arg in args {
            match arg {
                Arg::Day(value) => day = Some(value.base10_parse()?),
                Arg::Year(value) => year = Some(value.base10_parse()?),
                Arg::Part(value) => part = Some(value.base10_parse()?),
            }
        }

        Ok(Self {
            year: year.unwrap(),
            day: day.unwrap(),
            part: part.unwrap(),
        })
    }
}
