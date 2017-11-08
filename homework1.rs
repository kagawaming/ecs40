#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let a = 2;
        let b = 3;
        let c = Add;
        let x = [a,b,c];
        assert_eq!(5,eval(&x));
    }
}


pub enum Operator {
    Add,
    Sub,
    Mul,
}

pub enum Token {
    Operator(Operator),
    Operand(isize),
}

use std::vec::Vec;
pub fn eval(tokens: &[Token]) -> Option<isize> {
    let mut stack: Vec<isize> = Vec::new();
    for token in tokens {
        match token {
            Token::Operator(k) => match k {
                Operator::Add => {
                    let y1=stack.pop();
                    let y2=stack.pop();
                    let y3=y1+y2;
                    stack.push(y3);
                }
                Operator::Sub => {
                    let y1=stack.pop();
                    let y2=stack.pop();
                    let y3=y1-y2;
                    stack.push(y3);
                }
                Operator::Mul => {
                    let y1=stack.pop();
                    let y2=stack.pop();
                    let y3=y1*y2;
                    stack.push(y3);
                }
            }

            Token::Operand(x) => {stack.push(x);}
        }
    }
    let y:isize = stack[0];
    let y1 = stack[1];
    if y1==None { None,}
    else if {Some(y),}
}
