use cssparser::{AtRuleParser, DeclarationParser, ParseError, Parser, ParserState, QualifiedRuleParser, RuleBodyItemParser, RuleBodyParser, CowRcStr};

// One line of the Rules panel (practice NodeStyle)
#[derive(Debug)]
struct Decl {
    name: String,  
    value: String,
    priority: String,
}

//The object cssparser will ask questions to. No data needed
struct MyParser;


impl<'i> AtRuleParser<'i> for MyParser {
    type Prelude = ();
    type AtRule = Decl; // must be the SAME type as Declaration (see note A)
    type Error = ();
}

impl<'i> QualifiedRuleParser<'i> for MyParser {
    type Prelude = ();
    type QualifiedRule = Decl;
    type Error = ();
}

impl<'i> DeclarationParser<'i> for MyParser {
    type Declaration = Decl;
    type Error = ();

    fn parse_value(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i>,
        _declaration_start: &ParserState,
    ) -> Result<Decl, ParseError<()>> {
        let start = input.position(); // 1. bookmark , the value starts here
        while input.next().is_ok() {} // 2. walk to the end of the value
        let value = input.slice_from(start).trim().to_string(); // 3. the text from the bookmark to here
        Ok(Decl {name: name.to_string(), value: value, priority: String::new()})
    }
}

impl<'i> RuleBodyItemParser<'i, Decl, ()>
    for MyParser
{
    fn parse_declarations(&self) -> bool {
        true
    }
    fn parse_qualified(&self) -> bool {
        false
    }
}


fn main() {
    let text = "padding: 20px; color: red !important";
    let mut parser = Parser::new(text);
    for item in RuleBodyParser::new(&mut parser, &mut MyParser) {
        println!("{:?}", item);
    }
}



