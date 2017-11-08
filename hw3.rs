#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(Debug, PartialEq)]
pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}
#[derive(Debug, PartialEq)]
pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}
use std::vec::Vec;
use std::ops::BitOr;
pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>>{
    let mut a=0;
    let mut b=0;
    let mut i=0;
    for i in 0..tokens.len(){
        match tokens[i]{
            InfixToken::LeftParen => {
            a+=1;
            if i==tokens.len()-1 {return None;}
            if i!=0{
            match tokens[i-1]{
                InfixToken::Operand(k) => return None,
                InfixToken::RightParen => return None,
                _ =>{},
                }
            }
            }
            InfixToken::RightParen => {
                b+=1;
                if i==0 {return None;}
                if b>a {return None;}
                if i!=0 {
                match tokens[i-1]{
                    InfixToken::Operator(k) => return None,
                    InfixToken::LeftParen => return None,
                    _ =>{},
                }
            }
            }
            InfixToken::Operand(n)=>{
                if i==0 {}
                else{
                match tokens[i-1]{
                    InfixToken::Operand(k) => return None,
                    InfixToken::RightParen => return None,
                    _ =>{},
                }
            }
            }
            InfixToken::Operator(k) => {
                if i==0 {return None}
                if i==tokens.len()-1 {return None}
                else {
                match tokens[i-1]{
                    InfixToken::Operator(n) => return None,
                    InfixToken::LeftParen => return None,
                    _ =>{},
                }
            }
            }
        }
    }
    if a!=b {return None;}
    let mut stack: Vec<InfixToken> = Vec::new();
    let mut output: Vec<PostfixToken> = Vec::new();
    let mut j=0;
    for i in 0..tokens.len(){
        match tokens[i]{
            InfixToken::LeftParen => {
                stack.push(InfixToken::LeftParen);
                j=j+1;
                println!("{:?}",stack);
            }
            InfixToken::RightParen => {
                while let Some(InfixToken::Operator(s))= stack.pop() {
                    output.push(PostfixToken::Operator(s));
                    j=j-1;
                    println!("{:?}",stack);
                    println!("{:?}",j);
                }
                j=j-1;
            }

            InfixToken::Operator(k) =>{
                match k {
                    Operator::Add | Operator::Sub => {
                        if j==0 {
                            stack.push(InfixToken::Operator(k));
                            j=j+1;
                            println!("{:?}",stack);
                            continue;
                        }
                        if stack[j-1]==InfixToken::LeftParen{
                            stack.push(InfixToken::Operator(k));
                            j=j+1;
                            continue;
                        }
                    /*    while let Some(InfixToken::LeftParen)= stack.pop() {
                            stack.push(InfixToken::Operator(k));
                            continue;
                        }*/
                        println!("{:?}",stack);
                        while let Some(InfixToken::Operator(s))= stack.pop() {
                            println!("{:?}",stack);
                            output.push(PostfixToken::Operator(s));
                            println!("{:?}",output);
                            j=j-1;

                        }
                        stack.push(InfixToken::Operator(k));
                        j=j+1;

                    }
                    Operator::Mul | Operator::Div => {
                        println!("{:?}",j);
                        if j==0 {
                            stack.push(InfixToken::Operator(k));
                            j=j+1;
                            println!("{:?}",stack);
                            continue;
                        }
                        else if stack[j-1]==InfixToken::LeftParen{
                            stack.push(InfixToken::Operator(k));
                            j=j+1;
                            continue;
                        }
                        else{
                        while let Some(InfixToken::Operator(s)) = stack.pop(){
                            match s {
                                Operator::Mul | Operator::Div =>{
                                    output.push(PostfixToken::Operator(s));
                                    j=j-1;
                                    println!("{:?}",j);
                                    println!("{:?}",stack);
                                    println!("{:?}",output);
                                },
                                Operator::Add | Operator::Sub =>{
                                    stack.push(InfixToken::Operator(s));
                                    println!("{:?}",j);
                                    println!("{:?}",stack);
                                    println!("{:?}",output);
                                    break;
                                },

                            }
                        }
                        stack.push(InfixToken::Operator(k));
                        j=j+1;
                    }
                    }
                    }
        }
            InfixToken::Operand(k) => output.push(PostfixToken::Operand(k))
    }
}



    while let Some(InfixToken::Operator(s))= stack.pop() {
        output.push(PostfixToken::Operator(s));}
        Some(output)

}
fn main() {
    let w = InfixToken::LeftParen;
    let v = InfixToken::LeftParen;
    let u = InfixToken::LeftParen;
    let y = InfixToken::LeftParen;
    let a = InfixToken::LeftParen;
    let b = InfixToken::Operand(2);
    let c = InfixToken::Operator(Operator::Mul);
    let d = InfixToken::Operand(2);
    let e = InfixToken::RightParen;
    let f = InfixToken::Operator(Operator::Add);
    let g = InfixToken::LeftParen;
    let h = InfixToken::Operand(2);
    let i = InfixToken::Operator(Operator::Mul);
    let j = InfixToken::Operand(3);
    let k = InfixToken::RightParen;
    let l = InfixToken::RightParen;
    let m = InfixToken::Operator(Operator::Mul);
    let n = InfixToken::Operand(3);
    let z = InfixToken::Operator(Operator::Add);
    let q = InfixToken::Operator(Operator::Add);
    let r = InfixToken::Operator(Operator::Add);
    let o = {};
    let p = InfixToken::Operand(2);
    let x = [p,m,j,i,d,f,w,b,q,h,k];
    println!("{:?}",infix_to_postfix(&x))
}
