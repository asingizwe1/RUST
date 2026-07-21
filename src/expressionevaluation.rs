enum Operation {
    Add,//different variants
    Sub,
    Mul,
    Div,
    }
enum Expression{
Op{//struct variant
op:Operation,
left:Box<Expression>,//box is a smart pointer for larger / recursive cases
right:Box<Expression>//To evaluate a boxedexpression,use the deref operator (*) to ”unbox” it: eval(*boxed_expr).
},

value(i64)//normal variant
}

fn eval(e: Expression)-> Result<i64, String> {
    match e {
        Expression::Op { op, left, right } => {
        let left = match eval(*left) {
        Ok(v) => v,
        Err(e) => return Err(e),
        };
        let right = match eval(*right) {
        Ok(v) => v,
        Err(e) => return Err(e),
        };
        Ok(match op {
        Operation::Add => left + right,
        Operation::Sub => left- right,
        Operation::Mul => left * right,
        Operation::Div => {
        if right == 0 {
        
       return Err(String::from("division by zero"));
        } else {
        left / right
        }
        }
        })
        }
        Expression::Value(v) => Ok(v),
        }
        }
    
    fn test_value() {
    assert_eq!(eval(Expression::Value(19)), Ok(19));
    }
    fn test_sum() {
    assert_eq!(
    eval(Expression::Op {
    op: Operation::Add,
    left: Box::new(Expression::Value(10)),
    right: Box::new(Expression::Value(20)),
    }),
    Ok(30)
   );
    }
    fn test_recursion() {
    let term1 = Expression::Op {
    op: Operation::Mul,
    left: Box::new(Expression::Value(10)),
    right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
    op: Operation::Mul,
    left: Box::new(Expression::Op {
    op: Operation::Sub,
    left: Box::new(Expression::Value(3)),
    right: Box::new(Expression::Value(4)),
    }),
    right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
    eval(Expression::Op {
    op: Operation::Add,
    left: Box::new(term1),
    right: Box::new(term2),
    }),
    Ok(85)
    );
    }
    fn test_zeros() {
    assert_eq!(
    eval(Expression::Op {
    op: Operation::Add,
    left: Box::new(Expression::Value(0)),
    right: Box::new(Expression::Value(0))
    }),
    Ok(0)
    );
    assert_eq!(
    eval(Expression::Op {
    op: Operation::Mul,
    left: Box::new(Expression::Value(0)),
    right: Box::new(Expression::Value(0))
    }),
    Ok(0)
    );
    assert_eq!(
    eval(Expression::Op {
    op: Operation::Sub,
    left: Box::new(Expression::Value(0)),
    right: Box::new(Expression::Value(0))
    }),
    
   Ok(0)
    );
    }
    fn test_error() {
    assert_eq!(
    
    eval(Expression::Op {
    op: Operation::Div,
    left: Box::new(Expression::Value(99)),
    right: Box::new(Expression::Value(0)),
    }),
    Err(String::from("division by zero"))

);
}
fn main() {
    let expr = Expression::Op {
    op: Operation::Sub,
    left: Box::new(Expression::Value(20)),
    right: Box::new(Expression::Value(10)),
    };
    println!("expr: {expr:?}");
    println!("result: {:?}", eval(expr));
    }