mod checked {
  // Mathematical "errors" we want to catch
  #[derive(Debug)]
  enum MathError {
    DivisionByZero,
    NonPositiveLogarithm,
    NegativeSquareRoot,
  }

  type MathResult = Result<f64, MathError>;

  fn div(x: f64, y: f64) -> MathResult {
    if y == 0.0 {
      // This operation would `fail`, instead let's return the reason of
      // the failure wrapped in `Err`
      Err(MathError::DivisionByZero)
    } else {
      // This operation is valid, return the result wrapped in `Ok`
      Ok(x / y)
    }
  }

  fn sqrt(x: f64) -> MathResult {
    if x < 0.0 {
      Err(MathError::NegativeSquareRoot)
    } else {
      Ok(x.sqrt())
    }
  }

  fn ln(x: f64) -> MathResult {
    if x <= 0.0 {
      Err(MathError::NonPositiveLogarithm)
    } else {
      Ok(x.ln())
    }
  }

  fn op_(x: f64, y: f64) -> MathResult {
    // if `div` "fails", then `DivisionByZero` will be `return`ed
    let ratio = div(x, y)?;

    // if `ln` "fails", then `NonPositiveLogarithm` will be `return`ed
    let ln = ln(ratio)?;

    sqrt(ln)
  }

  // `op(x, y)` === `sqrt(ln(x / y))`
  pub fn op(x: f64, y: f64) {
    match op_(x, y) {
      Err(why) => panic!("{:?}", match why {
        MathError::DivisionByZero => "division by zero",
        MathError::NegativeSquareRoot => "square root of negative number",
        MathError::NonPositiveLogarithm => "logarithm of non-positive number",
      }),
      Ok(value) => println!("{}", value),
    }
  }
}

fn main() {
  checked::op(1.0, 10.0);
}