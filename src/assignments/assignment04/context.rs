//! Calculator.

use anyhow::*;
use std::collections::HashMap;

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
            Expression::Num(num) => Ok(*num),
            Expression::BinOp { op, lhs, rhs } => {
                let left = self.calc_expression(lhs)?;
                let right = self.calc_expression(rhs)?;
                match op {
                    BinOp::Add => Ok(left + right),
                    BinOp::Subtract => Ok(left - right),
                    BinOp::Divide => {
                        if right == 0 as f64 {
                            Err(Error::msg("divide zero"))
                        } else {
                            Ok(left / right)
                        }
                    }
                    BinOp::Multiply => Ok(left * right),
                    BinOp::Power => Ok(f64::powf(left, right)),
                }
            }
            Expression::Variable(var) => match self.variables.get(var) {
                Some(value) => Ok(*value),
                None => Err(Error::msg("message")),
            },
        }
    }

    /// Calculates the given command. (We assume the absence of overflow.)
    ///
    /// If there is no variable lhs in the command (i.e. `command.variable = None`), its value should be stored at `$0`, `$1`, `$2`, ... respectively.
    ///
    /// # Example
    ///
    /// After calculating commad `3 + 5` => Context's variables = `{($0,8)}`
    ///
    /// After calculating commad `v = 3 - 2` => Context's variables = `{($0,8),(v,1))}`
    ///
    /// After calculating commad `3 ^ 2` => Context's variables = `{($0,8),(v,1),($1,9)}`
    pub fn calc_command(&mut self, command: &Command) -> Result<(String, f64)> {
        let rc = self.calc_expression(&command.expression)?;
        let var = if command.variable.is_none() {
            let string = format!("${}", self.anonymous_counter);
            self.anonymous_counter += 1;
            string
        } else {
            command.variable.clone().unwrap()
        };

        let _ = self
            .variables
            .entry(var.clone())
            .and_modify(|value| *value = rc)
            .or_insert(rc);
        Ok((var, rc))
    }
}
