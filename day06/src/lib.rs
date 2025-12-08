pub fn solve(input: &str) -> (u128, u128) {
    let equations = parse_input(&input);
    let s1 = solve_part_1(&equations);
    let s2 = 0;
    (s1, s2)
}

pub fn solve_part_1(equations: &Vec<Equation>) -> u128 {
    let mut total = 0;

    for equation in equations {
        let result: u128 = match equation.operator {
            Operator::Add => equation.operands.iter().sum(),
            Operator::Multiply => equation.operands.iter().product(),
        };
        total += result
    }
    total
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
    Number(u128),
    Operator(Operator),
}

#[derive(Debug, PartialEq)]
struct Equation {
    operands: Vec<u128>,
    operator: Operator,
}

fn parse_line_to_tokens(line_str: &str) -> Vec<Token> {
    line_str
        .split_whitespace()
        .map(|tok| match tok {
            "+" => Token::Operator(Operator::Add),
            "*" => Token::Operator(Operator::Multiply),
            _ => {
                let n = tok.parse::<u128>().expect("Invalid integer token");
                Token::Number(n)
            }
        })
        .collect()
}

fn invert_grid(grid: Vec<Vec<Token>>) -> Vec<Vec<Token>> {
    let row_len = grid[0].len();
    (0..row_len)
        .map(|i| grid.iter().map(|row| row[i].clone()).collect())
        .collect()
}

fn parse_tokens_to_equation(tokens: Vec<Token>) -> Equation {
    let mut operands = Vec::new();
    for operand in &tokens[..tokens.len() - 1] {
        let n = match *operand {
            Token::Number(n) => n,
            _ => unreachable!("Expected a Number token"),
        };
        operands.push(n);
    }
    let operator = match tokens[tokens.len() - 1] {
        Token::Operator(o) => o,
        _ => unreachable!("Expected an Operator token"),
    };
        
    Equation { operands, operator }
}

fn parse_input(input: &str) -> Vec<Equation> {
    let mut grid = Vec::new();
    let mut equations = Vec::new();
    for (_line_num, line) in input.lines().enumerate() {
        let line_tokens = parse_line_to_tokens(&line);
        grid.push(line_tokens);
    }
    let grid = invert_grid(grid);
    for eq_tokens in grid  {
        let equation = parse_tokens_to_equation(eq_tokens); 
        equations.push(equation);
    }
    equations
}


#[test]
fn test_line_to_tokens() {
    let input = " 2  3   5 + ";
    let expected = vec![Token::Number(2), Token::Number(3), Token::Number(5), Token::Operator(Operator::Add)];
    let result = parse_line_to_tokens(input);
    assert_eq!(result, expected)
}

#[test]
fn test_invert_grid() {
    let input = vec![
        vec![Token::Number(1), Token::Number(2), Token::Number(3)],
        vec![Token::Number(4), Token::Number(5), Token::Number(6)],
    ];
    let expected = vec![
        vec![Token::Number(1), Token::Number(4)],
        vec![Token::Number(2), Token::Number(5)],
        vec![Token::Number(3), Token::Number(6)]
    ];

    let inverted = invert_grid(input);
    assert_eq!(inverted, expected);
}

#[test]
fn test_parse_tokens_to_equation() {
    let input = vec![Token::Number(1), Token::Number(2), Token::Number(3), Token::Operator(Operator::Add)];
    let expected = Equation {operands: vec![1, 2, 3], operator: Operator::Add};
    let result = parse_tokens_to_equation(input);
    assert_eq!(result, expected);
}