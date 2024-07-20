use crate::iterators::Tokenizer;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

// impl<'a> Parser<'a> {
//     pub fn from(string: &'a String) -> Self {
//         Self {
//             tokenizer: Tokenizer::from(&string)
//         }
//     }

//     fn parse_number(&mut self) {},
//     fn parse_
// }