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
        input.parse_until_before( cssparser::Delimiter::Bang, |i| {
            while i.next().is_ok() {}
            Ok::<(), ParseError<()>>(())
        })?;
        let value = input.slice_from(start).trim().to_string(); // 3. the text from the bookmark to here

        let priority = if input.try_parse(cssparser::parse_important).is_ok() {
            "important".to_string()
        } else {
            String::new()
        };
        Ok(Decl {name: name.to_string(), value: value, priority: priority})
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
    let tests = [
        "padding: 20px; color: red !important",
        "border: 2px solid var(--x)",
        "color red; padding: 1px",
        "/* note */ color: red",
        "color: red; ; ;",
        "a: b ! important",
        "x: f(!important) !important",
        "w: calc(1px + 2px)",
    ];
    for text in tests {
        println!("--- {text}");
        let mut parser = Parser::new(text);
        for item in RuleBodyParser::new(&mut parser, &mut MyParser) {
            println!("{:?}", item);
        }
    }
}



