//! Calculator.

use std::{collections::HashMap, f64::consts::PI};

use anyhow::*;
use etrace::*;

use super::syntax::{BinOp, Command, Expression};

/// Calculator's context.
#[derive(Debug, Default, Clone)]
pub struct Context {
    anonymous_counter: usize,
    variables: HashMap<String, f64>,
}

impl Context {
    /// Creates a new context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the current anonymous variable counter.
    pub fn current_counter(&self) -> usize {
        self.anonymous_counter
    }

    /// Calculates the given expression. (We assume the absence of overflow.)
    pub fn calc_expression(&self, expression: &Expression) -> Result<f64> {
        match expression {
            Expression::Num(n) => Ok(*n),

            Expression::Variable(name) => {
                // example variable lookup (you can adjust error message)
                self.variables
                    .get(name)
                    .copied()
                    .ok_or_else(|| anyhow!("undefined variable: {}", name))
            }

            Expression::BinOp { op, lhs, rhs } => {
                // 1) recursively evaluate both sides
                let left_val = self.calc_expression(lhs)?;
                let right_val = self.calc_expression(rhs)?;

                // 2) apply the operator
                let result = match op {
                    BinOp::Add => left_val + right_val,
                    BinOp::Subtract => left_val - right_val,
                    BinOp::Multiply => left_val * right_val,
                    BinOp::Divide => {
                        if right_val == 0.0 {
                            bail!("Error")
                        } else {
                            left_val / right_val
                        }
                    } // optionally check /0
                    BinOp::Power => left_val.powf(right_val), // exponent
                };

                Ok(result)
            }
        }
    }

    /// Calculates the given command. (We assume the absence of overflow.)
    ///
    /// If there is no variable lhs in the command (i.e. `command.variable = None`), its value
    /// should be stored at `$0`, `$1`, `$2`, ... respectively.
    ///
    /// # Example
    ///
    /// After calculating commad `3 + 5` => Context's variables = `{($0,8)}`
    ///
    /// After calculating commad `v = 3 - 2` => Context's variables = `{($0,8),(v,1))}`
    ///
    /// After calculating commad `3 ^ 2` => Context's variables = `{($0,8),(v,1),($1,9)}`
    pub fn calc_command(&mut self, command: &Command) -> Result<(String, f64)> {
        let mut result_str = String::new();
        match &command.variable {
            Some(x) => result_str.push_str(x),
            None => {
                result_str.push_str(&("$".to_owned() + &self.anonymous_counter.to_string()));
                self.anonymous_counter += 1;
            }
        }

        let result_exp = Self::calc_expression(self, &command.expression)?;
        let _unused = self.variables.insert(result_str.clone(), result_exp);

        Ok((result_str, result_exp))
    }
}
