macro_rules! loop_over {
    ($id:ident $(, $ids:ident)* => $b:block) => {
        for $id in 0..crate::consts::DIMENSIONS {
            loop_over! ($($ids),* => $b)
        }
    };
    (=> $b:block) => {$b};
}

macro_rules! add_over_to {
    ($id:ident $(, $ids:ident)* => $b:block [$var:ident]) => {
        for $id in 0..crate::consts::DIMENSIONS {
            add_over_to! ($($ids),* => $b [$var])
        }
    };
    (=> $b:block [$var:ident]) => {$var += $b;};
}

macro_rules! type_default_to {
    (: $ty:ty, $def:ty) => {
        $ty
    };
    (, $def:ty) => {
        $def
    };
}

macro_rules! add_over {
    ($($ids:ident),+ => $b:block $(: $t:ty)?) => {
        {
            let mut x: type_default_to!($(: $t)?, f64) = Default::default();
            add_over_to!($($ids),+ => $b [x]);
            x
        }
    };
}

// extern crate proc_macro;
// extern crate syn;
// #[macro_use]
// extern crate quote;
// use proc_macro::TokenStream;
// macro_rules! define_tensor {
//     ($tensor_name:ident, $vals_type:ty, $vals_val:expr) => {
//         struct $tensor_name {
//             vals: $vals_type
//         }
//         impl Default for $tensor_name {
//             $tensor_name { vals: $vals_expr }
//         }
//     }
// }
//
// fn add_dimension_array(ast: &syn::DeriveInput) -> quote::Tokens {
//     quote! {
//
//     }
// }
//
// #[proc_macro]
// fn define_tensors(item: TokenStream) -> TokenStream {
//     for i in 1..=4 {
//         define_tensor!()
//     }
// }
//
// define_tensors!();
