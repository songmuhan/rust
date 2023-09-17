#![allow(deprecated)]

//! Parser.

use crate::assignments::assignment04::parser::inner::{Rule, SyntaxParser};
use lazy_static::lazy_static;
use pest::{
    pratt_parser::{Assoc, Op, PrattParser},
    Parser,
};

use super::syntax::*;
use anyhow::Result;
use pest::iterators::Pairs;

#[allow(missing_docs)]
#[allow(missing_debug_implementations)]
mod inner {
    use pest_derive::*;

    #[derive(Parser)]
    #[grammar = "assignments/assignment04/syntax.pest"]
    pub(crate) struct SyntaxParser;
}

lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::infix(power, Right))
    };
}

/// Parses command.
///
/// ## Operator Associativty
///
/// For associativity of each operator, please follow [here](https://docs.rs/pest/latest/pest/prec_climber/struct.PrecClimber.html#examples).
///
/// e.g. `1+2+3` should be parsed into `(1+2)+3`, not `1+(2+3)` because the associativity of plus("add" in our hw) operator is `Left`.
pub fn parse_command(line: &str) -> Result<Command> {
    fn parse_expr(pairs: Pairs<'_, Rule>) -> Expression {
        println!("parse expr {:?}", pairs);
        PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                Rule::num => Expression::Num(primary.as_str().parse::<f64>().unwrap()),
                Rule::expr => parse_expr(primary.into_inner()),
                Rule::var => Expression::Variable(primary.as_str().to_string()),
                rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
            })
            .map_infix(|lhs, op, rhs| {
                let op = match op.as_rule() {
                    Rule::add => BinOp::Add,
                    Rule::subtract => BinOp::Subtract,
                    Rule::multiply => BinOp::Multiply,
                    Rule::divide => BinOp::Divide,
                    Rule::power => BinOp::Power,
                    rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
                };
                Expression::BinOp {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                }
            })
            .parse(pairs)
    }
    let mut pairs = SyntaxParser::parse(Rule::command, line)?;
    match pairs.peek().unwrap().as_rule() {
        Rule::var => {
            let pair = pairs.peek().unwrap();
            let variable = Some(pair.as_span().as_str().to_string());
            let _ = pairs.next();
            let expression = parse_expr(pairs.next().unwrap().into_inner());
            Ok(Command {
                variable,
                expression,
            })
        }
        Rule::expr => {
            let expression = parse_expr(pairs.next().unwrap().into_inner());
            Ok(Command {
                variable: None,
                expression,
            })
        }

        _ => unreachable!("here"),
    }
}
