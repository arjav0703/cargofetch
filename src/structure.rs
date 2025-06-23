use serde::Deserialize;

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
struct Dependency {
    name: String,
    source: Option<String>,
    req: String,
    kind: Option<String>,
    rename: Option<String>,
    optional: bool,
    uses_default_features: bool,
    features: Vec<String>,
    target: Option<String>,
    path: Option<String>,
    registry: Option<String>,
    public: Option<bool>,
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
