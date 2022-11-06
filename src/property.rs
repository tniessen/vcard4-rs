//! Types for properties.

use std::{
    fmt::{self, Display},
    str::FromStr,
};
use time::{Date, OffsetDateTime, Time, UtcOffset};
use uriparse::uri::URI as Uri;

#[cfg(feature = "language-tags")]
use language_tags::LanguageTag;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{
    parameter::Parameters,
    types::{ClientPidMap, DateAndOrTime, Float, Integer},
    Error, Result,
};

/// Trait for vCard properties.
pub trait Property: Display {
    /// Get the property group.
    fn group(&self) -> Option<&String>;

    /// Get the property parameters.
    fn parameters(&self) -> Option<&Parameters>;
}

/// Delivery address for the ADR property.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct DeliveryAddress {
    /// The post office box.
    pub po_box: Option<String>,
    /// The extended address (e.g: apartment or suite number).
    pub extended_address: Option<String>,
    /// The street address.
    pub street_address: Option<String>,
    /// The locality (e.g: city).
    pub locality: Option<String>,
    /// The region (e.g: state or province).
    pub region: Option<String>,
    /// The postal code.
    pub postal_code: Option<String>,
    /// The country name.
    pub country_name: Option<String>,
}

impl fmt::Display for DeliveryAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{};{};{};{};{};{};{}",
            self.po_box.as_ref().map(|s| &s[..]).unwrap_or_else(|| ""),
            self.extended_address
                .as_ref()
                .map(|s| &s[..])
                .unwrap_or_else(|| ""),
            self.street_address
                .as_ref()
                .map(|s| &s[..])
                .unwrap_or_else(|| ""),
            self.locality.as_ref().map(|s| &s[..]).unwrap_or_else(|| ""),
            self.region.as_ref().map(|s| &s[..]).unwrap_or_else(|| ""),
            self.postal_code
                .as_ref()
                .map(|s| &s[..])
                .unwrap_or_else(|| ""),
            self.country_name
                .as_ref()
                .map(|s| &s[..])
                .unwrap_or_else(|| ""),
        )
    }
}

/// The ADR property.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct AddressProperty {
    /// Group for this property.
    pub group: Option<String>,
    /// The value for the property.
    pub value: DeliveryAddress,
    /// The property parameters.
    pub parameters: Option<Parameters>,
}

/// Client PID map property.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct ClientPidMapProperty {
    /// Group for this property.
    pub group: Option<String>,
    /// The value for the property.
    pub value: ClientPidMap,
    /// The property parameters.
    pub parameters: Option<Parameters>,
}

/// Extension property.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct ExtensionProperty {
    /// The property name.
    pub name: String,
    /// Group for this property.
    pub group: Option<String>,
    /// The value for the property.
    pub value: AnyProperty,
    /// The property parameters.
    pub parameters: Option<Parameters>,
}

/// Value for any property type.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum AnyProperty {
    /// Text property.
    Text(String),
    /// Integer property.
    Integer(Integer),
    /// Float property.
    Float(Float),
    /// Boolean property.
    Boolean(bool),

    /// Date value.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    Date(Date),
    /// Date and time value.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    DateTime(OffsetDateTime),
    /// Time value.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    Time(Time),
    /// Date and or time value.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    DateAndOrTime(DateAndOrTime),
    /// Timetamp value.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    Timestamp(OffsetDateTime),
    /// URI property.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    Uri(Uri<'static>),
    /// UTC offset property.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    UtcOffset(UtcOffset),
    /// Language property.
    #[cfg(feature = "language-tags")]
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    Language(LanguageTag),

    /// Language property.
    #[cfg(not(feature = "language-tags"))]
    Language(String),
}

/// Language property.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct LanguageProperty {
    /// Group for this property.
    pub group: Option<String>,
    /// The value for the property.
    #[cfg(feature = "language-tags")]
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub value: LanguageTag,

    /// The value for the property.
    #[cfg(not(feature = "language-tags"))]
    pub value: String,

    /// The property parameters.
    pub parameters: Option<Parameters>,
}

/// Date time property.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct DateTimeProperty {
    /// Group for this property.
    pub group: Option<String>,
    /// The value for the property.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub value: OffsetDateTime,
    /// The property parameters.
    pub parameters: Option<Parameters>,
}

/// Date and or time property.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DateAndOrTimeProperty {
    /// Group for this property.
    pub group: Option<String>,
    /// The value for the property.
    pub value: DateAndOrTime,
    /// The property parameters.
    pub parameters: Option<Parameters>,
}

