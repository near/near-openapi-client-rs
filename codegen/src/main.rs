//! Code generator for near-openapi-client and near-openapi-types
//!
//! This replaces the Python progenitor_fixes.py script with a Rust implementation.
//! Uses `syn` for proper AST manipulation instead of string manipulation.

use anyhow::{Context, Result};
use proc_macro2::TokenStream;
use serde_json::Value;
use std::fs;
use std::path::Path;
use syn::parse_file;

mod ast_transforms;
mod spec_fixes;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let no_fetch = args.iter().any(|a| a == "--no-fetch");

    // Step 1: Fetch or load the OpenAPI spec
    let spec_path = Path::new("openapi.json");

    if !no_fetch {
        println!("Fetching openapi.json from nearcore...");
        let output = std::process::Command::new("curl")
            .args([
                "-H",
                "Cache-Control: no-cache",
                "-s",
                "https://raw.githubusercontent.com/near/nearcore/refs/heads/master/chain/jsonrpc/openapi/openapi.json",
                "-o",
                "openapi.json",
            ])
            .output()
            .context("Failed to fetch openapi.json")?;

        if !output.status.success() {
            anyhow::bail!(
                "Failed to fetch openapi.json: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    // Step 2: Load and fix the spec
    println!("Loading and fixing OpenAPI spec...");
    let spec_content = fs::read_to_string(spec_path).context("Failed to read openapi.json")?;

    let mut spec: Value =
        serde_json::from_str(&spec_content).context("Failed to parse openapi.json")?;

    spec_fixes::fix_spec(&mut spec)?;

    // Write the fixed spec back
    fs::write(spec_path, serde_json::to_string_pretty(&spec)?)
        .context("Failed to write fixed openapi.json")?;

    // Step 3: Run progenitor
    println!("Running progenitor...");
    let output = std::process::Command::new("cargo")
        .args([
            "progenitor",
            "-i",
            "openapi.json",
            "-o",
            "near-openapi",
            "-n",
            "near-openapi",
            "-v",
            "0.0.0",
        ])
        .output()
        .context("Failed to run progenitor")?;

    println!("{}", String::from_utf8_lossy(&output.stdout));
    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        anyhow::bail!("Progenitor failed");
    }

    // Step 4: Parse and transform the generated code
    println!("Parsing and transforming generated code...");
    let lib_path = Path::new("near-openapi/src/lib.rs");
    let lib_content = fs::read_to_string(lib_path).context("Failed to read generated lib.rs")?;

    // Fix the doc comment bug before parsing (progenitor bug puts fn on same line as ///)
    let lib_content = fix_doc_comment_bug(&lib_content);

    // Parse with syn
    let ast = parse_file(&lib_content).context("Failed to parse generated lib.rs")?;

    // Transform and split
    let (types_tokens, client_tokens) = ast_transforms::split_and_transform(ast)?;

    // Format with prettyplease
    let types_code = format_tokens(types_tokens, false)?;
    let client_code = format_tokens(client_tokens, true)?;

    // Step 5: Write output crates
    println!("Writing output crates...");
    fs::create_dir_all("near-openapi-types/src")?;
    fs::create_dir_all("near-openapi-client/src")?;

    fs::write("near-openapi-types/src/lib.rs", types_code)?;
    fs::write("near-openapi-client/src/lib.rs", client_code)?;

    // Step 6: Format with rustfmt
    println!("Formatting output crates...");
    for crate_dir in ["near-openapi-types", "near-openapi-client"] {
        let _ = std::process::Command::new("cargo")
            .args(["fmt"])
            .current_dir(crate_dir)
            .status();
    }

    println!("Done!");
    Ok(())
}

/// Fix progenitor bug where doc comments are on same line as function def
fn fix_doc_comment_bug(content: &str) -> String {
    // Pattern: ///...text    pub async fn
    // Should be: ///...text\n    pub async fn
    let mut result = String::with_capacity(content.len() + 1000);
    let mut lines = content.lines().peekable();

    while let Some(line) = lines.next() {
        // Check if this line has /// followed by "    pub async fn" on the same line
        if line.trim_start().starts_with("///") {
            if let Some(idx) = line.find("    pub async fn") {
                // Split the line
                result.push_str(&line[..idx]);
                result.push('\n');
                result.push_str(&line[idx..]);
                result.push('\n');
                continue;
            }
        }
        result.push_str(line);
        result.push('\n');
    }

    result
}

fn format_tokens(tokens: TokenStream, is_client: bool) -> Result<String> {
    let file = syn::parse2::<syn::File>(tokens)?;
    let mut code = prettyplease::unparse(&file);

    // Fix NearToken constructor - progenitor generates the wrong constructor
    code = code.replace(
        r#"super::NearToken("0".to_string())"#,
        "super::NearToken::from_yoctonear(0)",
    );

    // For client code: NEAR RPC is JSON-RPC over POST to single endpoint, not REST
    // Replace all URL paths like "{}/block" with "{}/""
    if is_client {
        code = fix_client_urls(&code);
    }

    Ok(code)
}

/// Fix client URLs - NEAR RPC uses single endpoint, not REST-style paths
fn fix_client_urls(code: &str) -> String {
    // Replace patterns like `"{}/block"` with `"{}/"`
    // The regex in Python was: re.sub('\"{}/\\w*', '\"{}/', client_lib_rs)
    // Match "{}/someword" and replace with "{}/"
    let re = regex::Regex::new(r#""\{\}/\w+""#).unwrap();
    re.replace_all(code, r#""{}/""#).to_string()
}
