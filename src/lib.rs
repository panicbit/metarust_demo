use metarust::metarust;

macro_rules! gen_vec {
    ($($idents:ident),*$(,)?) => {
        metarust! {
            use itertools::iproduct;

            let fields = &[
                $(stringify!($idents)),*
            ]
            .iter()
            .map(|ident| format_ident!("{ident}"))
            .collect_vec();

            let products = (0..fields.len())
                .map(|_| fields)
                .multi_cartesian_product();


            let swizzle_funs = products
                .map(|product| {
                    let fun_ident = format_ident!("{}", product.iter().join(""));

                    quote! {
                        pub fn #fun_ident(&self) -> Self {
                            Self::new(#(self.#product),*)
                        }
                    }
                });

            let vec_ident = format_ident!("Vec{}", fields.len());

            quote! {
                #[derive(Debug)]
                pub struct #vec_ident {
                    #(#fields: f32),*
                }

                impl #vec_ident {
                    pub fn new(#(#fields: f32),*) -> Self {
                        Self { #(#fields),* }
                    }

                    #(#swizzle_funs)*
                }
            }
        }
    }
}


macro_rules! gen_vec_all {
    ($($idents:ident),*$(,)?) => {
        metarust! {
            let fields = [
                $(stringify!($idents)),*
            ]
            .iter()
            .map(|ident| format_ident!("{ident}"))
            .collect_vec();

            let vecs = (1..=fields.len())
                .map(|len| {
                    let fields = &fields[..len];

                    quote!{
                        gen_vec!(#(#fields),*);
                    }
                });

            quote! {
                #(#vecs)*
            }
        }
    }
}

gen_vec_all!(α, β, γ, δ, ε /*, ζ, η, θ, ι, κ, λ, μ, ν, ξ, ο */);
