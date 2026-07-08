use alloc::string::{String, ToString as _};
use alloc::vec;
use alloc::vec::Vec;

use crate::errors::InsertError;

/// Characters that are not allowed in parameter names.
const INVALID_PARAM_CHARS: [u8; 4] = [b'*', b'<', b'>', b'/'];

/// A single part of a template.
#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) enum Part<'a> {
    Static { prefix: &'a [u8] },
    Dynamic { name: &'a str },
    Wildcard { name: &'a str },
}

/// A parsed template.
#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) struct Template<'a> {
    pub parts: Vec<Part<'a>>,
}

impl<'a> Template<'a> {
    /// Parses a template string into its parts.
    pub(crate) fn new(template: &'a str) -> Result<Self, InsertError> {
        let input = template.as_bytes();

        if input.is_empty() {
            return Err(InsertError::Empty);
        }

        if input[0] != b'/' {
            return Err(InsertError::MissingSlash);
        }

        let mut parts = vec![];
        let mut cursor = 0;

        let mut seen_parameters: Vec<(&str, usize)> = Vec::new();

        while cursor < input.len() {
            match input[cursor] {
                b'<' => {
                    let (part, next) = Self::parse_parameter_part(input, cursor)?;

                    // Check for touching parameters.
                    if seen_parameters
                        .last()
                        .is_some_and(|&(_, last)| cursor == last)
                    {
                        return Err(InsertError::TouchingParameters);
                    }

                    // Check for duplicate names.
                    if let Part::Dynamic { name } | Part::Wildcard { name } = &part {
                        if seen_parameters.iter().any(|(existing, _)| existing == name) {
                            return Err(InsertError::DuplicateParameter {
                                name: String::from(*name),
                            });
                        }

                        seen_parameters.push((name, next));
                    }

                    parts.push(part);
                    cursor = next;
                },
                b'>' => {
                    return Err(InsertError::UnbalancedAngle);
                },
                _ => {
                    let (part, next_cursor) = Self::parse_static_part(input, cursor);

                    parts.push(part);
                    cursor = next_cursor;
                },
            }
        }

        parts.reverse();
        Ok(Self { parts })
    }

    fn parse_static_part(input: &'a [u8], cursor: usize) -> (Part<'a>, usize) {
        let end = memchr::memchr2(b'<', b'>', &input[cursor..])
            .map_or(input.len(), |position| cursor + position);

        let prefix = &input[cursor..end];
        (Part::Static { prefix }, end)
    }

    fn parse_parameter_part(
        input: &'a [u8],
        cursor: usize,
    ) -> Result<(Part<'a>, usize), InsertError> {
        let start = cursor + 1;
        let end = memchr::memchr(b'>', &input[start..])
            .map(|position| start + position)
            .ok_or(InsertError::UnbalancedAngle)?;

        let content = &input[start..end];
        if content.is_empty() {
            return Err(InsertError::EmptyParameter);
        }

        let is_wildcard = content.starts_with(b"*");
        let name = if is_wildcard { &content[1..] } else { content };

        if is_wildcard && name.is_empty() {
            return Err(InsertError::EmptyParameter);
        }

        if name.iter().any(|&c| INVALID_PARAM_CHARS.contains(&c)) {
            return Err(InsertError::InvalidParameter {
                name: String::from_utf8_lossy(name).to_string(),
            });
        }

        let name: &'a str =
            core::str::from_utf8(name).map_err(|_err| InsertError::InvalidParameter {
                name: String::from_utf8_lossy(name).to_string(),
            })?;

        let part = if is_wildcard {
            Part::Wildcard { name }
        } else {
            Part::Dynamic { name }
        };

        Ok((part, end + 1))
    }
}

#[cfg(test)]
mod tests {
    use similar_asserts::assert_eq;

    use super::*;

    #[test]
    fn parser_static_route() {
        assert_eq!(
            Template::new("/abcd"),
            Ok(Template {
                parts: vec![Part::Static { prefix: b"/abcd" }],
            }),
        );
    }

    #[test]
    fn parser_dynamic_route() {
        assert_eq!(
            Template::new("/<name>"),
            Ok(Template {
                parts: vec![Part::Dynamic { name: "name" }, Part::Static {
                    prefix: b"/"
                },],
            }),
        );
    }

    #[test]
    fn parser_wildcard_route() {
        assert_eq!(
            Template::new("/<*wildcard>"),
            Ok(Template {
                parts: vec![Part::Wildcard { name: "wildcard" }, Part::Static {
                    prefix: b"/"
                },],
            }),
        );
    }

    #[test]
    fn parser_route_with_wildcard_at_end() {
        assert_eq!(
            Template::new("/files/<*path>"),
            Ok(Template {
                parts: vec![Part::Wildcard { name: "path" }, Part::Static {
                    prefix: b"/files/"
                },],
            }),
        );
    }

    #[test]
    fn parser_error_empty() {
        let error = Template::new("").unwrap_err();
        insta::assert_snapshot!(error, @"empty template");
    }

    #[test]
    fn parser_error_empty_parameter() {
        let error = Template::new("/users/<>").unwrap_err();
        insta::assert_snapshot!(error, @"empty parameter name");
    }

    #[test]
    fn parser_error_missing_leading_slash() {
        let error = Template::new("abc").unwrap_err();
        insta::assert_snapshot!(error, @"missing leading slash");
    }

    #[test]
    fn parser_error_unbalanced_angle_opening() {
        let error = Template::new("/users/<id/profile").unwrap_err();
        insta::assert_snapshot!(error, @"unbalanced angle bracket");
    }

    #[test]
    fn parser_error_unbalanced_angle_closing() {
        let error = Template::new("/users/id>/profile").unwrap_err();
        insta::assert_snapshot!(error, @"unbalanced angle bracket");
    }

    #[test]
    fn parser_error_invalid_parameter() {
        let error = Template::new("/users/<user*name>/profile").unwrap_err();
        insta::assert_snapshot!(error, @"invalid parameter name `user*name`");
    }

    #[test]
    fn parser_error_duplicate_parameter() {
        let error = Template::new("/users/<id>/posts/<id>").unwrap_err();
        insta::assert_snapshot!(error, @"duplicate parameter name `id`");
    }

    #[test]
    fn parser_error_empty_wildcard() {
        let error = Template::new("/files/<*>").unwrap_err();
        insta::assert_snapshot!(error, @"empty parameter name");
    }

    #[test]
    fn parser_error_touching_parameters() {
        let error = Template::new("/users/<id><*name>").unwrap_err();
        insta::assert_snapshot!(error, @"parameters must be separated by a static character");
    }
}
