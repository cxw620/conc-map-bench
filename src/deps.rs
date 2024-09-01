use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum DependencyValue {
    String(Arc<str>),
    Object {
        package: Option<Arc<str>>,
        version: Arc<str>,
        features: Option<Vec<Arc<str>>>,
        optional: Option<bool>,
    },
    Git {
        package: Option<Arc<str>>,
        git: Arc<str>,
        branch: Option<Arc<str>>,
        rev: Option<Arc<str>>,
        version: Option<Arc<str>>,
        features: Option<Vec<Arc<str>>>,
        optional: Option<bool>,
    },
}

impl DependencyValue {
    pub(crate) fn version(&self) -> Option<Arc<str>> {
        match self {
            Self::String(s) => Some(s.clone()),
            Self::Object { version, .. } => Some(version.clone()),
            Self::Git {
                git, branch, rev, ..
            } => Some(Arc::from({
                let mut git_url = String::with_capacity(128);
                git_url.push_str(git);

                if let Some(branch) = branch {
                    git_url.push_str("?branch=");
                    git_url.push_str(branch);
                }

                if let Some(rev) = rev {
                    git_url.push('#');
                    git_url.push_str(rev);
                }

                git_url
            })),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CargoToml {
    dependencies: HashMap<String, DependencyValue>,
}

pub(crate) static DEPS: LazyLock<HashMap<String, DependencyValue>> = LazyLock::new(|| {
    CargoToml::dependencies().unwrap_or_else(|e| {
        unreachable!("Failed to parse Cargo.toml: {:#?}", e);
    })
});

impl CargoToml {
    fn dependencies() -> Result<HashMap<String, DependencyValue>, toml::de::Error> {
        let cargo_toml_raw = include_str!("../Cargo.toml");
        let cargo_toml: CargoToml = toml::from_str(cargo_toml_raw)?;

        Ok(cargo_toml
            .dependencies
            .into_iter()
            .map(|(k, v)| (k.replace("-", "_").to_string(), v))
            .collect())
    }
}

#[macro_export]
macro_rules! dep_name_with_version {
    ($dep:ty, $name:literal) => {{
        let dep_name = stringify!($dep).trim_matches('"');
        format!(
            "{dep_name}@{} - {}",
            $crate::deps::DEPS
                .get(stringify!($dep))
                .and_then($crate::deps::DependencyValue::version)
                .unwrap()
                .trim_start_matches('='),
            $name,
        )
    }};
    ($dep:ty) => {{
        format!(
            "{}@{}",
            stringify!($dep),
            $crate::deps::DEPS
                .get(stringify!($dep))
                .and_then($crate::deps::DependencyValue::version)
                .unwrap()
                .trim_start_matches('='),
        )
    }};
}
