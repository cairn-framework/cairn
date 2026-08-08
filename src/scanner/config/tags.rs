//! Optional tag registry loaded from `cairn.config.yaml`.

use std::collections::BTreeMap;

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

    pub(crate) fn parse(source: &str) -> Option<Self> {
        let root: RootConfig = serde_yaml::from_str(source).ok()?;
        let tags = root.tags?;
        let entries = match tags {
            RawTags::Map(tags) => tags
                .into_iter()
                .map(|(name, tag)| (name, tag.into_entry()))
                .collect(),
            RawTags::List(tags) => tags
                .into_iter()
                .filter_map(|tag| {
                    let name = if tag.name.is_empty() {
                        tag.tag
                    } else {
                        tag.name
                    };
                    (!name.is_empty()).then_some((
                        name,
                        TagEntry {
                            description: tag.description,
                            behavior_affecting: tag.behavior_affecting,
                        },
                    ))
                })
                .collect(),
        };
        Some(Self { entries })
    }
}

#[derive(Debug, Deserialize)]
struct RootConfig {
    #[serde(default)]
    tags: Option<RawTags>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RawTags {
    Map(BTreeMap<String, RawTag>),
    List(Vec<RawNamedTag>),
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

#[derive(Debug, Default, Deserialize)]
struct RawNamedTag {
    #[serde(default)]
    name: String,
    #[serde(default)]
    tag: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    behavior_affecting: bool,
}
