use std::str::FromStr;

use proc_macro::{TokenStream, TokenTree};


#[proc_macro_attribute]
pub fn main(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    // let mut input = input.to_string();
    // input.push_str("#[no_mangle] ");
    // println!("attrs: {:?}", attrs);
    println!("input: {:?}", input);
    
    // // get name of function
    let mut name = String::new();
    let mut struc = false;
    let mut iter = input.clone().into_iter();
    while let Some(token) = iter.next() {
        if let TokenTree::Ident(ident) = token {
            if ident.to_string() == "struct" {
                struc = true;
                continue;
            }
            if struc {
                name = ident.to_string();
                break;
            }
        }
    };
    let output = format!("#[no_mangle]
    #[allow(dead_code)]
    fn create_instance() -> Box<dyn stf_interface::STF> {{
            Box::new({}::new())
    }}        
    {}
    ", name, input.to_string());
    println!("output: {:?}", output);
    println!("name: {:?}", name);


    TokenStream::from_str(&output).unwrap()
    // input   

}
