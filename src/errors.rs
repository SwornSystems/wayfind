use alloc::string::String;
use core::error::Error;
use core::fmt;

/// An error that occurred while inserting a template.
#[non_exhaustive]
#[derive(Clone, PartialEq, Debug)]
pub enum InsertError {
    /// The template is empty.
    Empty,

    /// The template does not start with `/`.
    MissingSlash,

    /// An unbalanced `<` or `>` was found in the template.
    UnbalancedAngle,

    /// A parameter name is empty.
    EmptyParameter,

    /// A parameter name contains invalid characters.
    InvalidParameter {
        /// The invalid parameter name.
        name: String,
    },

    /// A parameter name appears more than once in the template.
    DuplicateParameter {
        /// The duplicated parameter name.
        name: String,
    },

    /// Parameters are touching without a static separator.
    TouchingParameters,

    /// The template conflicts with an already inserted template.
    Conflict {
        /// The existing template that conflicts.
        existing: String,
    },
}

impl Error for InsertError {}

impl fmt::Display for InsertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty template"),
            Self::MissingSlash => write!(f, "missing leading slash"),
            Self::UnbalancedAngle => write!(f, "unbalanced angle bracket"),
            Self::EmptyParameter => write!(f, "empty parameter name"),
            Self::InvalidParameter { name } => write!(f, "invalid parameter name `{name}`"),
            Self::DuplicateParameter { name } => write!(f, "duplicate parameter name `{name}`"),
            Self::TouchingParameters => {
                write!(f, "parameters must be separated by a static character")
            },
            Self::Conflict { existing } => write!(f, "conflicts with `{existing}`"),
        }
    }
}
