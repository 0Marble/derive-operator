use proc_macro::*;
mod derive;
mod parse;

#[proc_macro]
/// Generates an impl block with given func_body; operator is +-*/&|^
/// Generics are optional
///
/// \[T1,T2...\] where \[T1: MyTrait,...\] (lhs_name: LhsType) operator (rhs_name: RhsType) = (ResType) { function_body }
pub fn derive_operator(t: TokenStream) -> TokenStream {
    let s = derive::derive_operator(t);
    // println!("{}", s);
    s.parse().unwrap()
}

#[proc_macro]
/// If &lhs op &rhs = res exists, implements the rest of the variants, plus assign operators.
/// Generics are optional.
///
/// # Example:
///
/// ```
/// # #[macro_use] extern crate derive_operator;
/// # fn main() {
/// use std::{fmt::Debug, ops::Add};
///
/// #[derive(Debug, Clone, PartialEq)]
/// struct MyStruct<T: Debug + Clone + PartialEq> {
///     val: T,
/// }
///
/// derive_operator!(['a, 'b, T]
///     where [T: Debug + Clone + PartialEq, &'a T: Add<&'b T, Output = T>]
///     (a: &'a MyStruct<T>) + (b: &'b MyStruct<T>) = (MyStruct<T>) {
///         MyStruct { val: &a.val + &b.val }
///     }
/// );
/// finish_derive!([T]
///     where [T: Debug + Clone + PartialEq, for<'a, 'b> &'a T: Add<&'b T, Output = T>]
///     (MyStruct<T>) + (MyStruct<T>) = (MyStruct<T>)
/// );
///
/// derive_operator!((a: &MyStruct<f32>) * (b: &f32) = (MyStruct<f32>) {
///     MyStruct { val: &a.val * b }
/// });
/// finish_derive!((MyStruct<f32>) * (f32) = (MyStruct<f32>));
///
/// #[test]
/// fn test_all() {
///     let mut a = MyStruct { val: 1.0f32 };
///     assert_eq!(a.clone() + a.clone(), MyStruct { val: 2.0f32 });
///     assert_eq!(a.clone() + &a, MyStruct { val: 2.0f32 });
///     assert_eq!(&a + a.clone(), MyStruct { val: 2.0f32 });
///     assert_eq!(&a + &a, MyStruct { val: 2.0f32 });
///
///     let mut b = a.clone();
///     a += &b;
///     assert_eq!(a, MyStruct { val: 2.0f32 });
///     a += b.clone();
///     assert_eq!(a, MyStruct { val: 3.0f32 });
///
///     assert_eq!(b.clone() * 2.0f32, MyStruct { val: 2.0f32 });
///     assert_eq!(b.clone() * &2.0f32, MyStruct { val: 2.0f32 });
///     assert_eq!(&b * &2.0f32, MyStruct { val: 2.0f32 });
///     assert_eq!(&b * 2.0f32, MyStruct { val: 2.0f32 });
///     b *= 2.0;
///     assert_eq!(b, MyStruct { val: 2.0f32 });
///     b *= &2.0;
///     assert_eq!(b, MyStruct { val: 4.0f32 });
/// }
/// # }
///
///````
///
/// \[T1,T2...\] where \[T1: MyTrait\] (LhsType) operator (RhsType) = (ResType)
pub fn finish_derive(t: TokenStream) -> TokenStream {
    let s = derive::finish_derive(t);
    // println!("{}", s);
    s.parse().unwrap()
}
