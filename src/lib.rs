use serde::Deserialize;
use std::process::Command;

pub fn get_meta() {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .output()
        .expect("Failed to execute command");

    jsonparser(output_to_string(output));
}

pub fn get_cargo_version() -> String {
    let output = Command::new("cargo")
        .arg("--version")
        .output()
        .expect("Failed to execute command");
    // this will return a String

    output_to_string(output)
        .split_whitespace()
        .nth(1)
        .unwrap_or("")
        .to_string()
}

fn output_to_string(output: std::process::Output) -> String {
    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        panic!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn jsonparser(json_data: String) {
    let metadata: CargoMetadata = serde_json::from_str(&json_data).expect("Failed to parse JSON");
    for package in &metadata.packages {
        output(package);
    }
}

fn output(package: &Package) {
    println!(
        "Package: {} v{}, {} dependencies: {}",
        package.name,
        package.version,
        package.repository.as_deref().unwrap_or(""),
        package
            .dependencies
            .iter()
            .map(|d| d.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    );
}

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