/// Either text or a Uri.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum TextOrUriProperty {
    /// Text value.
    Text(TextProperty),
    /// Uri value.
    Uri(UriProperty),
}

impl Property for TextOrUriProperty {
    fn group(&self) -> Option<&String> {
        match self {
            Self::Text(val) => val.group(),
            Self::Uri(val) => val.group(),
        }
    }

    fn parameters(&self) -> Option<&Parameters> {
        match self {
            Self::Text(val) => val.parameters(),
            Self::Uri(val) => val.parameters(),
        }
    }
}

impl fmt::Display for TextOrUriProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(val) => write!(f, "{}", val),
            Self::Uri(val) => write!(f, "{}", val),
        }
    }
}

/// Either text or a date and or time.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum DateTimeOrTextProperty {
    /// Date time value.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    DateTime(DateAndOrTimeProperty),
    /// Text value.
    Text(TextProperty),
}

impl Property for DateTimeOrTextProperty {
    fn group(&self) -> Option<&String> {
        match self {
            Self::Text(val) => val.group(),
            Self::DateTime(val) => val.group(),
        }
    }

    fn parameters(&self) -> Option<&Parameters> {
        match self {
            Self::Text(val) => val.parameters(),
            Self::DateTime(val) => val.parameters(),
        }
    }
}

impl fmt::Display for DateTimeOrTextProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(val) => write!(f, "{}", val),
            Self::DateTime(val) => write!(f, "{}", val),
        }
    }
}

/// Value for a UTC offset property.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct UtcOffsetProperty {
    /// Group for this property.
    pub group: Option<String>,
    /// The value for the UTC offset.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub value: UtcOffset,
    /// The parameters for the property.
    pub parameters: Option<Parameters>,
}

impl fmt::Display for UtcOffsetProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (h, m, _) = self.value.as_hms();
        let sign = if h >= 0 { '+' } else { '-' };
        let h = h.abs();
        let m = m.abs();
        let h = if h < 10 {
            format!("0{}", h)
        } else {
            h.to_string()
        };
        let m = if m < 10 {
            format!("0{}", m)
        } else {
            m.to_string()
        };
        write!(f, "{}{}{}", sign, h, m,)
    }
}

impl FromStr for UtcOffsetProperty {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() == 5 {
            let sign = &s[0..1];
            if sign != "+" && sign != "-" {
                return Err(Error::InvalidUtcOffset(s.to_string()));
            }
            let hours = &s[1..3];
            let minutes = &s[3..5];
            let mut hours: i8 = hours.parse()?;
            let mut minutes: i8 = minutes.parse()?;
            if sign == "-" {
                hours = -hours;
                minutes = -minutes;
            }
            return Ok(Self {
                value: UtcOffset::from_hms(hours, minutes, 0)?,
                parameters: None,
                group: None,
            });
        }

        Err(Error::InvalidUtcOffset(s.to_string()))
    }
}

/// Value for a timezone property.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum TimeZoneProperty {
    /// Text value.
    Text(TextProperty),
    /// Uri value.
    Uri(UriProperty),
    /// UTC offset value.
    UtcOffset(UtcOffsetProperty),
}

impl Property for TimeZoneProperty {
    fn group(&self) -> Option<&String> {
        match self {
            Self::Text(val) => val.group(),
            Self::Uri(val) => val.group(),
            Self::UtcOffset(val) => val.group(),
        }
    }

    fn parameters(&self) -> Option<&Parameters> {
        match self {
            Self::Text(val) => val.parameters(),
            Self::Uri(val) => val.parameters(),
            Self::UtcOffset(val) => val.parameters(),
        }
    }
}

impl fmt::Display for TimeZoneProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(val) => write!(f, "{}", val),
            Self::Uri(val) => write!(f, "{}", val),
            Self::UtcOffset(val) => write!(f, "{}", val),
        }
    }
}

/// Text property value.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct TextProperty {
    /// Group for this property.
    pub group: Option<String>,
    /// Value for this property.
    pub value: String,
    /// Parameters for this property.
    pub parameters: Option<Parameters>,
}

/// Text list property value.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct TextListProperty {
    /// Group for this property.
    pub group: Option<String>,
    /// Value for this property.
    pub value: Vec<String>,
    /// Parameters for this property.
    pub parameters: Option<Parameters>,
}

impl fmt::Display for TextListProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value.join(","))
    }
}

/// Uri property value.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct UriProperty {
    /// Group for this property.
    pub group: Option<String>,
    /// Value for this property.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub value: Uri<'static>,
    /// Parameters for this property.
    pub parameters: Option<Parameters>,
}

