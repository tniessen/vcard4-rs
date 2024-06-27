use thiserror::Error;

/// Error lexing a vcard string.
#[derive(Debug, Error, PartialEq, Clone, Default)]
#[doc(hidden)]
pub enum LexError {
    /// Generic lex error.
    #[default]
    #[error("vcard lex error")]
    Other,
}

/// Errors generated by the vCard library.
#[derive(Debug, Error)]
pub enum Error {
    /// Error generated when a token was expected but no more tokens
    /// are available; end-of-file (EOF) was reached.
    #[error("input token was expected but reached EOF")]
    TokenExpected,

    /// Error generated when a version is encountered that is not
    /// the first property.
    #[error("version must be the first property")]
    VersionMisplaced,

    /// Error generated when a control character is encountered.
    #[error("control characters are not allowed, got '{0}'")]
    ControlCharacter(String),

    /// Error generated when an expected token is of the wrong type.
    #[error("input token '{0}' was incorrect")]
    IncorrectToken(String),

    /// Error generated when an unknown parameter is encountered.
    #[error("unknown parameter '{0}'")]
    UnknownParameter(String),

    /// Error generated when an unknown property name is encountered.
    #[error("property name '{0}' is not supported")]
    UnknownPropertyName(String),

    /// Error generated when a property value is invalid.
    #[error("property value is invalid")]
    InvalidPropertyValue,

    /// Error generated when a time is invalid.
    #[error("time '{0}' is invalid")]
    InvalidTime(String),

    /// Error generated when a date is invalid.
    #[error("date '{0}' is invalid")]
    InvalidDate(String),

    /// Error generated when a delivery address (`ADR`) is invalid.
    #[error("delivery address '{0}' is invalid")]
    InvalidAddress(String),

    /// Error generated when a LABEL parameter is specified on a property
    /// other than ADR.
    #[error("parameter LABEL can only be applied to ADR but used on '{0}'")]
    InvalidLabel(String),

    /// Error generated when a boolean is invalid.
    #[error("value '{0}' is not a valid boolean")]
    InvalidBoolean(String),

    /// Error generated when a CLIENTPIDMAP value could not be parsed.
    #[error("client PID map '{0}' is not valid")]
    InvalidClientPidMap(String),

    /// Error generated when a property or parameter delimiter was expected.
    #[error("property or parameter delimiter expected")]
    DelimiterExpected,

    /// Error generated when a value type is not supported.
    #[error("value type '{0}' is not supported")]
    UnknownValueType(String),

    /// Error generated when a TYPE for a RELATED property is not supported.
    #[error("related type value '{0}' is not supported")]
    UnknownRelatedType(String),

    /// Error generated when a TYPE for a TEL property is not supported.
    #[error("telephone type value '{0}' is not supported")]
    UnknownTelephoneType(String),

    /// Error generated when a VALUE for a property is not supported.
    #[error("value '{0}' is not supported in this context '{1}'")]
    UnsupportedValueType(String, String),

    /// Error generated when a KIND is not supported.
    #[error("kind '{0}' is not supported")]
    UnknownKind(String),

    /// Error generated when the sex of a GENDER is not supported.
    #[error("sex '{0}' is not supported")]
    UnknownSex(String),

    /// Error generated when a GENDER does not specify the sex.
    #[error("gender value is missing sex")]
    NoSex,

    /// Error generated when a property appears more than once.
    #[error("property '{0}' may only appear exactly once")]
    OnlyOnce(String),

    /// Error generated when the FN property is not specified.
    #[error("formatted name (FN) is required")]
    NoFormattedName,

    /// Error generated when a date time is not valid.
    #[error("date time '{0}' is not valid, maybe missing 'T' delimiter")]
    InvalidDateTime(String),

    /// Error generated when a TYPE parameter is given for a property
    /// that does not support it.
    #[error("TYPE parameter is not supported for property '{0}'")]
    TypeParameter(String),

    /// Error generated when a PREF is out of bounds.
    #[error("pref '{0}' is out of bounds, must be between 1 and 100")]
    PrefOutOfRange(u8),

    /// Error generated when a PID is invalid.
    #[error("pid '{0}' is invalid")]
    InvalidPid(String),

    /// Error generated when an unquoted value was encountered when it must
    /// be quoted; eg: the GEO parameter URI.
    #[error("'{0}' must be enclosed in quotes")]
    NotQuoted(String),

    /// Error generated when MEMBER is specified but the kind is not group.
    #[error("member property is only allowed when the kind is group")]
    MemberRequiresGroup,

    /// Error generated when the PID parameter is used on the
    /// CLIENTPIDMAP property.
    #[error("PID parameter not allowed for CLIENTPIDMAP")]
    ClientPidMapPidNotAllowed,

    /// Errors generated by the language tags library.
    #[cfg(feature = "language-tags")]
    #[error(transparent)]
    LanguageParse(#[from] language_tags::ParseError),

    /// Errors generated by the URI library.
    #[error(transparent)]
    UriParse(#[from] uriparse::uri::URIError),

    /// Errors generated by time library.
    #[error(transparent)]
    ComponentRange(#[from] time::error::ComponentRange),

    /// Errors generated by time library parsing.
    #[error(transparent)]
    TimeParse(#[from] time::error::Parse),

    /// Errors generated by time library formatting.
    #[error(transparent)]
    TimeFormat(#[from] time::error::Format),

    /// Errors generated by time library format descriptions.
    #[error(transparent)]
    TimeInvalidFormat(#[from] time::error::InvalidFormatDescription),

    /// Error generated parsing a string to an integer.
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    /// Error generated parsing a string to a float.
    #[error(transparent)]
    ParseFloat(#[from] std::num::ParseFloatError),

    /// Error generated parsing a media type.
    #[cfg(feature = "mime")]
    #[error(transparent)]
    Mime(#[from] mime::FromStrError),

    /// Error generated decoding from base64.
    #[error(transparent)]
    Base64(#[from] base64::DecodeError),

    /// Error generated during lexing.
    #[error(transparent)]
    LexError(#[from] LexError),

    /// Error generated when a CHARSET other than UTF-8 is specified.
    #[error("CHARSET='{0}' is invalid, expected UTF-8")]
    CharsetParameter(String),
}
