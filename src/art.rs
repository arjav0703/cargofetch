use crate::Package;

pub fn output(package: &Package) {
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
