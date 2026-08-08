//! Optional tag registry loaded from `cairn.config.yaml`.

use std::collections::BTreeMap;

use super::parse::top_level_key;
use serde::Deserialize;

/// A documented blueprint tag.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TagEntry {
    /// One-line explanation of the tag.
    pub description: String,
    /// Whether the tag changes scanner or reconciler behaviour.
    pub behavior_affecting: bool,
}

/// The opt-in set of tags a project has documented.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TagRegistry {
    entries: BTreeMap<String, TagEntry>,
}

impl TagRegistry {
    /// Returns the entry for `tag`, if the project declared it.
    #[must_use]
    pub fn get(&self, tag: &str) -> Option<&TagEntry> {
        self.entries.get(tag)
    }

    /// Returns whether `tag` is declared in this registry.
    #[must_use]
    pub fn contains(&self, tag: &str) -> bool {
        self.entries.contains_key(tag)
    }

    pub(crate) fn parse(source: &str) -> Result<Option<Self>, String> {
        if !source
            .lines()
            .any(|line| top_level_key(line).as_deref() == Some("tags"))
        {
            return Ok(None);
        }
        let root: RootConfig = serde_yaml::from_str(source)
            .map_err(|error| format!("invalid `tags:` registry: {error}"))?;
        let tags = root.tags.unwrap_or_default();
        let entries = tags
            .into_iter()
            .map(|(name, tag)| (name, tag.into_entry()))
            .collect();
        Ok(Some(Self { entries }))
    }
}

#[derive(Debug, Deserialize)]
struct RootConfig {
    #[serde(default)]
    tags: Option<BTreeMap<String, RawTag>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RawTag {
    Description(String),
    Definition(RawTagDefinition),
}

impl RawTag {
    fn into_entry(self) -> TagEntry {
        match self {
            Self::Description(description) => TagEntry {
                description,
                behavior_affecting: false,
            },
            Self::Definition(definition) => TagEntry {
                description: definition.description,
                behavior_affecting: definition.behavior_affecting,
            },
        }
    }
}

#[derive(Debug, Default, Deserialize)]
struct RawTagDefinition {
    #[serde(default)]
    description: String,
    #[serde(default)]
    behavior_affecting: bool,
}
