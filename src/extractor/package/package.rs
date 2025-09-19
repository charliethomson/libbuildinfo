use cargo_metadata::MetadataCommand;
use serde::{Deserialize, Serialize};

use crate::extractor::{
    Extractor,
    package::{
        error::PackageError,
        partial_extractors::{
            AuthorsExtractor, CategoriesExtractor, DefaultRunExtractor, DescriptionExtractor,
            DocumentationExtractor, EditionExtractor, HomepageExtractor, KeywordsExtractor,
            LicenseExtractor, LicenseFileExtractor, LinksExtractor, MetadataExtractor,
            MsrvExtractor, NameExtractor, PackageSectionExtractor, PublishExtractor,
            ReadmeExtractor, RepositoryExtractor, VersionExtractor,
        },
    },
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Package {
    pub name: Option<String>,
    pub version: Option<String>,
    pub authors: Option<String>,
    pub edition: Option<String>,
    pub msrv: Option<String>,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub readme: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub license: Option<String>,
    pub license_file: Option<String>,
    pub keywords: Option<String>,
    pub categories: Option<String>,
    pub links: Option<String>,
    pub publish: Option<String>,
    pub metadata: Option<String>,
    pub default_run: Option<String>,
}

impl Extractor for Package {
    type Error = PackageError;

    fn extract() -> Result<Self, Self::Error> {
        let metadata = MetadataCommand::new()
            .exec()
            .map_err(|e| PackageError::Metadata {
                inner_error: e.into(),
            })?;
        let resolved_crates = metadata.resolve.ok_or(PackageError::Resolve)?;
        let root_id = resolved_crates.root.ok_or(PackageError::RootPackageId)?;
        let context = metadata
            .packages
            .iter()
            .find(|package| package.id == root_id)
            .ok_or(PackageError::RootPackage { id: root_id.repr })?
            .clone();

        Ok(Self {
            name: NameExtractor::extract(&context)?,
            version: VersionExtractor::extract(&context)?,
            authors: AuthorsExtractor::extract(&context)?,
            edition: EditionExtractor::extract(&context)?,
            msrv: MsrvExtractor::extract(&context)?,
            description: DescriptionExtractor::extract(&context)?,
            documentation: DocumentationExtractor::extract(&context)?,
            readme: ReadmeExtractor::extract(&context)?,
            homepage: HomepageExtractor::extract(&context)?,
            repository: RepositoryExtractor::extract(&context)?,
            license: LicenseExtractor::extract(&context)?,
            license_file: LicenseFileExtractor::extract(&context)?,
            keywords: KeywordsExtractor::extract(&context)?,
            categories: CategoriesExtractor::extract(&context)?,
            links: LinksExtractor::extract(&context)?,
            publish: PublishExtractor::extract(&context)?,
            metadata: MetadataExtractor::extract(&context)?,
            default_run: DefaultRunExtractor::extract(&context)?,
        })
    }
}
