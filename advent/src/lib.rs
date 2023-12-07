use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, ItemFn,
    Visibility,
};

mod kw {
    syn::custom_keyword!(day);
    syn::custom_keyword!(part);
}

#[derive(Debug, FromMeta)]
struct DayArgs {
    day: u8,
    part: u8,
}

#[proc_macro_attribute]
pub fn advent_of_code(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };
    let DayArgs { day, part } = match DayArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };
    let struct_name = format_ident!("Day{day}Part{part}");
    let file_path = format!("./test_data/day_{day}");
    // println!("item: \"{}\"", item.to_string());
    let mut function = parse_macro_input!(item as ItemFn);
    function.vis = Visibility::Inherited;
    let function_name = &function.sig.ident;
    let tokens = quote! {
        #function

        pub struct #struct_name;

        impl DayPart for #struct_name {
            const FILE : &'static str = #file_path;
            const DAY : u8 = #day;
            const PART : u8 = #part;

            fn run(lines: impl Iterator<Item = String>) -> String {
                #function_name(lines)
            }
        }

        inventory::submit! {
            TestRunner::new::<#struct_name>()
        }
    };
    tokens.into()
}
