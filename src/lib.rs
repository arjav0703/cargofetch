use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CargoMetadata {
    packages: Vec<Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    version: String,
    id: String,
    license: Option<String>,
    license_file: Option<String>,
    description: Option<String>,
    source: Option<String>,
    dependencies: Vec<Dependency>,
    features: std::collections::HashMap<String, Vec<String>>,
    manifest_path: String,
    metadata: Option<PackageMetadata>,
    publish: Option<Vec<String>>,
    authors: Vec<String>,
    categories: Vec<String>,
    default_run: Option<String>,
    rust_version: Option<String>,
    keywords: Vec<String>,
    readme: Option<String>,
    repository: Option<String>,
    homepage: Option<String>,
    documentation: Option<String>,
    edition: String,
    links: Option<String>,
    targets: Vec<Target>,
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
struct PackageMetadata {
    docs: Option<DocsRsMetadata>,
}

#[derive(Debug, Deserialize)]
struct DocsRsMetadata {
    rs: Option<DocsRsAllFeatures>,
}

#[derive(Debug, Deserialize)]
struct DocsRsAllFeatures {
    #[serde(rename = "all-features")]
    all_features: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct Target {
    // Add fields as needed if using `targets`
}
//fn metad() {
//    for package in &metadata.packages {
//        println!("Package: {} v{}", package.name, package.version);
//    }
//}
