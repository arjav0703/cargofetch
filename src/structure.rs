use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, ValueEnum, Clone, Serialize, Deserialize)]
pub enum ArtType {
    Crab,
    Rust,
}

impl Default for ArtType {
    fn default() -> Self {
        ArtType::Crab
    }
}

#[derive(Debug, Deserialize)]
pub struct CargoMetadata {
    pub packages: Vec<Package>,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub id: String,
    pub license: Option<String>,
    pub license_file: Option<String>,
    pub description: Option<String>,
    pub source: Option<String>,
    pub dependencies: Vec<Dependency>,
    pub features: std::collections::HashMap<String, Vec<String>>,
    pub manifest_path: String,
    pub metadata: Option<PackageMetadata>,
    pub publish: Option<Vec<String>>,
    pub authors: Vec<String>,
    pub categories: Vec<String>,
    pub default_run: Option<String>,
    pub rust_version: Option<String>,
    pub keywords: Vec<String>,
    pub readme: Option<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub edition: String,
    pub links: Option<String>,
    pub targets: Vec<Target>,
}

#[derive(Debug, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub source: Option<String>,
    pub req: String,
    pub kind: Option<String>,
    pub rename: Option<String>,
    pub optional: bool,
    pub uses_default_features: bool,
    pub features: Vec<String>,
    pub target: Option<String>,
    pub path: Option<String>,
    pub registry: Option<String>,
    pub public: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PackageMetadata {
    docs: Option<DocsRsMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct DocsRsMetadata {
    rs: Option<DocsRsAllFeatures>,
}

#[derive(Debug, Deserialize)]
pub struct DocsRsAllFeatures {
    #[serde(rename = "all-features")]
    all_features: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Target {
    // Add fields as needed if using `targets`
}
//fn metad() {
//    for package in &metadata.packages {
//        println!("Package: {} v{}", package.name, package.version);
//    }
//}
