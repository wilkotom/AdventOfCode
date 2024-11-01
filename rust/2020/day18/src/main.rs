use std::collections::VecDeque;

#[derive(PartialEq,Eq,Clone,Debug)]
enum ExpressionEntry {
    OpeningParen,
    Add,
    Multiply,
    Value(i128)
}


fn main() {
    let expressions = read_expressions("./input.txt".to_string());
    let enums = parse_part_1(&expressions);
    println!("Part 1: {}", evaluate_rpn(&enums));
    let enums = parse_part_2(&expressions);
    println!("Part 2: {}", evaluate_rpn(&enums));

}


fn read_expressions(filename: String) -> Vec<Vec<char>> {
    let file = std::fs::read_to_string(filename).unwrap_or("".to_string());
    let mut results: Vec<Vec<char>> = Vec::new();
    for line in file.split("\n") {
        results.push(line.chars().filter(|&x| x != ' ').collect());
    }
    results
}

fn parse_part_2(expressions: &Vec<Vec<char>>) -> Vec<Vec<ExpressionEntry>> {
    let mut parsed_expressions: Vec<Vec<ExpressionEntry>> = Vec::new();
    for expression in expressions {
        let mut rpn: Vec<ExpressionEntry>= Vec::new();
        let mut oper_stack: VecDeque<ExpressionEntry>= VecDeque::new();
        for c in expression {
            match c {
                '(' => oper_stack.push_back(ExpressionEntry::OpeningParen),
                '+' => oper_stack.push_back(ExpressionEntry::Add),

                '*' =>{
                    if oper_stack.len() > 0 {
                        let mut stack_top = oper_stack.pop_back().unwrap_or(ExpressionEntry::OpeningParen);
                        while stack_top == ExpressionEntry::Add {
                            rpn.push(ExpressionEntry::Add);
                            stack_top = oper_stack.pop_back().unwrap_or(ExpressionEntry::OpeningParen);
                        }
                        oper_stack.push_back(stack_top);
                    } 
                    oper_stack.push_back(ExpressionEntry::Multiply);
                },
                ')' => {
                    let mut oper = oper_stack.pop_back().unwrap_or(ExpressionEntry::OpeningParen);
                    while oper != ExpressionEntry::OpeningParen {
                        rpn.push(oper); 
                        oper = oper_stack.pop_back().unwrap_or(ExpressionEntry::OpeningParen);
                    }
                    match oper_stack.pop_back() {
                        Some(ExpressionEntry::Add) => rpn.push(ExpressionEntry::Add),
                        Some(ExpressionEntry::OpeningParen) => oper_stack.push_back(ExpressionEntry::OpeningParen),
                        Some(x) => oper_stack.push_back(x),
                        _ => {}
                    }
                    
                }
                _ => {
                    c.to_string().parse::<i64>().unwrap_or(0);
                    rpn.push(ExpressionEntry::Value(c.to_string().parse::<i128>().unwrap_or(0)));
                    match oper_stack.pop_back() {
                        Some(ExpressionEntry::Add) => rpn.push(ExpressionEntry::Add),
                        Some(o) => oper_stack.push_back(o),
                        _ => {}
                    }
            
                }

            }
        }
        rpn.extend(oper_stack);
        parsed_expressions.push(rpn);
    }
    println!("{:?}", parsed_expressions);
    parsed_expressions
}


fn parse_part_1(expressions: &Vec<Vec<char>>) -> Vec<Vec<ExpressionEntry>> {
    let mut parsed_expressions: Vec<Vec<ExpressionEntry>> = Vec::new();
    for expression in expressions {
        let mut rpn: Vec<ExpressionEntry>= Vec::new();
        let mut oper_stack: VecDeque<ExpressionEntry>= VecDeque::new();
        for c in expression {
            match c {
                '(' => oper_stack.push_back(ExpressionEntry::OpeningParen),
                '+' => oper_stack.push_back(ExpressionEntry::Add),
                '*' => oper_stack.push_back(ExpressionEntry::Multiply),
                ')' => {
                    let oper = oper_stack.pop_back().unwrap_or(ExpressionEntry::OpeningParen);
                    if oper == ExpressionEntry::OpeningParen {
                        match oper_stack.pop_back() {
                            Some(ExpressionEntry::OpeningParen) => oper_stack.push_back(ExpressionEntry::OpeningParen),
                            Some(x) => rpn.push(x),
                            _ => {}
                        }
                    }
                }
                _ => {
                    c.to_string().parse::<i64>().unwrap_or(0);
                    rpn.push(ExpressionEntry::Value(c.to_string().parse::<i128>().unwrap_or(0)));
                    match oper_stack.pop_back() {
                        Some(ExpressionEntry::OpeningParen) => oper_stack.push_back(ExpressionEntry::OpeningParen),
                        Some(o) => rpn.push(o),
                        _ => {}
                    }
            
                }

            }
        }
        rpn.extend(oper_stack);
        parsed_expressions.push(rpn);
    }
    parsed_expressions
}

fn evaluate_rpn(expressions: &Vec<Vec<ExpressionEntry>>) -> i128 {
    let mut total: i128 = 0;
    for rpn in expressions {
        let mut stack: VecDeque<i128>= VecDeque::new();
        for c in rpn.iter() {
            match c {
                ExpressionEntry::Value(n) => stack.push_back(*n),
                ExpressionEntry::Multiply => {
                            let n1 = stack.pop_back().unwrap();
                            let n2 = stack.pop_back().unwrap();
                            stack.push_back(n1*n2);
                            },
                ExpressionEntry::Add => {
                            let n1 = stack.pop_back().unwrap();
                            let n2 = stack.pop_back().unwrap();
                            stack.push_back(n1+n2);
                            },
                _ => {}
            }
        }    
        total += stack[0];
    }
    total
}

