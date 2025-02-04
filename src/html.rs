use combine::{
    between, many, many1,
    parser::char::{self, char, letter, newline, space},
    satisfy, ParseError, Parser, Stream,
};

fn attribute<Input>() -> impl Parser<Input, Output = (String, String)>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        many1::<String, _, _>(letter()),
        many::<String, _, _>(space().or(newline())),
        char('='),
        many::<String, _, _>(space().or(newline())),
        between(
            char('"'),
            char('"'),
            many1::<String, _, _>(satisfy(|c: char| c != '"')),
        ),
    )
        .map(|v| (v.0, v.4))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_attribute() {
        assert_eq!(
            attribute().parse("test=\"foobar\""),
            Ok((("test".to_string(), "foobar".to_string()), ""))
        );

        assert_eq!(
            attribute().parse("test = \"foobar\""),
            Ok((("test".to_string(), "foobar".to_string()), ""))
        );
    }
}
