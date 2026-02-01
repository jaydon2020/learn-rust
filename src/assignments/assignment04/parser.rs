#![allow(deprecated)]

//! Parser.

use std::default;

use anyhow::{bail, Result};
use etrace::*;
use lazy_static::*;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use pest::Parser;

use super::syntax::*;

#[allow(missing_docs)]
#[allow(missing_debug_implementations)]
mod inner {
    use pest_derive::*;

    #[derive(Parser)]
    #[grammar = "assignments/assignment04/syntax.pest"]
    pub(crate) struct SyntaxParser;
}

use inner::*;

// Helper: build a precedence climber once
fn climber() -> PrecClimber<Rule> {
    PrecClimber::new(vec![
        // lowest precedence
        Operator::new(Rule::add, Assoc::Left) | Operator::new(Rule::subtract, Assoc::Left),
        Operator::new(Rule::multiply, Assoc::Left) | Operator::new(Rule::divide, Assoc::Left),
        // highest precedence
        Operator::new(Rule::power, Assoc::Right),
    ])
}

// Map pest rule -> BinOp
fn rule_to_binop(rule: Rule) -> Result<BinOp> {
    Ok(match rule {
        Rule::add => BinOp::Add,
        Rule::subtract => BinOp::Subtract,
        Rule::multiply => BinOp::Multiply,
        Rule::divide => BinOp::Divide,
        Rule::power => BinOp::Power,
        _ => bail!("unexpected operator rule: {:?}", rule),
    })
}

// Parse an expr Pair into Expression AST
fn parse_expr_pair(pair: Pair<'_, Rule>) -> Result<Expression> {
    if pair.as_rule() != Rule::expr {
        bail!("expected expr, got {:?}", pair.as_rule());
    }

    let climber = climber();

    // climb over expr's inner pairs: atom op atom op atom ...
    let expr = climber.climb(
        pair.into_inner(),
        |p: Pair<'_, Rule>| parse_primary(p),
        |lhs: Result<Expression>, op: Pair<'_, Rule>, rhs: Result<Expression>| {
            let lhs = lhs?;
            let rhs = rhs?;
            let op = rule_to_binop(op.as_rule())?;

            Ok(Expression::BinOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            })
        },
    )?;

    Ok(expr)
}

// Parse "primary"/atom: num | var | parenthesized expr (which appears as Rule::expr)
fn parse_primary(pair: Pair<'_, Rule>) -> Result<Expression> {
    match pair.as_rule() {
        Rule::num => {
            let n: f64 = pair.as_str().parse()?;
            Ok(Expression::Num(n))
        }
        Rule::var => Ok(Expression::Variable(pair.as_str().to_string())),
        Rule::expr => parse_expr_pair(pair),
        other => bail!("unexpected primary: {:?}", other),
    }
}

/// Parses command.
///
/// ## Operator Associativty
///
/// For associativity of each operator, please follow [here](https://docs.rs/pest/latest/pest/prec_climber/struct.PrecClimber.html#examples).
///
/// e.g. `1+2+3` should be parsed into `(1+2)+3`, not `1+(2+3)` because the associativity of
/// plus("add" in our hw) operator is `Left`.

pub fn parse_command<'i>(line: &'i str) -> Result<Command> {
    let mut parsed = SyntaxParser::parse(Rule::command, line)?;
    println!("pared {:?}", parsed);

    let first = parsed
        .next()
        .ok_or_else(|| anyhow::anyhow!("empty parse result"))?;

    // Collect pairs whether `command` is silent or not
    let mut top_pairs: Vec<Pair<'i, Rule>> = if first.as_rule() == Rule::command {
        first.into_inner().collect()
    } else {
        let mut v = vec![first];
        v.extend(parsed);
        v
    };

    // Filter out SOI/EOI if they appear
    top_pairs.retain(|p| !matches!(p.as_rule(), Rule::EOI));

    match top_pairs.as_slice() {
        // assignment: var expr
        [var_pair, expr_pair]
            if var_pair.as_rule() == Rule::var && expr_pair.as_rule() == Rule::expr =>
        {
            Ok(Command {
                variable: Some(var_pair.as_str().to_string()),
                expression: parse_expr_pair(expr_pair.clone())?,
            })
        }

        // expression only: expr
        [expr_pair] if expr_pair.as_rule() == Rule::expr => Ok(Command {
            variable: None,
            expression: parse_expr_pair(expr_pair.clone())?,
        }),

        other => bail!(
            "unexpected command shape: {:?}",
            other.iter().map(|p| p.as_rule()).collect::<Vec<_>>()
        ),
    }
}
