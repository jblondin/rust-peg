#![feature(plugin)]
#![plugin(peg_syntax_ext)]

extern crate regex;

fn remove_underscores(input: &str) -> String {
    let mut s = String::new();
    for c in input.chars().filter(|&c| c != '_') { s.push(c); }
    s
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Float(f64),
    Int(i64)
}
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
}

peg! rgx(r##"

use regex::Regex;
use super::remove_underscores;
use super::{Expression, Literal};

#[pub]
expression -> Expression
    = i:identifier { Expression::Identifier(i) }
    / l:literal { Expression::Literal(l) };

#[pub]
identifier -> String
    = s:@"\p{XID_Start}\p{XID_Continue}*"@ { s.get(0).unwrap().as_str().to_string() }

literal -> Literal
    = f: float { Literal::Float(f) }
    / i: int { Literal::Int(i) };

#[pub]
float -> f64
    = s:@"(?x)
        (?:[_0-9]+\.(?:[_0-9]+(?:[eE][\+-]?[_0-9]+)?)?)
        |
        (?:[_0-9]+[eE][\+-]?[_0-9]+)
    "@ {? remove_underscores(s.get(0).unwrap().as_str()).parse::<f64>().map_err(
            |_| "unable to parse f64") }

#[pub]
int -> i64
    = s:@"(?x)
        0x(?P<hex>[_0-9A-Fa-f]+)   # hexadecimal
        |                          # or
        0o(?P<oct>[_0-8]+)         # octal
        |                          # or
        0b(?P<bin>[_0-1]+)         # binary
        |                          # or
        (?P<dec>[0-9][_0-9]*)      # decimal
    "@ {?
        if let Some(mat) = s.name("dec") {
            i64::from_str_radix(remove_underscores(mat.as_str()).as_str(), 10)
                .map_err(|_| "unable to parse decimal integer")
        } else if let Some(mat) = s.name("hex") {
            i64::from_str_radix(remove_underscores(mat.as_str()).as_str(), 16)
                .map_err(|_| "unable to parse hexadecimal integer")
        } else if let Some(mat) = s.name("bin") {
            i64::from_str_radix(remove_underscores(mat.as_str()).as_str(), 2)
                .map_err(|_| "unable to parse binary integer")
        } else if let Some(mat) = s.name("oct") {
            i64::from_str_radix(remove_underscores(mat.as_str()).as_str(), 8)
                .map_err(|_| "unable to parse octal integer")
        } else {
            Err("unable to parse integer")
        }
    }

"##);

#[test]
fn regex() {
    // println!()
    assert_eq!(rgx::identifier("foo"), Ok("foo".to_string()));
    assert_eq!(rgx::float("5.305"), Ok(5.305));
    assert_eq!(rgx::int("5"), Ok(5));
    assert_eq!(rgx::int("0x0FAF"), Ok(4015));

    assert_eq!(rgx::expression("0x0FAF"), Ok(Expression::Literal(Literal::Int(4015))));
}

