use crate::parser::*;

pub fn interpret(expr: &Expr) -> Result<i64, String> {
    match *expr {
        Expr::Literal(n) => Ok(n),
        Expr::Unary(op, ref expr) => handle_unop(op, interpret(expr)?),
        Expr::Binary(ref left, op, ref right) => {
            let v1 = interpret(left)?;
            let v2 = interpret(right)?;
            handle_binop(op, v1, v2)
        }
    }
}

fn handle_unop(op: UnaryOp, left: i64) -> Result<i64, String> {
    match op {
        UnaryOp::Neg => Ok(left * -1),
    }
}

fn handle_binop(op: BinaryOp, left: i64, right: i64) -> Result<i64, String> {
    match op {
        BinaryOp::Mul => Ok(left * right),
        BinaryOp::Div => Ok(left / right),
        BinaryOp::Add => Ok(left + right),
        BinaryOp::Sub => Ok(left - right),
        BinaryOp::Pow => {
            if right >= 0 {
                Ok(left.pow(right as u32))
            } else {
                Err("cannot raise number to negative power".to_owned())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_literal() {
        let result = interpret(&Expr::Literal(2));
        assert_eq!(result, Ok(2))
    }

    #[test]
    fn eval_simple() {
        let ast = parse_and_lex(&"3 * (2 + -4) ^ 4 / 2".to_owned()).unwrap();
        let result = interpret(&ast);
        assert_eq!(result, Ok(24))
    }
}
