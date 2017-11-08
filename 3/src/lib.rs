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
                    InfixToken::RightParen => return None,
                    _ =>{},
                }
            }
            }
        }
    }
    let mut stack: Vec<InfixToken> = Vec::new();
    let mut output: Vec<PostfixToken> = Vec::new();
    let mut j=0;
    for i in 0..tokens.len(){
        match tokens[i]{
            InfixToken::LeftParen => {
                stack.push(InfixToken::LeftParen);
                j=j+1;
            }
            InfixToken::RightParen => {
                while let InfixToken::Operator(s) = stack[j]{
                /*    let mut y = stack.pop().unwrap();
                    match y {
                        InfixToken::Operator(s) => {*/
                            output.push(PostfixToken::Operator(s));
                            j=j-1;
                            if j==0 {
                                break;
                            }
                    /*    },
                        InfixToken::LeftParen => {
                            stack.pop();
                            break;
                        },
                        _ => {
                            break;
                        },
                    }*/
                }
            }

            InfixToken::Operator(k) =>{
            /*    if j==0{stack.push(InfixToken::Operator(k));
                j=j+1;
                println!("{:?}",stack);
            }
/*    /*test*/        else {*/
                match k {
                    Operator::Add | Operator::Sub | Operator::Mul | Operator::Div => {
                        while let Some(InfixToken::Operator(s))= stack.pop(){
                        output.push(PostfixToken::Operator(s));
                        j=j-1;
                    }
                        stack.push(InfixToken::Operator(k));
                        j=j+1;
                        println!("{:?}",stack);
                        println!("{:?}",output);
                }
            }
    /*    }*//*test*/*/
                match k {
                    Operator::Add | Operator::Sub => {
                        if j==0 {
                            stack.push(InfixToken::Operator(k));
                            j=j+1;
                            println!("{:?}",stack);
                            continue;
                        }
                        while let Some(InfixToken::Operator(s))= stack.pop() {
                            println!("{:?}",stack);
                            output.push(PostfixToken::Operator(s));
                            println!("{:?}",output);
                            j=j-1;

                        }
                        while let Some(InfixToken::LeftParen)= stack.pop(){}
                        stack.push(InfixToken::Operator(k));
                        j=j+1;

                    }
                    Operator::Mul | Operator::Div => {
                        println!("{:?}",stack);
                        if j==0 {
                            stack.push(InfixToken::Operator(k));
                            j=j+1;
                            continue;
                        }
                        let mut y = stack.pop();

                        while let Some(InfixToken::Operator(s)) = y{
                            match s {
                                Operator::Mul | Operator::Div =>{
                                    output.push(PostfixToken::Operator(s));
                                    j=j-1;

                                },
                                Operator::Add | Operator::Sub =>{
                                    stack.push(InfixToken::Operator(s));
                                    j=j+1;
                                    stack.push(InfixToken::Operator(k));
                                    j=j+1;

                                    break;
                                },
                            }
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
