use nom::{
    branch::alt, bytes::complete::tag, character::complete::alphanumeric1, multi::many_m_n, IResult,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MediaType<'a>(&'a str);

impl MediaType<'_> {
    pub fn parse(input: &str) -> Option<(&str, MediaType<'_>)> {
        let (rest, _) = many_m_n(1, 127, character)(input).ok()?;
        Some((rest, MediaType(&input[..input.len() - rest.len()])))
    }

    pub fn as_str(&self) -> &str {
        self.0
    }
}

fn character(input: &str) -> IResult<&str, &str> {
    alt((
        alphanumeric1,
        tag("!"),
        tag("#"),
        tag("$"),
        tag("&"),
        tag("."),
        tag("+"),
        tag("-"),
        tag("^"),
        tag("_"),
    ))(input)
}
