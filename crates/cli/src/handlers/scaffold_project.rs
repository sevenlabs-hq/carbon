use std::{collections::HashSet, fs};

use heck::{ToSnakeCase, ToUpperCamelCase};

pub fn scaffold_project(
    program_name: String,
    output: String,
    decoders: String,
) -> anyhow::Result<()> {
    let crate_dir = if output.ends_with("/") {
        format!("{}-{}", output, program_name.to_snake_case())
    } else {
        format!("{}/{}", output, program_name.to_snake_case())
    };

    fs::create_dir_all(&crate_dir).expect("Failed to create decoder directory");

    let src_dir = format!("{}/src", crate_dir);

    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let decoders_set = parse_decoders(decoders);

    Ok(())
}

pub fn parse_decoders(decoders: String) -> HashSet<String> {
    decoders
        .split(',')
        .map(|s| s.trim().to_string().to_upper_camel_case())
        .filter(|s| !s.is_empty())
        .collect()
}
