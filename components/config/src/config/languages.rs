use std::collections::HashMap;

use errors::{bail, Result};
use libs::unic_langid::LanguageIdentifier;
use serde::{Deserialize, Serialize};

use crate::config::search;
use crate::config::taxonomies;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LanguageOptions {
    /// Title of the site. Defaults to None
    pub title: Option<String>,
    /// Description of the site. Defaults to None
    pub description: Option<String>,
    /// Whether to generate a feed for that language, defaults to `false`
    pub generate_feed: bool,
    /// The filename to use for feeds. Used to find the template, too.
    /// Defaults to "atom.xml", with "rss.xml" also having a template provided out of the box.
    pub feed_filename: String,
    pub taxonomies: Vec<taxonomies::TaxonomyConfig>,
    /// Whether to generate search index for that language, defaults to `false`
    pub build_search_index: bool,
    /// The search config, telling what to include in the search index for that language
    pub search: search::Search,
    /// A toml crate `Table` with String key representing term and value
    /// another `String` representing its translation.
    /// Use `get_translation()` method for translating key into different languages.
    pub translations: HashMap<String, String>,
}

/// We want to ensure the language codes are valid ones
pub fn validate_code(code: &str) -> Result<()> {
    if LanguageIdentifier::from_bytes(code.as_bytes()).is_err() {
        bail!("Language `{}` is not a valid Unicode Language Identifier (see http://unicode.org/reports/tr35/#Unicode_language_identifier)", code)
    }

    Ok(())
}