/// Property for a vCard kind.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct KindProperty {
    /// Group for this property.
    pub group: Option<String>,
    /// The value for the property.
    pub value: Kind,
    /// The property parameters.
    pub parameters: Option<Parameters>,
}

/// Kind of vCard.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum Kind {
    /// An individual.
    Individual,
    /// A group.
    Group,
    /// An organization.
    Org,
    /// A location.
    Location,
    // TODO: x-name
    // TODO: iana-token
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Individual => "individual",
                Self::Group => "group",
                Self::Org => "org",
                Self::Location => "location",
            }
        )
    }
}

impl FromStr for Kind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "individual" => Ok(Self::Individual),
            "group" => Ok(Self::Group),
            "org" => Ok(Self::Org),
            "location" => Ok(Self::Location),
            _ => Err(Error::UnknownKind(s.to_string())),
        }
    }
}

/// Property for a vCard gender.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct GenderProperty {
    /// Group for this property.
    pub group: Option<String>,
    /// The value for the property.
    pub value: Gender,
    /// The property parameters.
    pub parameters: Option<Parameters>,
}

/// Represents a gender associated with a vCard.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct Gender {
    /// The sex for the gender.
    pub sex: Sex,
    /// The identity text.
    pub identity: Option<String>,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(identity) = &self.identity {
            write!(f, "{};{}", self.sex, identity)
        } else {
            write!(f, "{}", self.sex,)
        }
    }
}

impl FromStr for Gender {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.is_empty() {
            return Ok(Gender {
                sex: Sex::None,
                identity: None,
            });
        }

        let mut it = s.splitn(2, ";");
        let sex = it.next().ok_or(Error::NoSex)?;
        let sex: Sex = sex.parse()?;
        let mut gender = Gender {
            sex,
            identity: None,
        };
        if let Some(identity) = it.next() {
            gender.identity = Some(identity.to_string());
        }

        Ok(gender)
    }
}

/// Enumeration for sex.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum Sex {
    /// No sex specified.
    None,
    /// Male sex.
    Male,
    /// Female sex.
    Female,
    /// Other sex.
    Other,
    /// Not applicable.
    NotApplicable,
    /// Unknown sex.
    Unknown,
}

impl fmt::Display for Sex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "",
                Self::Male => "M",
                Self::Female => "F",
                Self::Other => "O",
                Self::NotApplicable => "N",
                Self::Unknown => "U",
            }
        )
    }
}

impl FromStr for Sex {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "" => Ok(Self::None),
            "M" => Ok(Self::Male),
            "F" => Ok(Self::Female),
            "O" => Ok(Self::Other),
            "N" => Ok(Self::NotApplicable),
            "U" => Ok(Self::Unknown),
            _ => Err(Error::UnknownSex(s.to_string())),
        }
    }
}

macro_rules! property_impl {
    ($prop:ty) => {
        impl Property for $prop {
            fn group(&self) -> Option<&String> {
                self.group.as_ref()
            }

            fn parameters(&self) -> Option<&Parameters> {
                self.parameters.as_ref()
            }
        }
    };
}

macro_rules! display_impl {
    ($prop:ty) => {
        impl fmt::Display for $prop {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.value)
            }
        }
    };
}

property_impl!(AddressProperty);
display_impl!(AddressProperty);

property_impl!(UriProperty);
display_impl!(UriProperty);

property_impl!(KindProperty);
display_impl!(KindProperty);

property_impl!(TextProperty);
display_impl!(TextProperty);

property_impl!(LanguageProperty);
display_impl!(LanguageProperty);

property_impl!(DateTimeProperty);
display_impl!(DateTimeProperty);

property_impl!(DateAndOrTimeProperty);
display_impl!(DateAndOrTimeProperty);

property_impl!(ClientPidMapProperty);
display_impl!(ClientPidMapProperty);

property_impl!(GenderProperty);
display_impl!(GenderProperty);

// Bespoke Display implementations
property_impl!(TextListProperty);
property_impl!(UtcOffsetProperty);

//property_impl!(ExtensionProperty);

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn parse_utc_offset() -> Result<()> {
        let east = "+1200".parse::<UtcOffsetProperty>()?;
        let west = "-0500".parse::<UtcOffsetProperty>()?;

        assert_eq!("+1200", east.to_string());
        assert_eq!("-0500", west.to_string());

        assert!("0500".parse::<UtcOffsetProperty>().is_err());
        assert!("foo".parse::<UtcOffsetProperty>().is_err());
        assert!("+4400".parse::<UtcOffsetProperty>().is_err());

        Ok(())
    }
}
