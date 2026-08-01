//! REQ-130: module map inspection — §10.1 modules exist as crate modules.

#[test]
fn module_map_modules_are_linked() {
    // Bind a public item from each §10.1 module. Removing a `pub mod` or
    // renaming the symbol fails compile (not a string-table smoke check).
    let _ = (
        foundry::cli::run as fn() -> std::process::ExitCode,
        foundry::spec::field_name_is_denied as fn(&str) -> bool,
        foundry::catalog::STUB_CATALOG_DIGEST,
        foundry::resolve::CANONICAL_PROFILE_ORDER,
        foundry::plan::assert_path_jailed
            as fn(&str) -> Result<(), foundry::plan::ConstructError>,
        std::any::type_name::<foundry::render::RenderMap>(),
        std::any::type_name::<foundry::fsx::Admissibility>(),
        std::any::type_name::<foundry::generate::GenerateResult>(),
        foundry::verify::is_stripped_env_key as fn(&str) -> bool,
        foundry::report::format_error_json as fn(&str, &str) -> String,
    );
    assert_eq!(foundry::VERSION, env!("CARGO_PKG_VERSION"));
}
