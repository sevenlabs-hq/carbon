use {
    crate::{
        accounts::{AccountsModTemplate, AccountsStructTemplate},
        events::EventsStructTemplate,
        handlers::codama::{
            processors::{
                process_codama_accounts, process_codama_defined_types, process_codama_instructions,
            },
            utils::{parse_event_hints, read_codama_idl},
        },
        instructions::{InstructionsModTemplate, InstructionsStructTemplate},
        types::TypeStructTemplate,
        util::is_big_array,
    },
    anyhow::{bail, Result},
    askama::Template,
    heck::{ToKebabCase, ToSnakeCase, ToUpperCamelCase},
    std::fs::{self},
};

pub fn parse_codama(
    path: String,
    output: String,
    as_crate: bool,
    event_hints: Option<String>,
) -> Result<()> {
    let (accounts_data, instructions_data, types_data, events_data, program_name) =
        match read_codama_idl(&path) {
            Ok(idl) => {
                let accounts_data = process_codama_accounts(&idl.program);
                let instructions_data = process_codama_instructions(&idl.program);

                let event_hints = parse_event_hints(event_hints);
                let (types_data, events_data) =
                    process_codama_defined_types(&idl.program, &event_hints);
                let program_name = idl.program.name;

                (
                    accounts_data,
                    instructions_data,
                    types_data,
                    events_data,
                    program_name,
                )
            }
            Err(error) => {
                bail!("Error parsing Codama IDL: {error}");
            }
        };

    let decoder_name = format!("{}Decoder", program_name.to_upper_camel_case());
    let decoder_name_kebab = program_name.to_kebab_case();
    let program_struct_name = format!("{}Account", program_name.to_upper_camel_case());
    let program_instruction_enum = format!("{}Instruction", program_name.to_upper_camel_case());

    let crate_dir = if output.ends_with("/") {
        if as_crate {
            format!("{}{}-decoder", output, decoder_name_kebab)
        } else {
            format!("{}{}_decoder", output, program_name.to_snake_case())
        }
    } else if as_crate {
        format!("{}/{}-decoder", output, decoder_name_kebab)
    } else {
        format!("{}/{}_decoder", output, program_name.to_snake_case())
    };

    fs::create_dir_all(&crate_dir).expect("Failed to create decoder directory");

    let src_dir = if as_crate {
        format!("{}/src", crate_dir)
    } else {
        crate_dir.clone()
    };

    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let needs_big_array = types_data.iter().any(|type_data| {
        type_data.fields.iter().any(|field| {
            field.rust_type.starts_with("[")
                && field.rust_type.ends_with("]")
                && is_big_array(&field.rust_type)
        })
    });

    // Generate types
    let types_dir = format!("{}/types", src_dir);
    fs::create_dir_all(&types_dir).expect("Failed to create types directory");

    for type_data in &types_data {
        let template = TypeStructTemplate { type_data };
        let rendered = template
            .render()
            .expect("Failed to render type struct template");
        let filename = format!("{}/{}.rs", types_dir, type_data.name.to_snake_case());
        fs::write(&filename, rendered).expect("Failed to write type struct file");
        println!("Generated {}", filename);
    }

    let types_mod_content = types_data
        .iter()
        .map(|type_data| {
            format!(
                "pub mod {};\npub use {}::*;",
                type_data.name.to_snake_case(),
                type_data.name.to_snake_case()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let types_mod_filename = format!("{}/mod.rs", types_dir);
    fs::write(&types_mod_filename, types_mod_content).expect("Failed to write types mod file");
    println!("Generated {}", types_mod_filename);

    // Generate Accounts

    let accounts_dir = format!("{}/accounts", src_dir);
    fs::create_dir_all(&accounts_dir).expect("Failed to create accounts directory");

    for account in &accounts_data {
        let template = AccountsStructTemplate { account };
        let rendered = template
            .render()
            .expect("Failed to render account struct template");
        let filename = format!("{}/{}.rs", accounts_dir, account.module_name);
        fs::write(&filename, rendered).expect("Failed to write account struct file");
        println!("Generated {}", filename);
    }

    let accounts_mod_template = AccountsModTemplate {
        accounts: &accounts_data,
        decoder_name: decoder_name.clone(),
        program_struct_name: program_struct_name.clone(),
    };
    let accounts_mod_rendered = accounts_mod_template
        .render()
        .expect("Failed to render accounts mod template");
    let accounts_mod_filename = format!("{}/mod.rs", accounts_dir);

    fs::write(&accounts_mod_filename, accounts_mod_rendered)
        .expect("Failed to write accounts mod file");
    println!("Generated {}", accounts_mod_filename);

    // Generate Instructions

    let instructions_dir = format!("{}/instructions", src_dir);
    fs::create_dir_all(&instructions_dir).expect("Failed to create instructions directory");

    for instruction in &instructions_data {
        let template = InstructionsStructTemplate { instruction };
        let rendered = template
            .render()
            .expect("Failed to render instruction struct template");
        let filename = format!("{}/{}.rs", instructions_dir, instruction.module_name);
        fs::write(&filename, rendered).expect("Failed to write instruction struct file");
        println!("Generated {}", filename);
    }

    for event in &events_data {
        let template = EventsStructTemplate { event };
        let rendered = template
            .render()
            .expect("Failed to render event struct template");
        let filename = format!("{}/{}.rs", instructions_dir, event.module_name);
        fs::write(&filename, rendered).expect("Failed to write event struct file");
        println!("Generated {}", filename);
    }

    let instructions_mod_template = InstructionsModTemplate {
        instructions: &instructions_data,
        decoder_name: decoder_name.clone(),
        program_instruction_enum: program_instruction_enum.clone(),
        events: &events_data,
    };
    let instructions_mod_rendered = instructions_mod_template
        .render()
        .expect("Failed to render instructions mod template");
    let instructions_mod_filename = format!("{}/mod.rs", instructions_dir);

    fs::write(&instructions_mod_filename, instructions_mod_rendered)
        .expect("Failed to write instructions mod file");

    println!("Generated {}", instructions_mod_filename);

    if as_crate {
        let lib_rs_content = format!(
            "pub struct {decoder_name};\npub mod accounts;\npub mod instructions;\npub mod types;",
            decoder_name = decoder_name
        );
        let lib_rs_filename = format!("{}/lib.rs", src_dir);
        fs::write(&lib_rs_filename, lib_rs_content).expect("Failed to write lib.rs file");
        println!("Generated {}", lib_rs_filename);

        let cargo_toml_content = format!(
            r#"[package]
name = "{decoder_name_kebab}-decoder"
version = "0.9.1"
edition = {{ workspace = true }}

[lib]
crate-type = ["rlib"]

[dependencies]
carbon-core = {{ workspace = true }}
carbon-proc-macros = {{ workspace = true }}
carbon-macros = {{ workspace = true }}
solana-account = {{ workspace = true }}
solana-instruction = {{ workspace = true }}
solana-pubkey = {{ workspace = true }}
serde = {{ workspace = true }}
{big_array}
"#,
            decoder_name_kebab = decoder_name_kebab,
            big_array = if needs_big_array {
                "serde-big-array = { workspace = true }"
            } else {
                ""
            }
        );
        let cargo_toml_filename = format!("{}/Cargo.toml", crate_dir);
        fs::write(&cargo_toml_filename, cargo_toml_content)
            .expect("Failed to write Cargo.toml file");
        println!("Generated {}", cargo_toml_filename);
    } else {
        let mod_rs_content = format!(
            "pub struct {decoder_name};\npub mod accounts;\npub mod instructions;\npub mod types;",
            decoder_name = decoder_name
        );
        let mod_rs_filename = format!("{}/mod.rs", src_dir);
        fs::write(&mod_rs_filename, mod_rs_content).expect("Failed to write mod.rs file");
        println!("Generated {}", mod_rs_filename);
    }

    Ok(())
}
