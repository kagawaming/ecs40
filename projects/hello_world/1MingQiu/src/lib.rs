/*#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let a = Token::Operand(2);
        let b = Token::Operand(3);
        let c = Token::Operator(Operator::Add);
        let x = [a,b,c];
        assert_eq!(Some(5),eval(&x));
    }
}*/


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
        match *token {
            Token::Operator(ref k) => {
                if stack.len()==0{
                    stack.push(1);
                    stack.push(2);
                    break;
                }
                if stack.len()==1{
                    stack.push(1);
                    stack.push(2);
                    break;
                }
                match *k {
                Operator::Add => {
                    let y1=stack.pop().unwrap();
                    let y2=stack.pop().unwrap();
                    let y3=y1+y2;
                    stack.push(y3);

                }
                Operator::Sub => {
                    let y1=stack.pop().unwrap();
                    let y2=stack.pop().unwrap();
                    let y3=y2-y1;
                    stack.push(y3);
                }
                Operator::Mul => {
                    let y1=stack.pop().unwrap();
                    let y2=stack.pop().unwrap();
                    let y3=y1*y2;
                    stack.push(y3);
                    }
            }},

            Token::Operand(x) => {
                stack.push(x);}

        }
    }
    if stack.len()==1{ Some (stack[0])}
        else {None}

}
