use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use indexmap::IndexSet;

#[derive(Debug)]
pub enum TypingError {
    UnknownValue { t: String, possible: String, val: String }
}
impl Display for TypingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypingError::UnknownValue { t, possible, val } => write!(f, "{t} contains unknown value {val}. Try {possible}. Do you have the right features enabled?"),
        }
    }
}
impl Error for TypingError {}

#[derive(Clone, Eq, PartialEq)]
pub enum SpecType {
    CustomHttp,
    CustomFile,
    GitHubPublic,
    #[cfg(feature = "gh-pri")]
    GitHubPrivate,
}
impl Default for SpecType {
    fn default() -> Self {
        SpecType::GitHubPublic
    }
}
impl FromStr for SpecType {
    type Err = TypingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cuhttp" => Ok(SpecType::CustomHttp),
            "cufile" => Ok(SpecType::CustomFile),
            "gh-pub" => Ok(SpecType::GitHubPublic),
            #[cfg(feature = "gh-pri")]
            "gh-pri" => Ok(SpecType::GitHubPrivate),
            s => Err(TypingError::UnknownValue { t: "Spec".to_string(), possible: "cuhttp, cufile, gh-pub".to_string(), val: s.to_string() })
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SigType {
    None,
    #[cfg(feature = "minisign")]
    MiniSign
}
impl Default for SigType {
    fn default() -> Self {
        #[cfg(feature = "minisign")]
        return SigType::MiniSign;
        #[cfg(not(feature = "minisign"))]
        return SigType::None;
    }
}
impl FromStr for SigType {
    type Err = TypingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(SigType::None),
            #[cfg(feature = "minisign")]
            "minisign" => Ok(SigType::MiniSign),
            s => Err(TypingError::UnknownValue { t: "Sig".to_string(), possible: "none, minisign".to_string(), val: s.to_string() })
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum VerifyType {
    None,
    #[cfg(feature = "sha3")]
    Sha3_512,
    #[cfg(feature = "sha3")]
    Sha3_256,
    #[cfg(feature = "sha2")]
    Sha512,
    #[cfg(feature = "sha2")]
    Sha256,
}
impl VerifyType {
    pub fn get_defaults() -> IndexSet<VerifyType> {
        let mut c = IndexSet::new();

        #[cfg(feature = "sha3")]
        {
            c.insert(VerifyType::Sha3_512);
            c.insert(VerifyType::Sha3_256);
        }
        #[cfg(feature = "sha2")]
        {
            c.insert(VerifyType::Sha512);
            c.insert(VerifyType::Sha256);
        }
        #[cfg(not(any(feature = "sha2", feature = "sha3")))]
        set.insert(VerifyType::None);

        c
    }
}
impl FromStr for VerifyType{
    type Err = TypingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(VerifyType::None),
            #[cfg(feature = "sha3")]
            "sha3_512" => Ok(VerifyType::Sha3_512),
            #[cfg(feature = "sha3")]
            "sha3_256" => Ok(VerifyType::Sha3_256),
            #[cfg(feature = "sha2")]
            "sha512" => Ok(VerifyType::Sha512),
            #[cfg(feature = "sha2")]
            "sha256" => Ok(VerifyType::Sha256),
            s => Err(TypingError::UnknownValue { t: "Verify".to_string(), possible: "none, sha256, sha512, sha3_256, sha3_512".to_string(), val: s.to_string() })
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ReportType {
    AuditDL,
    AuditOut,
    DepsDL,
    DepsOut,
    LicenseDL,
    LicenseOut,
}
impl ReportType {
    pub fn get_defaults() -> HashSet<ReportType> {
        HashSet::from([ReportType::LicenseDL])
    }
}
impl FromStr for ReportType {
    type Err = TypingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "audit_dl" => Ok(ReportType::AuditDL),
            "audit_out" => Ok(ReportType::AuditOut),
            "deps_dl" => Ok(ReportType::DepsDL),
            "deps_out" => Ok(ReportType::DepsOut),
            "license_dl" => Ok(ReportType::LicenseDL),
            "license_out" => Ok(ReportType::LicenseOut),
            s => Err(TypingError::UnknownValue { t: "Report".to_string(), possible: "audit_dl, audit_out, deps_dl, deps_out, license_dl, license_out".to_string(), val: s.to_string() })
        }
    }
}
