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
        Set(SetStatement),
        Define(DefineStatement),
        DefineAndSet(DefineAndSetStatement),
        If(IfStatement),
        While(WhileStatement),
        For(ForStatement),
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct ForStatement {
        pub initializer: Box<Statement>,
        pub condition: Expression,
        pub increment: Box<Statement>,
        pub statement: Box<Statement>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct IfStatement {
        pub conditionals: Vec<(Expression, Statement)>,
        pub otherwise: Option<Box<Statement>>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct WhileStatement {
        pub expression: Expression,
        pub statement: Box<Statement>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct SetStatement {
        pub identifier: String,
        pub expression: Expression,
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct DefineStatement {
        pub identifier: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct DefineAndSetStatement {
        pub identifier: String,
        pub expression: Expression,
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Expression {
        Value(Value),
        Infixed(Operation),
        Prefixed(PrefixedExpression),
        Postfixed(PostfixedExpression),
        Identifier(String),
        Block(Block),
        Map(MapExpression),
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct MapExpression {
        pub input: Box<Expression>,
        pub map: Vec<(Expression, Expression)>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct Block {
        pub statements: Vec<Statement>,
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
        Eq,
        Neq,
        Gt,
        Lt,
        Gte,
        Lte,
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

impl ParseMulti for Block {
    fn parse(pairs: Pairs) -> Self {
        let mut statements = vec![];
        for pair in pairs {
            match pair.as_rule() {
                Rule::statement => statements.push(Statement::parse(pair.first_child())),
                _ => unreachable!("{:#?}", pair),
            }
        }
        Block { statements }
    }
}

impl ParseSingle for Statement {
    fn parse(pair: Pair) -> Self {
        match pair.as_rule() {
            Rule::expression => Statement::Expression(Expression::parse(pair.childs())),
            Rule::set_statement => Statement::Set(SetStatement::parse(pair.childs())),
            Rule::define_statement => Statement::Define(DefineStatement::parse(pair.first_child())),
            Rule::define_and_set_statement => {
                Statement::DefineAndSet(DefineAndSetStatement::parse(pair.childs()))
            }
            Rule::if_statement => Statement::If(IfStatement::parse(pair.childs())),
            Rule::while_statement => Statement::While(WhileStatement::parse(pair.childs())),
            Rule::for_statement => Statement::For(ForStatement::parse(pair.childs())),
            _ => unreachable!("{:#?}", pair),
        }
    }
}

impl ParseMulti for ForStatement {
    fn parse(mut pairs: Pairs) -> Self {
        let initializer = Statement::parse(pairs.take_().first_child());
        let condition = Expression::parse(pairs.take_().childs());
        let increment = Statement::parse(pairs.take_().first_child());
        let statement = Statement::parse(pairs.take_().first_child());

        ForStatement {
            initializer: Box::new(initializer),
            condition,
            increment: Box::new(increment),
            statement: Box::new(statement),
        }
    }
}

impl ParseMulti for WhileStatement {
    fn parse(mut pairs: Pairs) -> Self {
        let expression = Expression::parse(pairs.take_().childs());
        let statement = Statement::parse(pairs.take_().first_child());

        WhileStatement {
            expression,
            statement: Box::new(statement),
        }
    }
}

impl ParseMulti for IfStatement {
    fn parse(pairs: Pairs) -> Self {
        let mut iter = pairs.into_iter();
        let mut conditionals = vec![];
        let mut otherwise = None;

        while let Some(pair) = iter.next() {
            match pair.as_rule() {
                Rule::expression => {
                    let expression = Expression::parse(pair.childs());
                    let statement = Statement::parse(iter.next().unwrap().first_child());
                    conditionals.push((expression, statement));
                }
                Rule::statement => {
                    otherwise = Some(Box::new(Statement::parse(pair.first_child())));
                }
                _ => unreachable!("{:#?}", pair),
            }
        }

        Self {
            conditionals,
            otherwise,
        }
    }
}

impl ParseSingle for DefineStatement {
    fn parse(pair: Pair) -> Self {
        let identifier = pair.as_str().to_string();
        DefineStatement { identifier }
    }
}

impl ParseMulti for DefineAndSetStatement {
    fn parse(mut pairs: Pairs) -> Self {
        let identifier = pairs.take_().as_str().to_string();
        let expression = pairs.take_();
        DefineAndSetStatement {
            identifier,
            expression: Expression::parse(expression.childs()),
        }
    }
}

impl ParseMulti for SetStatement {
    fn parse(mut pairs: Pairs) -> Self {
        let identifier = pairs.take_().as_str().to_string();
        let expression = pairs.take_();
        SetStatement {
            identifier,
            expression: Expression::parse(expression.childs()),
        }
    }
}

impl ParseMulti for MapExpression {
    fn parse(pairs: Pairs) -> Self {
        let mut iter = pairs.into_iter();
        let mut map = vec![];
        let input = Expression::parse(iter.take_().childs());

        while let Some(pair) = iter.next() {
            let case = Expression::parse(pair.childs());
            let value = Expression::parse(iter.take_().childs());
            map.push((case, value));
        }

        MapExpression {
            input: Box::new(input),
            map,
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
                Rule::block => Expression::Block(Block::parse(primary.childs())),
                Rule::map => Expression::Map(MapExpression::parse(primary.childs())),
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
                        Rule::eq => Operator::Eq,
                        Rule::neq => Operator::Neq,
                        Rule::gt => Operator::Gt,
                        Rule::lt => Operator::Lt,
                        Rule::gte => Operator::Gte,
                        Rule::lte => Operator::Lte,
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
