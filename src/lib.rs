//! Info about the current version, git info, os info and rustc version

/// The collection of information
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Info {
    /// Pkg version
    pub version: semver::Version,
    /// Git hash
    pub git_version: &'static str,
    /// OS info
    pub os: os_info::Info,
    /// Rustc version used to compile
    pub rustc_version: semver::Version,
}

impl Info {
    /// Create [`Info`] using `version` and `git_version`
    ///
    /// Recommended to use `info!()` which wraps this.
    ///
    /// # Panics
    /// Panics if `version` does not parse as semver or `rustc_version` does not parse as semver
    pub fn new(version: &'static str, git_version: &'static str) -> Self {
        Self {
            version: version.parse().unwrap(),
            git_version,
            os: os_info::get(),
            rustc_version: env!("RUSTC_VERSION").parse().unwrap(),
        }
    }
}

impl std::fmt::Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            version,
            git_version,
            os,
            rustc_version,
        } = self;
        f.write_fmt(format_args!(
            "{version} {git_version} rustc-{rustc_version} {os}"
        ))
    }
}

#[doc(hidden)]
pub use git_version;

/// Get [`Info`] for the current pkg
///
/// # Panics
/// Panics if `version` does not parse as semver or `rustc_version` does not parse as semver
#[macro_export]
macro_rules! info {
    () => {
        $crate::Info::new(
            env!("CARGO_PKG_VERSION"),
            $crate::git_version::git_version!(fallback = "unknown"),
        )
    };
}

/// Lazy static for info string
///
/// # Panics
/// Panics if `version` does not parse as semver or `rustc_version` does not parse as semver
#[macro_export]
macro_rules! lazy_info_str {
    () => {{
        static INFO_STR: std::sync::LazyLock<String> =
            std::sync::LazyLock::new(|| $crate::info!().to_string());
        &*INFO_STR
    }};
}
