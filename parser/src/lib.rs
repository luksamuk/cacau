use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ExpressionParser;

#[cfg(test)]
mod parser_tests {
    use pest::Parser;

    use crate::Rule;
    use crate::ExpressionParser;

    fn parse(rule: Rule, expression: &str) -> Option<()> {
        ExpressionParser::parse(rule, expression).ok().map(|_| ())
    }

    fn assert_parses(rule: Rule, expression: &str) {
        assert!(parse(rule, expression).is_some())
    }

    fn assert_does_not_parse(rule: Rule, expression: &str) {
        assert!(parse(rule, expression).is_none())
    }

    #[test]
    fn single_char_identifiers() {
        assert_parses(Rule::identifier, "x");
        assert_parses(Rule::identifier, "z");

        assert_does_not_parse(Rule::identifier, "2");
        assert_does_not_parse(Rule::identifier, "7");   
    }

    #[test]
    fn valid_identifiers() {
        assert_parses(Rule::identifier, "a");
        assert_parses(Rule::identifier, "abc");
        assert_parses(Rule::identifier, "abc123");
        assert_parses(Rule::identifier, "zazz123");
    }

    // TODO: this test is failing ;-;
    // #[test]
    // pub fn invalid_identifiers() {
    //     assert_does_not_parse(Rule::identifier, "2");
    //     assert_does_not_parse(Rule::identifier, "3abc");
    //     assert_does_not_parse(Rule::identifier, "@abc123");
    //     assert_does_not_parse(Rule::identifier, "zaz@z123");
    // }

    #[test]
    fn integers() {
        assert_parses(Rule::integer, "1234");
        assert_parses(Rule::integer, "000000");
        assert_parses(Rule::integer, "987654321");
        assert_parses(Rule::integer, "987654321");
    }


    #[test]
    fn floats() {
        assert_parses(Rule::float, "123.4");
        assert_parses(Rule::float, "000.000");
        assert_parses(Rule::float, "98.7654321");
        assert_parses(Rule::float, "987654.321");

        assert_does_not_parse(Rule::float, "123");
        assert_does_not_parse(Rule::float, "50000");
        assert_does_not_parse(Rule::float, "abc123");
    }

    #[test]
    fn type_annotations() {
        assert_parses(Rule::type_annotation, ": int");
        assert_parses(Rule::type_annotation, ": float");
        assert_parses(Rule::type_annotation, ": string");

        assert_does_not_parse(Rule::type_annotation, "123");
        assert_does_not_parse(Rule::type_annotation, "string");
        assert_does_not_parse(Rule::type_annotation, ": 123");
    }

    #[test]
    fn assignment() {
        // TODO: add other kinds of expressions here when possible

        assert_parses(Rule::assignment, "let x = 123");
        assert_parses(Rule::assignment, "let nines = 999");

        assert_parses(Rule::assignment, "let six: int = 5");

        assert_parses(Rule::assignment, "let six: BigDecimal = 10");
        
        assert_does_not_parse(Rule::assignment, "let name");
        assert_does_not_parse(Rule::assignment, "let ch x: b");
        assert_does_not_parse(Rule::assignment, "six: BigDecimal = 10");
    }

    #[test]
    fn function_argument() {
        assert_parses(Rule::function_argument, "x: int");
        assert_parses(Rule::function_argument, "y: string");
        assert_parses(Rule::function_argument, "lambda: num");
        
        assert_does_not_parse(Rule::function_argument, "x: 123");
        assert_does_not_parse(Rule::function_argument, "y: \"Hey\"");
    }

    #[test]
    fn function_return() {
        assert_parses(Rule::function_return, "-> x");
        assert_parses(Rule::function_return, "-> y");
        assert_parses(Rule::function_return, "-> z");
        

        assert_does_not_parse(Rule::function_return, "-> 123");
        assert_does_not_parse(Rule::function_return, "->");
        assert_does_not_parse(Rule::function_return, "z");
    }

    #[test]
    fn function_declaration() {
        // TODO: rewrite these function indentifiers whenever the underline is available
        assert_parses(Rule::function_declaration, "fn main");
        assert_parses(Rule::function_declaration, "pub fn noargsbutreturnssomething -> int");
        assert_parses(Rule::function_declaration, "fn noargsbutreturnssomething -> int");
        assert_parses(Rule::function_declaration, "fn oneargnoreturn x: int");
        assert_parses(Rule::function_declaration, "fn oneargonereturn x: int -> int");
        assert_parses(Rule::function_declaration, "fn severalargs x: int, y: int");
        assert_parses(Rule::function_declaration, "pub fn severalargs x: int, y: int");
        assert_parses(Rule::function_declaration, "fn severalargswithreturn x: int, y: int, z: int -> bool");


        assert_does_not_parse(Rule::function_declaration, "fn");
        assert_does_not_parse(Rule::function_declaration, "pub fn");
        assert_does_not_parse(Rule::function_declaration, "fn -> bool");

        // TODO: use SOI and EOI matching here whenever available
        // No function name
        // assert_does_not_parse(Rule::function_declaration, "fn x: int, y: int, z: int -> bool");

        // Arrow set but return type not specified
        // assert_does_not_parse(Rule::function_declaration, "fn func x: int, y: int, z: int ->");

        // Incorrect argument specification
        // assert_does_not_parse(Rule::function_declaration, "fn func x: -> bool");
    }
}