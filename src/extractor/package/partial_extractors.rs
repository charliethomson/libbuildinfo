use crate::extractor::package::error::PackageError;

pub(in crate::extractor::package) trait PackageSectionExtractor {
    fn extract(context: &cargo_metadata::Package) -> Result<Option<String>, PackageError>;
}

macro_rules! define_extractor {
    (join, $extractor:ident, $path:tt) => {
        pub(in crate::extractor::package) struct $extractor;
        impl PackageSectionExtractor for $extractor {
            fn extract(context: &cargo_metadata::Package) -> Result<Option<String>, PackageError> {
                Ok(Some(context.$path.join(",")))
            }
        }
    };
    (join_opt, $extractor:ident, $path:tt) => {
        pub(in crate::extractor::package) struct $extractor;
        impl PackageSectionExtractor for $extractor {
            fn extract(context: &cargo_metadata::Package) -> Result<Option<String>, PackageError> {
                Ok(context.$path.as_ref().map(|f| f.join(",")))
            }
        }
    };
    (raw, $extractor:ident, $path:tt) => {
        pub(in crate::extractor::package) struct $extractor;
        impl PackageSectionExtractor for $extractor {
            fn extract(context: &cargo_metadata::Package) -> Result<Option<String>, PackageError> {
                Ok(Some(context.$path.to_string()))
            }
        }
    };
    (raw_opt, $extractor:ident, $path:tt) => {
        pub(in crate::extractor::package) struct $extractor;
        impl PackageSectionExtractor for $extractor {
            fn extract(context: &cargo_metadata::Package) -> Result<Option<String>, PackageError> {
                Ok(context.$path.as_ref().map(|f| f.to_string()))
            }
        }
    };
    (json, $extractor:ident, $path:tt) => {
        pub(in crate::extractor::package) struct $extractor;
        impl PackageSectionExtractor for $extractor {
            fn extract(context: &cargo_metadata::Package) -> Result<Option<String>, PackageError> {
                if (context.$path.is_null()) {
                    return Ok(None);
                }

                Ok(Some(serde_json::to_string(&context.$path).map_err(
                    |e| PackageError::Serialize {
                        inner_error: e.into(),
                    },
                )?))
            }
        }
    };
    (semver, $extractor:ident, $path:tt) => {
        pub(in crate::extractor::package) struct $extractor;
        impl PackageSectionExtractor for $extractor {
            fn extract(context: &cargo_metadata::Package) -> Result<Option<String>, PackageError> {
                // TODO: if u wanna, support non major.minor.patch fuckery
                Ok(context.$path.as_ref().map(|version| {
                    format!("{}.{}.{}", version.major, version.minor, version.patch)
                }))
            }
        }
    };
}

define_extractor!(join, AuthorsExtractor, authors);
define_extractor!(raw, NameExtractor, name);
define_extractor!(raw, VersionExtractor, version);
define_extractor!(raw, EditionExtractor, edition);
define_extractor!(semver, MsrvExtractor, rust_version);
define_extractor!(raw_opt, DescriptionExtractor, description);
define_extractor!(raw_opt, DocumentationExtractor, documentation);
define_extractor!(raw_opt, ReadmeExtractor, readme);
define_extractor!(raw_opt, HomepageExtractor, homepage);
define_extractor!(raw_opt, RepositoryExtractor, repository);
define_extractor!(raw_opt, LicenseExtractor, license);
define_extractor!(raw_opt, LicenseFileExtractor, license_file);
define_extractor!(join, KeywordsExtractor, keywords);
define_extractor!(join, CategoriesExtractor, categories);
define_extractor!(raw_opt, LinksExtractor, links);
define_extractor!(join_opt, PublishExtractor, publish);
define_extractor!(json, MetadataExtractor, metadata);
define_extractor!(raw_opt, DefaultRunExtractor, default_run);
