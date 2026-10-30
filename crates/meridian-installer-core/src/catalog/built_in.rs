//! The catalog the Installer was built with.

use super::LoadError;
use crate::formats::parse_catalog_text;
use substrate_installer_domain::channel::Catalog;

/// `catalog/catalog.sdata`, as it was when the Installer was built.
const BUILT_IN: &str = include_str!("../../../../catalog/catalog.sdata");

/// The text of the built-in catalog.
#[must_use]
pub fn built_in_text() -> &'static str {
    BUILT_IN
}

/// The built-in catalog, read and checked.
///
/// # Errors
///
/// [`LoadError`] when the file built in is not a catalog (a build
/// that tests catch).
pub fn built_in() -> Result<Catalog, LoadError> {
    parse_catalog_text(BUILT_IN).map_err(LoadError::BuiltIn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_built_in_catalog_is_a_catalog_with_the_meridian_entries() {
        let catalog = built_in().expect("catalog/catalog.sdata is valid");
        assert!(catalog.installer().is_some());
        assert_eq!(catalog.products().len(), 1);
        assert_eq!(catalog.workloads().len(), 2);
        assert!(catalog.manifest().is_some());
        let ids: Vec<_> = catalog.components().iter().map(|c| c.id.as_str()).collect();
        for id in ["meridian.cli", "meridian.sdk", "rust.toolchain"] {
            assert!(ids.contains(&id), "{id} is in the catalog");
        }
    }
}

#[cfg(test)]
mod public_cli_tests {
    use substrate_installer_domain::sources::Source;

    #[test]
    fn cli_installation_uses_a_public_registry_instead_of_private_release_assets() {
        let catalog = super::built_in().expect("valid catalog");
        let cli = catalog
            .components()
            .iter()
            .find(|component| component.id.as_str() == "meridian.cli")
            .expect("CLI component");
        assert!(
            matches!(&cli.source, Source::CratesIo { name } if name.as_str() == "meridian-cli")
        );
    }
}
