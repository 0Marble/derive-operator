use crate::parse::*;
use proc_macro::*;

pub fn derive_operator(t: proc_macro::TokenStream) -> String {
    let it = t.into_iter();
    let (it, maybe_generic_list) = read_list(it, Delimiter::Bracket);
    let (it, generic_list, where_block) = match maybe_generic_list {
        Some(generic_list) => {
            let (it, maybe_where) = read_specific_identifier(it, "where");
            match maybe_where {
                Some(_) => {
                    let (it, should_be_where_block) = read_list(it, Delimiter::Bracket);
                    match should_be_where_block {
                        Some(where_block) => (
                            it,
                            format!("<{generic_list}>"),
                            format!(" where {where_block}"),
                        ),
                        None => panic!("Expected a where block after \"where\""),
                    }
                }
                None => (it, format!("<{generic_list}>",), "".to_owned()),
            }
        }
        None => (it, "".to_owned(), "".to_owned()),
    };

    let (it, should_be_lhs_name_and_type) = read_var(it);
    let (it, should_be_op) = read_operator(it);
    let (it, should_be_rhs_name_and_type) = read_var(it);
    let (it, should_be_equal_sign) = read_specific_punct(it, '=');
    let (it, should_be_res_type) = read_list(it, Delimiter::Parenthesis);
    let (_, should_be_func_body) = read_list(it, Delimiter::Brace);

    let (lhs_name, lhs_type) = should_be_lhs_name_and_type.expect("Expected (lhs_name: lhs_type)");
    let (rhs_name, rhs_type) = should_be_rhs_name_and_type.expect("Expected (rhs_name: rhs_type)");
    let op = should_be_op.expect("Expected +-*/&^|");
    let _ = should_be_equal_sign.expect("Expected =");
    let res_type = should_be_res_type.expect("Expected res type");
    let func_body = should_be_func_body.expect("Expected func body");

    let trait_name = op.trait_name();
    let op_name = op.op_name();

    format!(
        "
impl{generic_list} {trait_name}<{rhs_type}> for {lhs_type} {where_block} {{
    type Output = {res_type};
    fn {op_name}(self, rhs: {rhs_type}) -> Self::Output {{
        (|{lhs_name}: {lhs_type}, {rhs_name}: {rhs_type}| {{
            {func_body}
        }})(self,rhs)
    }} 
}}"
    )
}

pub fn finish_derive(t: TokenStream) -> String {
    let it = t.into_iter();
    let (it, maybe_generic_list) = read_list(it, Delimiter::Bracket);
    let (it, generic_list, where_block) = match maybe_generic_list {
        Some(generic_list) => {
            let (it, maybe_where) = read_specific_identifier(it, "where");
            match maybe_where {
                Some(_) => {
                    let (it, should_be_where_block) = read_list(it, Delimiter::Bracket);
                    match should_be_where_block {
                        Some(where_block) => (
                            it,
                            format!("<{generic_list}>"),
                            format!(" where {where_block}"),
                        ),
                        None => panic!("Expected a where block after \"where\""),
                    }
                }
                None => (it, format!("<{generic_list}>",), "".to_owned()),
            }
        }
        None => (it, "".to_owned(), "".to_owned()),
    };

    let (it, should_be_lhs_type) = read_list(it, Delimiter::Parenthesis);
    let (it, should_be_op) = read_operator(it);
    let (it, should_be_rhs_type) = read_list(it, Delimiter::Parenthesis);
    let (it, should_be_equal_sign) = read_specific_punct(it, '=');
    let (_, should_be_res_type) = read_list(it, Delimiter::Parenthesis);

    let lhs_type = should_be_lhs_type.expect("Expected (lhs_type)");
    let rhs_type = should_be_rhs_type.expect("Expected (rhs_type)");
    let op = should_be_op.expect("Expected +-*/&^|");
    let _ = should_be_equal_sign.expect("Expected =");
    let res_type = should_be_res_type.expect("Expected res type");

    let trait_name = op.trait_name();
    let op_name = op.op_name();
    let trait_assign_name = op.trait_assign_name();
    let op_assign_name = op.op_assign_name();

    format!(
        "
impl{generic_list} {trait_name}<{rhs_type}> for {lhs_type} {where_block} {{
    type Output = {res_type};
    fn {op_name}(self, rhs: {rhs_type}) -> Self::Output {{
        {trait_name}::{op_name}(&self, &rhs)
    }}
}}

impl{generic_list} {trait_name}<&{rhs_type}> for {lhs_type} {where_block} {{
    type Output = {res_type};
    fn {op_name}(self, rhs: &{rhs_type}) -> Self::Output {{
        {trait_name}::{op_name}(&self, rhs)
    }}
}}

impl{generic_list} {trait_name}<{rhs_type}> for &{lhs_type} {where_block} {{
    type Output = {res_type};
    fn {op_name}(self, rhs: {rhs_type}) -> Self::Output {{
        {trait_name}::{op_name}(self, &rhs)
    }}
}}

impl{generic_list} {trait_assign_name}<{rhs_type}> for {lhs_type} {where_block} {{
    fn {op_assign_name}(&mut self, rhs: {rhs_type}) {{
        *self = {trait_name}::{op_name}(&*self, &rhs);
    }}
}}

impl{generic_list} {trait_assign_name}<&{rhs_type}> for {lhs_type} {where_block} {{
    fn {op_assign_name}(&mut self, rhs: &{rhs_type}) {{
        *self = {trait_name}::{op_name}(&*self, rhs);
    }}
}}
"
    )
}
