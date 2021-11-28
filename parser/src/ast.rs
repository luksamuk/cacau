use crate::{CacauParser, Rule};
use pest::prec_climber as pcl;
use pest::prec_climber::PrecClimber;
use pest_consume::{match_nodes, Error};

use lazy_static::lazy_static;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use pcl::Assoc::*;
        use pcl::Operator;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(Add, Left) | Operator::new(Subtract, Left),
            Operator::new(Multiply, Left)
                | Operator::new(Divide, Left)
                | Operator::new(Modulo, Left),
            Operator::new(Power, Right),
        ])
    };
}

// damn rust keeps saying CacauParser::ArithmeticOperation is private
pub fn okay(o: Node) -> Result<Term> {
    CacauParser::ArithmeticOperation(o)
}

#[pest_consume::parser]
#[allow(non_snake_case)]
impl CacauParser {
    pub fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    pub fn FloatLiteral(input: Node) -> Result<FloatLiteral> {
        let x: f64 = input.as_str().trim().parse().map_err(|e| input.error(e))?;

        Ok(FloatLiteral(x))
    }

    pub fn IntegerLiteral(input: Node) -> Result<IntegerLiteral> {
        let x: i64 = input.as_str().trim().parse().map_err(|e| input.error(e))?;

        Ok(IntegerLiteral(x))
    }

    pub fn CharLiteral(input: Node) -> Result<CharLiteral> {
        let x: char = input.as_str().trim().parse().map_err(|e| input.error(e))?;

        Ok(CharLiteral(x))
    }

    pub fn BooleanLiteral(input: Node) -> Result<BooleanLiteral> {
        let x: bool = input.as_str().trim().parse().map_err(|e| input.error(e))?;

        Ok(BooleanLiteral(x))
    }

    pub fn StringLiteral(input: Node) -> Result<StringLiteral> {
        let x: String = input.as_str().to_owned();

        Ok(StringLiteral(x))
    }

    pub fn Literal(input: Node) -> Result<Literal> {
        Ok(match_nodes!(input.into_children();
        [FloatLiteral(n)] => Literal::FloatLiteral(n),
        [IntegerLiteral(n)] => Literal::IntegerLiteral(n),
        [CharLiteral(n)] => Literal::CharLiteral(n),
        [BooleanLiteral(n)] => Literal::BooleanLiteral(n),
        [StringLiteral(n)] => Literal::StringLiteral(n),
            ))
    }

    pub fn Identifier(input: Node) -> Result<Identifier> {
        let x: String = input.as_str().to_owned();

        Ok(Identifier(x))
    }

    pub fn Term(input: Node) -> Result<Term> {
        Ok(match_nodes!(input.into_children();
        [Literal(n)] => Term::Literal(n),
        [Identifier(n)] => Term::Identifier(n),
        [ArithmeticOperation(n)] => n,
            ))
    }

    #[prec_climb(Term, PREC_CLIMBER)]
    pub fn ArithmeticOperation(left: Term, op: Node, right: Term) -> Result<Term> {
        match op.as_rule() {
            Rule::Add => Ok(Term::ArithmeticOperation(Box::new(ArithmeticOperation {
                left,
                op: ArithmeticOperator::Add,
                right,
            }))),
            Rule::Subtract => Ok(Term::ArithmeticOperation(Box::new(ArithmeticOperation {
                left,
                op: ArithmeticOperator::Subtract,
                right,
            }))),
            Rule::Multiply => Ok(Term::ArithmeticOperation(Box::new(ArithmeticOperation {
                left,
                op: ArithmeticOperator::Multiply,
                right,
            }))),
            Rule::Divide => Ok(Term::ArithmeticOperation(Box::new(ArithmeticOperation {
                left,
                op: ArithmeticOperator::Divide,
                right,
            }))),
            Rule::Power => Ok(Term::ArithmeticOperation(Box::new(ArithmeticOperation {
                left,
                op: ArithmeticOperator::Power,
                right,
            }))),
            Rule::Modulo => Ok(Term::ArithmeticOperation(Box::new(ArithmeticOperation {
                left,
                op: ArithmeticOperator::Modulo,
                right,
            }))),
            r => Err(op.error(format!("Rule {:?} isn't an operator", r))),
        }
    }

    /*
    fn ArithmeticOperation(op: Node) -> Result<ArithmeticOperation> {
        let mut op = op;
        while <Self as ::pest_consume::Parser>::allows_shortcut(op.as_rule()) {
            if let ::std::result::Result::Ok(child) = op.children().single() {
                if child.as_aliased_rule::<Self>()
                    == <Self as ::pest_consume::Parser>::rule_alias(Rule::ArithmeticOperation)
                {
                    op = child;
                    continue;
                }
            }
            break;
        }
        match op.as_rule() {
            Rule::ArithmeticOperation => {
                #[allow(non_snake_case)]
                fn ArithmeticOperation(
                    left: Term,
                    op: Node,
                    right: Term,
                ) -> Result<ArithmeticOperation> {
                    match op.as_rule() {
                        Rule::Add => Ok(ArithmeticOperation {
                            left,
                            op: ArithmeticOperator::Add,
                            right,
                        }),
                        Rule::Subtract => Ok(ArithmeticOperation {
                            left,
                            op: ArithmeticOperator::Subtract,
                            right,
                        }),
                        Rule::Multiply => Ok(ArithmeticOperation {
                            left,
                            op: ArithmeticOperator::Multiply,
                            right,
                        }),
                        Rule::Divide => Ok(ArithmeticOperation {
                            left,
                            op: ArithmeticOperator::Divide,
                            right,
                        }),
                        Rule::Power => Ok(ArithmeticOperation {
                            left,
                            op: ArithmeticOperator::Power,
                            right,
                        }),
                        Rule::Modulo => Ok(ArithmeticOperation {
                            left,
                            op: ArithmeticOperator::Modulo,
                            right,
                        }),
                        r => panic!(),
                    }
                }
                op.into_children()
                    .prec_climb(&*PREC_CLIMBER, Self::Term, ArithmeticOperation)
            }
            r => panic!(),
        }
    }*/
}

#[derive(Debug)]
pub enum Literal {
    FloatLiteral(FloatLiteral),
    IntegerLiteral(IntegerLiteral),
    CharLiteral(CharLiteral),
    BooleanLiteral(BooleanLiteral),
    StringLiteral(StringLiteral),
}

#[derive(Debug)]
pub struct FloatLiteral(f64);

#[derive(Debug)]
pub struct IntegerLiteral(i64);

#[derive(Debug)]
pub struct CharLiteral(char);

#[derive(Debug)]
pub struct BooleanLiteral(bool);
// TODO: parse the string (removing quotes, expanding \n, \t, etc)
#[derive(Debug)]
pub struct StringLiteral(String);

#[derive(Debug)]
pub struct Identifier(String);

#[derive(Debug)]
pub enum Term {
    Literal(Literal),
    Identifier(Identifier),
    ArithmeticOperation(Box<ArithmeticOperation>),
}

#[derive(Debug)]
pub struct ArithmeticOperation {
    left: Term,
    op: ArithmeticOperator,
    right: Term,
}

/*
}*/

#[derive(Debug)]
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulo,
}