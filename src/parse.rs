use proc_macro::*;

pub fn read_var<I: Iterator<Item = TokenTree> + Clone>(it: I) -> (I, Option<(String, String)>) {
    let mut iter = it.clone();
    match iter.next() {
        Some(item) => match item {
            TokenTree::Group(g) if g.delimiter() == Delimiter::Parenthesis => {
                let mut group_iter = g.stream().into_iter();
                match group_iter.next() {
                    Some(item) => match item {
                        TokenTree::Ident(i) => {
                            let _start = i.span();
                            let (after_colon, should_be_colon) =
                                read_specific_punct(group_iter, ':');
                            let _end = match should_be_colon {
                                Some(p) => p.span(),
                                None => return (it, None),
                            };

                            let s = after_colon.fold("".to_string(), |acc, item| match item {
                                TokenTree::Punct(p) if p.as_char() == '\'' => acc + &p.to_string(),
                                _ => acc + &item.to_string() + " ",
                            });
                            (iter, Some((i.to_string(), s)))
                        }

                        _ => (it, None),
                    },
                    None => (it, None),
                }
            }
            _ => (it, None),
        },
        None => (it, None),
    }
}

pub fn read_specific_identifier<I: Iterator<Item = TokenTree> + Clone>(
    it: I,
    identifier: &str,
) -> (I, Option<()>) {
    let mut iter = it.clone();
    match iter.next() {
        Some(item) => match item {
            TokenTree::Ident(i) if i.to_string() == identifier => (iter, Some(())),
            _ => (it, None),
        },
        None => (it, None),
    }
}

pub fn read_specific_punct<I: Iterator<Item = TokenTree> + Clone>(
    it: I,
    punct: char,
) -> (I, Option<Punct>) {
    let mut iter = it.clone();
    match iter.next() {
        Some(item) => match item {
            TokenTree::Punct(p) if p.as_char() == punct => (iter, Some(p)),
            _ => (it, None),
        },
        None => (it, None),
    }
}

pub fn read_list<I: Iterator<Item = TokenTree> + Clone>(
    it: I,
    delimeter: Delimiter,
) -> (I, Option<String>) {
    let mut iter = it.clone();
    match iter.next() {
        Some(item) => match item {
            TokenTree::Group(group) if group.delimiter() == delimeter => {
                let mut s = group.to_string()[1..].to_owned();
                s.pop();
                (iter, Some(s))
            }
            _ => (it, None),
        },
        None => (it, None),
    }
}

pub fn read_punctuation<I: Iterator<Item = TokenTree> + Clone>(it: I) -> (I, Option<char>) {
    let mut start = it.clone();
    match start.next() {
        Some(item) => match item {
            TokenTree::Punct(p) => (start, Some(p.as_char())),
            _ => (it, None),
        },
        None => (it, None),
    }
}

pub fn read_operator<I: Iterator<Item = TokenTree> + Clone>(it: I) -> (I, Option<Operator>) {
    let (next, maybe_char) = read_punctuation(it.clone());
    match maybe_char {
        Some(c) => {
            let op = match Operator::from_char(c) {
                Some(op) => op,
                None => return (it, None),
            };
            (next, Some(op))
        }
        None => (it, None),
    }
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    BitAnd,
    BitOr,
    BitXor,
}

impl Operator {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Self::Add),
            '-' => Some(Self::Sub),
            '*' => Some(Self::Mul),
            '/' => Some(Self::Div),
            '&' => Some(Self::BitAnd),
            '^' => Some(Self::BitXor),
            '|' => Some(Self::BitOr),
            _ => None,
        }
    }

    pub fn trait_name(&self) -> String {
        match self {
            Operator::Add => "std::ops::Add".to_owned(),
            Operator::Sub => "std::ops::Sub".to_owned(),
            Operator::Mul => "std::ops::Mul".to_owned(),
            Operator::Div => "std::ops::Div".to_owned(),
            Operator::BitAnd => "std::ops::BitAnd".to_owned(),
            Operator::BitOr => "std::ops::BitOr".to_owned(),
            Operator::BitXor => "std::ops::BitXor".to_owned(),
        }
    }

    pub fn op_name(&self) -> String {
        match self {
            Operator::Add => "add".to_owned(),
            Operator::Sub => "sub".to_owned(),
            Operator::Mul => "mul".to_owned(),
            Operator::Div => "div".to_owned(),
            Operator::BitAnd => "bitand".to_owned(),
            Operator::BitOr => "bitor".to_owned(),
            Operator::BitXor => "bitxor".to_owned(),
        }
    }
    pub fn trait_assign_name(&self) -> String {
        match self {
            Operator::Add => "std::ops::AddAssign".to_owned(),
            Operator::Sub => "std::ops::SubAssign".to_owned(),
            Operator::Mul => "std::ops::MulAssign".to_owned(),
            Operator::Div => "std::ops::DivAssign".to_owned(),
            Operator::BitAnd => "std::ops::BitAndAssign".to_owned(),
            Operator::BitOr => "std::ops::BitOrAssign".to_owned(),
            Operator::BitXor => "std::ops::BitXorAssign".to_owned(),
        }
    }
    pub fn op_assign_name(&self) -> String {
        match self {
            Operator::Add => "add_assign".to_owned(),
            Operator::Sub => "sub_assign".to_owned(),
            Operator::Mul => "mul_assign".to_owned(),
            Operator::Div => "div_assign".to_owned(),
            Operator::BitAnd => "bitand_assign".to_owned(),
            Operator::BitOr => "bitor_assign".to_owned(),
            Operator::BitXor => "bitxor_assign".to_owned(),
        }
    }
}
