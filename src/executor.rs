//
// executor.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//

use std::fmt;
use parser::*;

#[derive(Debug)]
pub enum Value {
  Number(u32),
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    match self {
      &Value::Number(n) => write!(f, "{}", n),
    }
  }
}

impl fmt::Binary for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    match self {
      &Value::Number(n) => write!(f, "{:b}", n),
    }
  }
}

impl fmt::Octal for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    match self {
      &Value::Number(n) => write!(f, "{:o}", n),
    }
  }
}

impl fmt::LowerHex for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    match self {
      &Value::Number(n) => write!(f, "{:x}", n),
    }
  }
}

pub fn execute(expr: &Expr) -> Result<Value, &'static str> {
  match expr {
    &Expr::Number(i) => {
      Ok(Value::Number(i))
    },
    &Expr::Add(ref l, ref r) => {
      let lhs = execute(&*l).ok().unwrap();
      let rhs = execute(&*r).ok().unwrap();

      match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => {
          Ok(Value::Number(lhs + rhs))
        }
      }
    },
    &Expr::Sub(ref l, ref r) => {
      let lhs = execute(&*l).ok().unwrap();
      let rhs = execute(&*r).ok().unwrap();

      match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => {
          Ok(Value::Number(lhs - rhs))
        }
      }
    },
    &Expr::Mul(ref l, ref r) => {
      let lhs = execute(&*l).ok().unwrap();
      let rhs = execute(&*r).ok().unwrap();

      match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => {
          Ok(Value::Number(lhs * rhs))
        }
      }
    },
    &Expr::Div(ref l, ref r) => {
      let lhs = execute(&*l).ok().unwrap();
      let rhs = execute(&*r).ok().unwrap();

      match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => {
          if rhs == 0 {
            return Err("Division by zero!");
          }

          Ok(Value::Number(lhs / rhs))
        }
      }
    },
  }
}

/*
 * vi: ts=2:sw=2 expandtab
 */
