//! Info about the current cargo pkg version, git info, os info and rustc version
//!
//! # Example
//!
//! ```rust
//! use info::{Info, raw_info};
//!
//! let info = Info::new(raw_info!());
//! println!("{info}");
//! ```

use std::borrow::Cow;

/// Raw info without parsing that is compiled into the program
#[derive(Debug, Clone, Copy)]
pub struct RawInfo {
    /// Cargo pkg version, from the env var cargo sets `CARGO_PKG_VERSION`
    pub cargo_pkg_version: &'static str,
    /// Current git info
    pub git_version: &'static str,
    /// Rustc version used to compile the program
    pub rustc_version: &'static str,
}

/// The collection of information
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Info {
    /// Cargo pkg version, from the env var cargo sets `CARGO_PKG_VERSION`
    pub cargo_pkg_version: semver::Version,
    /// Current git info
    pub git_version: Cow<'static, str>,
    /// Rustc version used to compile the program
    pub rustc_version: semver::Version,
    /// Runtime information about the current operating system
    pub os: os_info::Info,
}

impl Info {
    /// Create [`Info`] using [`RawInfo`]
    ///
    /// It is recommended to construct [`RawInfo`] using [`raw_info!()`].
    ///
    /// # Example
    /// ```rust
    /// use info::{Info, raw_info};
    ///
    /// let info = Info::new(raw_info!());
    /// println!("{info}");
    /// ```
    ///
    /// # Panics
    /// Panics if `version` does not parse as semver or `rustc_version` does not parse as semver
    pub fn new(raw: RawInfo) -> Self {
        Self {
            cargo_pkg_version: raw.cargo_pkg_version.parse().unwrap(),
            git_version: Cow::Borrowed(raw.git_version),
            rustc_version: raw.rustc_version.parse().unwrap(),
            os: os_info::get(),
        }
    }
}

impl std::fmt::Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            cargo_pkg_version,
            git_version,
            os,
            rustc_version,
        } = self;
        f.write_fmt(format_args!(
            "{cargo_pkg_version} {git_version} rustc-{rustc_version} {os}"
        ))
    }
}

#[doc(hidden)]
pub use git_version;

/// Get [`RawInfo`] for the current pkg
#[macro_export]
macro_rules! raw_info {
    () => {
        $crate::RawInfo {
            cargo_pkg_version: env!("CARGO_PKG_VERSION"),
            git_version: $crate::git_version::git_version!(fallback = "unknown"),
            rustc_version: env!("RUSTC_VERSION"),
        }
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
