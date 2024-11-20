pub mod nodes {
    #![allow(unused)]
    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct Program {
        pub statements: Vec<Statement>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Statement {
        Expression(Expression),
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Expression {
        Value(Value),
        Infixed(Operation),
        Prefixed(PrefixedExpression),
        Postfixed(PostfixedExpression),
        Identifier(String),
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Value {
        Number(f64),
        String(String),
        Boolean(bool),
        Nil,
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum PrefixedExpression {
        Negative(Box<Expression>),
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum PostfixedExpression {
        Debug(Box<Expression>),
        Print(Box<Expression>),
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct Operation {
        pub lhs: Box<Expression>,
        pub infix: Operator,
        pub rhs: Box<Expression>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Operator {
        Add,
        Sub,
        Mul,
        Div,
    }
}

use nodes::*;

use super::pest::{Pair as PestPair, Pairs as PestPairs, Rule, PRATT_PARSER};
type Pair<'i> = PestPair<'i, Rule>;
type Pairs<'i> = PestPairs<'i, Rule>;

trait BetterPairs<'a> {
    fn take_(&mut self) -> Pair<'a>;
    // fn peek_(&self) -> Pair<'a>;
}
impl<'a> BetterPairs<'a> for Pairs<'a> {
    fn take_(&mut self) -> Pair<'a> {
        self.next().unwrap()
    }
    // fn peek_(&self) -> Pair<'a> {
    //     self.peek().unwrap()
    // }
}

trait BetterPair<'a> {
    fn childs(self) -> Pairs<'a>;
    fn first_child(self) -> Pair<'a>;
}
impl<'a> BetterPair<'a> for Pair<'a> {
    fn childs(self) -> Pairs<'a> {
        self.into_inner()
    }
    fn first_child(self) -> Pair<'a> {
        self.childs().take_()
    }
}

pub trait ParseMulti {
    fn parse(pairs: Pairs) -> Self;
}

pub trait ParseSingle {
    fn parse(pair: Pair) -> Self;
}

impl ParseMulti for Program {
    fn parse(pairs: Pairs) -> Self {
        let mut statements = vec![];
        for pair in pairs {
            match pair.as_rule() {
                Rule::statement => statements.push(Statement::parse(pair.first_child())),
                Rule::EOI => (),
                _ => unreachable!("{:#?}", pair),
            }
        }
        Program { statements }
    }
}

impl ParseSingle for Statement {
    fn parse(pair: Pair) -> Self {
        match pair.as_rule() {
            Rule::expression => Statement::Expression(Expression::parse(pair.childs())),
            _ => unreachable!(),
        }
    }
}

impl ParseMulti for Expression {
    fn parse(pairs: Pairs) -> Self {
        PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                Rule::value => Expression::Value(Value::parse(primary.first_child())),
                Rule::expression => Expression::parse(primary.childs()),
                Rule::identifier => Expression::Identifier(primary.as_str().to_string()),
                _ => unreachable!("{:#?}", primary),
            })
            .map_infix(|lhs, op, rhs| {
                Expression::Infixed(Operation {
                    lhs: Box::new(lhs),
                    infix: match op.as_rule() {
                        Rule::add => Operator::Add,
                        Rule::sub => Operator::Sub,
                        Rule::mul => Operator::Mul,
                        Rule::div => Operator::Div,
                        _ => unreachable!("{:#?}", op),
                    },
                    rhs: Box::new(rhs),
                })
            })
            .map_prefix(|op, rhs| match op.as_rule() {
                Rule::negate => Expression::Prefixed(PrefixedExpression::Negative(Box::new(rhs))),
                _ => unreachable!("{:#?}", op),
            })
            .map_postfix(|lhs, op| match op.as_rule() {
                Rule::debug => Expression::Postfixed(PostfixedExpression::Debug(Box::new(lhs))),
                Rule::print => Expression::Postfixed(PostfixedExpression::Print(Box::new(lhs))),
                _ => unreachable!("{:#?}", op),
            })
            .parse(pairs)
    }
}

impl ParseSingle for Value {
    fn parse(pair: Pair) -> Self {
        match pair.as_rule() {
            Rule::number => Value::Number(pair.as_str().parse().unwrap()),
            Rule::string => {
                let str = pair.as_str();
                Value::String(str[1..str.len() - 1].to_string())
            }
            Rule::boolean => Value::Boolean(pair.as_str() == "true"),
            Rule::nil => Value::Nil,
            _ => unreachable!("{:#?}", pair),
        }
    }
}
