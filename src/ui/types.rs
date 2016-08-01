// Traits
pub use std::io::Write;
pub use std::fmt::Write as FmtWrite;
pub use std::iter::FromIterator;

pub use std::path::{Path, PathBuf};
pub use std::collections::{HashMap, HashSet, VecDeque};

// modules
pub use regex::{Regex, RegexBuilder};

pub use super::super::core::{
    Settings, Artifact, Artifacts,
    ArtType, Loc,
    ArtName, ArtNames,
    LoadFromStr};

/// settings for what to format
/// [#SPC-core-fmt-settings]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FmtSettings {
    pub long: bool,
    pub recurse: u8,
    pub path: bool,
    pub parts: bool,
    pub partof: bool,
    pub loc_path: bool,
    pub text: bool,
    pub refs: bool,
}

impl FmtSettings {
    pub fn is_empty(&self) -> bool {
        !self.long && !self.path && !self.parts
            && !self.partof && !self.loc_path
            && !self.text && !self.refs
    }
}

/// structure which contains all the information necessary to
/// format an artifact for cmdline, html, or anything else
/// purposely doesn't contain items that are *always* displayed
/// such as completed or tested
/// [#SPC-core-fmt-artifact]
#[derive(Debug, Default)]
pub struct FmtArtifact {
    pub long: bool,
    pub path: Option<PathBuf>,
    pub parts: Option<Vec<FmtArtifact>>,
    pub partof: Option<Vec<FmtArtifact>>,
    pub loc: Option<Loc>,
    // pub loc_path: Option<PathBuf>,
    // pub loc_line_col: (usize, usize),
    // pub loc_valid: Option<bool>,
    pub refs: Option<Vec<String>>,
    pub text: Option<String>,
    pub name: ArtName,
}


#[derive(Debug, Default, PartialEq, Eq)]
pub struct PercentSearch {
    pub lt: bool,
    pub perc: u8,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct SearchSettings {
    pub use_regex: bool,
    pub name: bool,
    pub path: bool,
    pub parts: bool,
    pub partof: bool,
    pub loc: bool,
    pub refs: bool,
    pub text: bool,
    pub completed: PercentSearch,
    pub tested: PercentSearch,
}