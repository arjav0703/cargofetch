use serde::Deserialize;
use std::process::Command;

pub fn get_meta() -> String {
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version=1")
        .output()
        .expect("Failed to execute command");
    abc();

    // this will return a String
    output_to_string(output)
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

fn abc() {
    let json_data = r#"
    {
        "packages": [
            {
                "name": "my-package",
                "version": "0.1.0",
                "id": "file:///path/to/my-package#0.1.0",
                "license": "MIT/Apache-2.0",
                "license_file": "LICENSE",
                "description": "Package description.",
                "source": null,
                "dependencies": [
                    {
                        "name": "bitflags",
                        "source": "registry+https://github.com/rust-lang/crates.io-index",
                        "req": "^1.0",
                        "kind": null,
                        "rename": null,
                        "optional": false,
                        "uses_default_features": true,
                        "features": [],
                        "target": "cfg(windows)",
                        "path": "/path/to/dep",
                        "registry": null,
                        "public": false
                    }
                ],
                "targets": [],
                "features": {
                    "default": ["feat1"],
                    "feat1": [],
                    "feat2": []
                },
                "manifest_path": "/path/to/my-package/Cargo.toml",
                "metadata": {
                    "docs": {
                        "rs": {
                            "all-features": true
                        }
                    }
                },
                "publish": ["crates-io"],
                "authors": ["Jane Doe <user@example.com>"],
                "categories": ["command-line-utilities"],
                "default_run": null,
                "rust_version": "1.56",
                "keywords": ["cli"],
                "readme": "README.md",
                "repository": "https://github.com/rust-lang/cargo",
                "homepage": "https://rust-lang.org",
                "documentation": "https://doc.rust-lang.org/stable/std",
                "edition": "2018",
                "links": null
            }
        ]
    }
    "#;

    let metadata: CargoMetadata = serde_json::from_str(json_data).unwrap();
    for package in &metadata.packages {
        println!("Package: {} v{}", package.name, package.version);
    }
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
    all_features: bool,
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
