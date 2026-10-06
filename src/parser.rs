use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::errors::InsertError;

/// Characters that are not allowed in parameter names.
const INVALID_PARAM_CHARS: [char; 4] = ['*', '<', '>', '/'];

/// A single part of a template.
#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) enum Part<'a> {
    Static { prefix: &'a str },
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
    ///
    /// # Errors
    ///
    /// Returns an error if the template is invalid.
    pub(crate) fn new(template: &'a str) -> Result<Self, InsertError> {
        if template.is_empty() {
            return Err(InsertError::Empty);
        }

        if !template.starts_with('/') {
            return Err(InsertError::MissingSlash);
        }

        let input = template.as_bytes();

        let mut parts = vec![];
        let mut cursor = 0;

        let mut seen_parameters: Vec<(&str, usize)> = Vec::new();

        while cursor < input.len() {
            match input[cursor] {
                b'<' => {
                    let (part, next) = Self::parse_parameter_part(template, cursor)?;

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
                    let (part, next_cursor) = Self::parse_static_part(template, cursor);

                    parts.push(part);
                    cursor = next_cursor;
                },
            }
        }

        parts.reverse();
        Ok(Self { parts })
    }

    fn parse_static_part(template: &'a str, cursor: usize) -> (Part<'a>, usize) {
        let end = memchr::memchr2(b'<', b'>', &template.as_bytes()[cursor..])
            .map_or(template.len(), |position| cursor + position);

        let prefix = &template[cursor..end];
        (Part::Static { prefix }, end)
    }

    /// Parses a single parameter, starting from its opening angle bracket.
    ///
    /// # Errors
    ///
    /// Returns an error if the parameter is invalid.
    fn parse_parameter_part(
        template: &'a str,
        cursor: usize,
    ) -> Result<(Part<'a>, usize), InsertError> {
        let start = cursor + 1;
        let end = memchr::memchr(b'>', &template.as_bytes()[start..])
            .map(|position| start + position)
            .ok_or(InsertError::UnbalancedAngle)?;

        let content = &template[start..end];
        let part = match content.strip_prefix('*') {
            Some(name) => Part::Wildcard {
                name: Self::parse_name(name)?,
            },
            None => Part::Dynamic {
                name: Self::parse_name(content)?,
            },
        };

        Ok((part, end + 1))
    }

    /// Validates a parameter name.
    ///
    /// # Errors
    ///
    /// When the name is empty or invalid.
    fn parse_name(name: &'a str) -> Result<&'a str, InsertError> {
        if name.is_empty() {
            return Err(InsertError::EmptyParameter);
        }

        if name.chars().any(|char| INVALID_PARAM_CHARS.contains(&char)) {
            return Err(InsertError::InvalidParameter { name: name.into() });
        }

        Ok(name)
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
                parts: vec![Part::Static { prefix: "/abcd" }],
            }),
        );
    }

    #[test]
    fn parser_dynamic_route() {
        assert_eq!(
            Template::new("/<name>"),
            Ok(Template {
                parts: vec![Part::Dynamic { name: "name" }, Part::Static { prefix: "/" },],
            }),
        );
    }

    #[test]
    fn parser_wildcard_route() {
        assert_eq!(
            Template::new("/<*wildcard>"),
            Ok(Template {
                parts: vec![Part::Wildcard { name: "wildcard" }, Part::Static {
                    prefix: "/"
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
                    prefix: "/files/"
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
