from __future__ import annotations

import argparse
import re
from dataclasses import dataclass
from pathlib import Path
from textwrap import dedent


@dataclass
class AccountVariant:
    name: str
    source_type: str
    is_boxed: bool


@dataclass
class InstructionVariant:
    name: str
    source_type: str
    accounts_type: str


RUST_KEYWORDS = {
    "as",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "abstract",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "override",
    "priv",
    "typeof",
    "unsized",
    "virtual",
    "yield",
    "async",
    "await",
    "dyn",
    "try",
    "union",
    "macro_rules",
}

PATH_ROOT_KEYWORDS = {"crate", "self", "super", "Self"}


def escape_rust_keywords(value: str) -> str:
    def _replace(match: re.Match[str]) -> str:
        token = match.group(0)
        if token in PATH_ROOT_KEYWORDS:
            return token
        if token in RUST_KEYWORDS:
            return f"r#{token}"
        return token

    return re.sub(r"(?<!r#)\b[A-Za-z_][A-Za-z0-9_]*\b", _replace, value)


def snake_case(name: str) -> str:
    first = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", name)
    second = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", first)
    return second.replace("-", "_").lower()


def strip_carbon_prefix(name: str) -> str:
    return name.removeprefix("carbon-")


def decoder_prefix(decoder_dir: Path) -> str:
    return strip_carbon_prefix(decoder_dir.name.removesuffix("-decoder")).replace(
        "-", "_"
    )


def decoder_app_name(decoder_dir: Path) -> str:
    return strip_carbon_prefix(decoder_dir.name.removesuffix("-decoder"))


def enum_name(source: str) -> str:
    match = re.search(r"pub\s+enum\s+(\w+)", source)
    if not match:
        raise ValueError("Could not find enum name")
    return match.group(1)


def extract_enum_body(source: str, name: str) -> str:
    start = source.find(f"pub enum {name}")
    if start < 0:
        raise ValueError(f"Could not find enum {name}")

    brace_start = source.find("{", start)
    if brace_start < 0:
        raise ValueError(f"Could not find opening brace for enum {name}")

    depth = 0
    for index in range(brace_start, len(source)):
        char = source[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return source[brace_start + 1 : index]

    raise ValueError(f"Could not find closing brace for enum {name}")


def parse_account_variants(source: str) -> tuple[str, list[AccountVariant]]:
    name = enum_name(source)
    body = extract_enum_body(source, name)
    variants: list[AccountVariant] = []

    for match in re.finditer(r"(?ms)^\s*(\w+)\s*\(\s*(.+?)\s*\),\s*$", body):
        variant_name = match.group(1)
        inner = match.group(2).strip().rstrip(",").strip()
        is_boxed = False
        if inner.startswith("Box<") and inner.endswith(">"):
            inner = inner[4:-1].strip()
            is_boxed = True
        variants.append(AccountVariant(variant_name, inner, is_boxed))

    return name, variants


def parse_instruction_variants(source: str) -> tuple[str, list[InstructionVariant]]:
    name = enum_name(source)
    body = extract_enum_body(source, name)
    variants: list[InstructionVariant] = []

    for match in re.finditer(r"(?ms)^\s*(\w+)\s*\{\s*(.*?)\s*\},\s*$", body):
        variant_name = match.group(1)
        variant_body = match.group(2)
        data_match = re.search(r"\bdata:\s*([^,]+),", variant_body)
        accounts_match = re.search(r"\baccounts:\s*([^,]+),", variant_body)
        if not data_match or not accounts_match:
            raise ValueError(f"Could not parse instruction variant {variant_name}")
        variants.append(
            InstructionVariant(
                variant_name,
                data_match.group(1).strip(),
                accounts_match.group(1).strip(),
            )
        )

    return name, variants


def source_path_for_account(variant: AccountVariant) -> str:
    source = variant.source_type
    if source.startswith("crate::"):
        return escape_rust_keywords(source)
    return escape_rust_keywords(f"crate::accounts::{source}")


def source_path_for_instruction(variant: InstructionVariant) -> str:
    source = variant.source_type
    if source.startswith("crate::"):
        return escape_rust_keywords(source)
    return escape_rust_keywords(
        f"crate::instructions::{snake_case(variant.name)}::{source}"
    )


def render_account_row(
    decoder_prefix_value: str,
    variant: AccountVariant,
) -> tuple[str, str]:
    row_name = f"{variant.name}Row"
    table_base = f"{decoder_prefix_value}_{snake_case(variant.name)}_account"
    source_path = source_path_for_account(variant)

    content = dedent(
        f"""\
        //! This code was AUTOGENERATED using the Codama library.
        #[derive(clickhouse::Row, serde::Serialize, serde::Deserialize, Debug, Clone)]
        pub struct {row_name} {{
            pub __pubkey: String,
            pub __slot: u64,
            pub data: String,
        }}

        impl
            TryFrom<(
                {source_path},
                carbon_core::account::AccountMetadata,
            )> for {row_name}
        {{
            type Error = carbon_core::error::Error;

            fn try_from(
                (source, metadata): (
                    {source_path},
                    carbon_core::account::AccountMetadata,
                ),
            ) -> Result<Self, Self::Error> {{
                let data = serde_json::to_string(&source)
                    .map_err(|error| carbon_core::error::Error::Custom(error.to_string()))?;

                Ok(Self {{
                    __pubkey: metadata.pubkey.to_string(),
                    __slot: metadata.slot,
                    data,
                }})
            }}
        }}

        impl TryFrom<{row_name}> for ({source_path}, carbon_core::account::AccountMetadata) {{
            type Error = carbon_core::error::Error;

            fn try_from(value: {row_name}) -> Result<Self, Self::Error> {{
                let pubkey = value
                    .__pubkey
                    .parse::<solana_pubkey::Pubkey>()
                    .map_err(|error| carbon_core::error::Error::Custom(error.to_string()))?;
                let source: {source_path} = serde_json::from_str(&value.data)
                    .map_err(|error| carbon_core::error::Error::Custom(error.to_string()))?;

                Ok((
                    source,
                    carbon_core::account::AccountMetadata {{
                        slot: value.__slot,
                        pubkey,
                        transaction_signature: None,
                    }},
                ))
            }}
        }}

        #[cfg(feature = "clickhouse-cluster")]
        impl carbon_core::clickhouse::ClusterTable for {row_name} {{
            fn local_table() -> &'static str {{
                "{table_base}_local"
            }}

            fn distributed_table() -> &'static str {{
                "{table_base}"
            }}
        }}

        #[cfg(not(feature = "clickhouse-cluster"))]
        impl carbon_core::clickhouse::Table for {row_name} {{
            fn table() -> &'static str {{
                "{table_base}"
            }}
        }}

        #[async_trait::async_trait]
        impl carbon_core::clickhouse::Insert for {row_name} {{
            async fn insert(
                client: &clickhouse::Client,
                rows: &[Self],
            ) -> carbon_core::error::CarbonResult<()> {{
                if rows.is_empty() {{
                    return Ok(());
                }}

                let mut insert = client
                    .insert::<Self>("{table_base}")
                    .await
                    .map_err(|error| carbon_core::error::Error::Custom(error.to_string()))?;

                for row in rows {{
                    insert
                        .write(row)
                        .await
                        .map_err(|error| carbon_core::error::Error::Custom(error.to_string()))?;
                }}

                insert
                    .end()
                    .await
                    .map_err(|error| carbon_core::error::Error::Custom(error.to_string()))?;
                Ok(())
            }}
        }}

        pub struct {row_name}MigrationOperation;

        #[cfg(feature = "clickhouse-cluster")]
        #[async_trait::async_trait]
        impl carbon_core::clickhouse::Operation for {row_name}MigrationOperation {{
            async fn up(
                &self,
                client: &clickhouse::Client,
                config: &carbon_core::clickhouse::MigrationConfig,
            ) -> clickhouse::error::Result<()> {{
                let cluster_name = &config.cluster_name;
                let replica_path = format!(
                    "/clickhouse/tables/{{}}/{table_base}_local",
                    config.shard_name,
                );

                client
                    .query(&format!(
                        r#"
                            CREATE TABLE IF NOT EXISTS {table_base}_local ON CLUSTER {{cluster}} (
                                __pubkey String,
                                __slot UInt64,
                                data String
                            )
                            ENGINE = ReplicatedReplacingMergeTree('{{replica_path}}', '{{{{replica}}}}')
                            ORDER BY (__pubkey)
                        "#,
                        cluster = cluster_name,
                        replica_path = replica_path,
                    ))
                    .execute()
                    .await?;

                client
                    .query(&format!(
                        r#"
                            CREATE TABLE IF NOT EXISTS {table_base} ON CLUSTER {{cluster}} AS {table_base}_local
                            ENGINE = Distributed({{cluster}}, currentDatabase(), {table_base}_local, rand())
                        "#,
                        cluster = cluster_name,
                    ))
                    .execute()
                    .await?;

                Ok(())
            }}

            async fn down(
                &self,
                client: &clickhouse::Client,
                config: &carbon_core::clickhouse::MigrationConfig,
            ) -> clickhouse::error::Result<()> {{
                let cluster_name = &config.cluster_name;

                client
                    .query(&format!(
                        "DROP TABLE IF EXISTS {table_base} ON CLUSTER {{cluster}}",
                        cluster = cluster_name,
                    ))
                    .execute()
                    .await?;

                client
                    .query(&format!(
                        "DROP TABLE IF EXISTS {table_base}_local ON CLUSTER {{cluster}}",
                        cluster = cluster_name,
                    ))
                    .execute()
                    .await?;

                Ok(())
            }}
        }}

        #[cfg(not(feature = "clickhouse-cluster"))]
        #[async_trait::async_trait]
        impl carbon_core::clickhouse::Operation for {row_name}MigrationOperation {{
            async fn up(&self, client: &clickhouse::Client) -> clickhouse::error::Result<()> {{
                client
                    .query(
                        r#"
                            CREATE TABLE IF NOT EXISTS {table_base} (
                                __pubkey String,
                                __slot UInt64,
                                data String
                            )
                            ENGINE = ReplacingMergeTree()
                            ORDER BY (__pubkey)
                        "#,
                    )
                    .execute()
                    .await?;

                Ok(())
            }}

            async fn down(&self, client: &clickhouse::Client) -> clickhouse::error::Result<()> {{
                client
                    .query("DROP TABLE IF EXISTS {table_base}")
                    .execute()
                    .await?;

                Ok(())
            }}
        }}
        """
    )
    return row_name, content


def render_instruction_row(
    decoder_prefix_value: str,
    variant: InstructionVariant,
) -> tuple[str, str]:
    row_name = f"{variant.name}Row"
    table_base = f"{decoder_prefix_value}_{snake_case(variant.name)}_instruction"
    source_path = source_path_for_instruction(variant)
    accounts_type = variant.accounts_type
    if not accounts_type.startswith("crate::"):
        accounts_type = (
            f"crate::instructions::{snake_case(variant.name)}::{accounts_type}"
        )
    accounts_type = escape_rust_keywords(accounts_type)

    content = dedent(
        f"""\
        //! This code was AUTOGENERATED using the Codama library.
        #[derive(clickhouse::Row, serde::Serialize, serde::Deserialize, Debug, Clone)]
        pub struct {row_name} {{
            pub __signature: String,
            pub __instruction_index: u32,
            pub __stack_height: u32,
            pub __slot: u64,
            pub __accounts: String,
            pub data: String,
        }}

        impl
            TryFrom<(
                {source_path},
                carbon_core::instruction::InstructionMetadata,
                {accounts_type},
            )> for {row_name}
        {{
            type Error = carbon_core::error::Error;

            fn try_from(
                (source, metadata, accounts): (
                    {source_path},
                    carbon_core::instruction::InstructionMetadata,
                    {accounts_type},
                ),
            ) -> Result<Self, Self::Error> {{
                let accounts = serde_json::to_string(&accounts)
                    .map_err(|error| carbon_core::error::Error::Custom(error.to_string()))?;
                let data = serde_json::to_string(&source)
                    .map_err(|error| carbon_core::error::Error::Custom(error.to_string()))?;

                Ok(Self {{
                    __signature: metadata.transaction_metadata.signature.to_string(),
                    __instruction_index: metadata.index,
                    __stack_height: metadata.stack_height,
                    __slot: metadata.transaction_metadata.slot,
                    __accounts: accounts,
                    data,
                }})
            }}
        }}

        impl TryFrom<{row_name}> for ({source_path}, {accounts_type}, {row_name}) {{
            type Error = carbon_core::error::Error;

            fn try_from(value: {row_name}) -> Result<Self, Self::Error> {{
                let source: {source_path} = serde_json::from_str(&value.data)
                    .map_err(|error| carbon_core::error::Error::Custom(error.to_string()))?;
                let accounts: {accounts_type} = serde_json::from_str(&value.__accounts)
                    .map_err(|error| carbon_core::error::Error::Custom(error.to_string()))?;

                Ok((
                    source,
                    accounts,
                    value,
                ))
            }}
        }}

        #[cfg(feature = "clickhouse-cluster")]
        impl carbon_core::clickhouse::ClusterTable for {row_name} {{
            fn local_table() -> &'static str {{
                "{table_base}_local"
            }}

            fn distributed_table() -> &'static str {{
                "{table_base}"
            }}
        }}

        #[cfg(not(feature = "clickhouse-cluster"))]
        impl carbon_core::clickhouse::Table for {row_name} {{
            fn table() -> &'static str {{
                "{table_base}"
            }}
        }}

        #[async_trait::async_trait]
        impl carbon_core::clickhouse::Insert for {row_name} {{
            async fn insert(
                client: &clickhouse::Client,
                rows: &[Self],
            ) -> carbon_core::error::CarbonResult<()> {{
                if rows.is_empty() {{
                    return Ok(());
                }}

                let mut insert = client
                    .insert::<Self>("{table_base}")
                    .await
                    .map_err(|error| carbon_core::error::Error::Custom(error.to_string()))?;

                for row in rows {{
                    insert
                        .write(row)
                        .await
                        .map_err(|error| carbon_core::error::Error::Custom(error.to_string()))?;
                }}

                insert
                    .end()
                    .await
                    .map_err(|error| carbon_core::error::Error::Custom(error.to_string()))?;
                Ok(())
            }}
        }}

        pub struct {row_name}MigrationOperation;

        #[cfg(feature = "clickhouse-cluster")]
        #[async_trait::async_trait]
        impl carbon_core::clickhouse::Operation for {row_name}MigrationOperation {{
            async fn up(
                &self,
                client: &clickhouse::Client,
                config: &carbon_core::clickhouse::MigrationConfig,
            ) -> clickhouse::error::Result<()> {{
                let cluster_name = &config.cluster_name;
                let replica_path = format!(
                    "/clickhouse/tables/{{}}/{table_base}_local",
                    config.shard_name,
                );

                client
                    .query(&format!(
                        r#"
                            CREATE TABLE IF NOT EXISTS {table_base}_local ON CLUSTER {{cluster}} (
                                __signature String,
                                __instruction_index UInt32,
                                __stack_height UInt32,
                                __slot UInt64,
                                __accounts String,
                                data String
                            )
                            ENGINE = ReplicatedReplacingMergeTree('{{replica_path}}', '{{{{replica}}}}')
                            ORDER BY (__signature, __instruction_index, __stack_height)
                        "#,
                        cluster = cluster_name,
                        replica_path = replica_path,
                    ))
                    .execute()
                    .await?;

                client
                    .query(&format!(
                        r#"
                            CREATE TABLE IF NOT EXISTS {table_base} ON CLUSTER {{cluster}} AS {table_base}_local
                            ENGINE = Distributed({{cluster}}, currentDatabase(), {table_base}_local, rand())
                        "#,
                        cluster = cluster_name,
                    ))
                    .execute()
                    .await?;

                Ok(())
            }}

            async fn down(
                &self,
                client: &clickhouse::Client,
                config: &carbon_core::clickhouse::MigrationConfig,
            ) -> clickhouse::error::Result<()> {{
                let cluster_name = &config.cluster_name;

                client
                    .query(&format!(
                        "DROP TABLE IF EXISTS {table_base} ON CLUSTER {{cluster}}",
                        cluster = cluster_name,
                    ))
                    .execute()
                    .await?;

                client
                    .query(&format!(
                        "DROP TABLE IF EXISTS {table_base}_local ON CLUSTER {{cluster}}",
                        cluster = cluster_name,
                    ))
                    .execute()
                    .await?;

                Ok(())
            }}
        }}

        #[cfg(not(feature = "clickhouse-cluster"))]
        #[async_trait::async_trait]
        impl carbon_core::clickhouse::Operation for {row_name}MigrationOperation {{
            async fn up(&self, client: &clickhouse::Client) -> clickhouse::error::Result<()> {{
                client
                    .query(
                        r#"
                            CREATE TABLE IF NOT EXISTS {table_base} (
                                __signature String,
                                __instruction_index UInt32,
                                __stack_height UInt32,
                                __slot UInt64,
                                __accounts String,
                                data String
                            )
                            ENGINE = ReplacingMergeTree()
                            ORDER BY (__signature, __instruction_index, __stack_height)
                        "#,
                    )
                    .execute()
                    .await?;

                Ok(())
            }}

            async fn down(&self, client: &clickhouse::Client) -> clickhouse::error::Result<()> {{
                client
                    .query("DROP TABLE IF EXISTS {table_base}")
                    .execute()
                    .await?;

                Ok(())
            }}
        }}
        """
    )
    return row_name, content


def render_account_mod(
    app_name: str,
    enum_name_value: str,
    variants: list[AccountVariant],
) -> str:
    base_name = enum_name_value.removesuffix("Account")

    lines: list[str] = []
    lines.append("//! This code was AUTOGENERATED using the Codama library.")

    if variants:
        for variant in variants:
            lines.append(f"pub mod {snake_case(variant.name)}_row;")
        lines.append("")
        for variant in variants:
            lines.append(f"pub use self::{snake_case(variant.name)}_row::*;")
        lines.append("")

    lines.append(f"use super::{base_name}Account;")
    lines.append("")
    lines.append(f"pub struct {base_name}AccountsMigration;")
    lines.append("")
    lines.append(
        f"impl carbon_core::clickhouse::Migration for {base_name}AccountsMigration {{"
    )
    lines.append("    fn app(&self) -> &str {")
    lines.append(f'        "{app_name}"')
    lines.append("    }")
    lines.append("")
    lines.append("    fn name(&self) -> &str {")
    lines.append(f'        "{snake_case(base_name)}_accounts"')
    lines.append("    }")
    lines.append("")
    lines.append(
        "    fn operations(&self) -> Vec<Box<dyn carbon_core::clickhouse::Operation>> {"
    )
    if not variants:
        lines.append("        vec![]")
    else:
        lines.append("        vec![")
        for variant in variants:
            lines.append(f"            Box::new({variant.name}RowMigrationOperation),")
        lines.append("        ]")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    lines.append(f"pub enum {base_name}AccountRow {{")
    for variant in variants:
        lines.append(f"    {variant.name}({variant.name}Row),")
    lines.append("}")
    lines.append("")

    lines.append(f"pub struct {base_name}AccountMetadata<'a>(")
    lines.append("    pub &'a carbon_core::account::AccountMetadata,")
    lines.append(f"    pub &'a {base_name}Account,")
    lines.append(");")
    lines.append("")

    lines.append(
        f"impl <'a> carbon_core::clickhouse::BatchInsert for {base_name}AccountMetadata<'a> {{"
    )
    lines.append(f"    type Row = {base_name}AccountRow;")
    lines.append("")
    lines.append("    fn batch_insert(")
    lines.append("        &self,")
    lines.append("        rows: &mut Vec<Self::Row>,")
    lines.append("    ) -> carbon_core::error::CarbonResult<()> {")
    if not variants:
        lines.append("        let _ = rows;")
        lines.append("        let Self(_metadata, _account) = self;")
        lines.append(
            '        unreachable!("BatchInsert called for program with no account row variants");'
        )
    else:
        lines.append("        let &Self(metadata, account) = self;")
        lines.append("")
        lines.append("        macro_rules! insert_branch {")
        lines.append("            ($variant:ident, $row:ty, boxed) => {")
        lines.append(
            "                if let "
            + f"{base_name}Account"
            + "::$variant(account) = account {"
        )
        lines.append(
            "                    rows.push("
            + f"{base_name}AccountRow"
            + "::$variant(<$row>::try_from((account.as_ref().clone(), metadata.clone()))?));"
        )
        lines.append("                    return Ok(());")
        lines.append("                }")
        lines.append("            };")
        lines.append("            ($variant:ident, $row:ty, plain) => {")
        lines.append(
            "                if let "
            + f"{base_name}Account"
            + "::$variant(account) = account {"
        )
        lines.append(
            "                    rows.push("
            + f"{base_name}AccountRow"
            + "::$variant(<$row>::try_from((account.clone(), metadata.clone()))?));"
        )
        lines.append("                    return Ok(());")
        lines.append("                }")
        lines.append("            };")
        lines.append("        }")
        lines.append("")
        for variant in variants:
            mode = "boxed" if variant.is_boxed else "plain"
            lines.append(
                f"        insert_branch!({variant.name}, {variant.name}Row, {mode});"
            )
        lines.append("")
        lines.append("        Ok(())")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    lines.append("#[async_trait::async_trait]")
    lines.append(
        f"impl carbon_core::clickhouse::BatchCommit for {base_name}AccountRow {{"
    )
    lines.append("    async fn batch_commit(")
    lines.append("        client: &clickhouse::Client,")
    lines.append("        rows: &[Self],")
    lines.append("    ) -> carbon_core::error::CarbonResult<()> {")
    if not variants:
        lines.append("        let _ = (client, rows);")
        lines.append(
            '        unreachable!("BatchCommit called for program with no account row variants");'
        )
    else:
        lines.append("        macro_rules! commit_branch {")
        lines.append("            ($variant:ident, $row:ty) => {")
        lines.append("                {")
        lines.append("                    let branch_rows: Vec<$row> = rows")
        lines.append("                        .iter()")
        lines.append("                        .filter_map(|row| match row {")
        lines.append(
            "                            Self::$variant(row) => Some(row.clone()),"
        )
        lines.append("                            _ => None,")
        lines.append("                        })")
        lines.append("                        .collect();")
        lines.append("")
        lines.append("                    if !branch_rows.is_empty() {")
        lines.append(
            "                        <$row as carbon_core::clickhouse::Insert>::insert(client, &branch_rows).await?;"
        )
        lines.append("                    }")
        lines.append("                }")
        lines.append("            };")
        lines.append("        }")
        lines.append("")
        for variant in variants:
            lines.append(f"        commit_branch!({variant.name}, {variant.name}Row);")
        lines.append("")
        lines.append("        Ok(())")
    lines.append("    }")
    lines.append("}")

    return "\n".join(lines)


def render_instruction_mod(
    app_name: str,
    enum_name_value: str,
    variants: list[InstructionVariant],
) -> str:
    base_name = enum_name_value.removesuffix("Instruction")

    lines: list[str] = []
    lines.append("//! This code was AUTOGENERATED using the Codama library.")

    if variants:
        for variant in variants:
            lines.append(f"pub mod {snake_case(variant.name)}_row;")
        lines.append("")
        for variant in variants:
            lines.append(f"pub use self::{snake_case(variant.name)}_row::*;")
        lines.append("")

    lines.append(f"use super::{base_name}Instruction;")
    lines.append("")
    lines.append(f"pub struct {base_name}InstructionsMigration;")
    lines.append("")
    lines.append(
        f"impl carbon_core::clickhouse::Migration for {base_name}InstructionsMigration {{"
    )
    lines.append("    fn app(&self) -> &str {")
    lines.append(f'        "{app_name}"')
    lines.append("    }")
    lines.append("")
    lines.append("    fn name(&self) -> &str {")
    lines.append(f'        "{snake_case(base_name)}_instructions"')
    lines.append("    }")
    lines.append("")
    lines.append(
        "    fn operations(&self) -> Vec<Box<dyn carbon_core::clickhouse::Operation>> {"
    )
    if not variants:
        lines.append("        vec![]")
    else:
        lines.append("        vec![")
        for variant in variants:
            lines.append(f"            Box::new({variant.name}RowMigrationOperation),")
        lines.append("        ]")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    lines.append(f"pub enum {base_name}InstructionRow {{")
    for variant in variants:
        lines.append(f"    {variant.name}({variant.name}Row),")
    lines.append("}")
    lines.append("")

    lines.append(f"pub struct {base_name}InstructionMetadata<'a>(")
    lines.append("    pub &'a carbon_core::instruction::InstructionMetadata,")
    lines.append(f"    pub &'a {base_name}Instruction,")
    lines.append(");")
    lines.append("")

    lines.append(
        f"impl <'a> carbon_core::clickhouse::BatchInsert for {base_name}InstructionMetadata<'a> {{"
    )
    lines.append(f"    type Row = {base_name}InstructionRow;")
    lines.append("")
    lines.append("    fn batch_insert(")
    lines.append("        &self,")
    lines.append("        rows: &mut Vec<Self::Row>,")
    lines.append("    ) -> carbon_core::error::CarbonResult<()> {")
    if not variants:
        lines.append("        let _ = rows;")
        lines.append("        let Self(_metadata, _instruction) = self;")
        lines.append(
            '        unreachable!("BatchInsert called for program with no instruction row variants");'
        )
    else:
        lines.append("        let &Self(metadata, instruction) = self;")
        lines.append("")
        lines.append("        macro_rules! insert_branch {")
        lines.append("            ($variant:ident, $row:ty) => {")
        lines.append(
            "                if let "
            + f"{base_name}Instruction"
            + "::$variant { data, accounts, .. } = instruction {"
        )
        lines.append(
            "                    rows.push("
            + f"{base_name}InstructionRow"
            + "::$variant(<$row>::try_from((data.clone(), metadata.clone(), accounts.clone()))?));"
        )
        lines.append("                    return Ok(());")
        lines.append("                }")
        lines.append("            };")
        lines.append("        }")
        lines.append("")
        for variant in variants:
            lines.append(f"        insert_branch!({variant.name}, {variant.name}Row);")
        lines.append("")
        lines.append("        Ok(())")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    lines.append("#[async_trait::async_trait]")
    lines.append(
        f"impl carbon_core::clickhouse::BatchCommit for {base_name}InstructionRow {{"
    )
    lines.append("    async fn batch_commit(")
    lines.append("        client: &clickhouse::Client,")
    lines.append("        rows: &[Self],")
    lines.append("    ) -> carbon_core::error::CarbonResult<()> {")
    if not variants:
        lines.append("        let _ = (client, rows);")
        lines.append(
            '        unreachable!("BatchCommit called for program with no instruction row variants");'
        )
    else:
        lines.append("        macro_rules! commit_branch {")
        lines.append("            ($variant:ident, $row:ty) => {")
        lines.append("                {")
        lines.append("                    let branch_rows: Vec<$row> = rows")
        lines.append("                        .iter()")
        lines.append("                        .filter_map(|row| match row {")
        lines.append(
            "                            Self::$variant(row) => Some(row.clone()),"
        )
        lines.append("                            _ => None,")
        lines.append("                        })")
        lines.append("                        .collect();")
        lines.append("")
        lines.append("                    if !branch_rows.is_empty() {")
        lines.append(
            "                        <$row as carbon_core::clickhouse::Insert>::insert(client, &branch_rows).await?;"
        )
        lines.append("                    }")
        lines.append("                }")
        lines.append("            };")
        lines.append("        }")
        lines.append("")
        for variant in variants:
            lines.append(f"        commit_branch!({variant.name}, {variant.name}Row);")
        lines.append("")
        lines.append("        Ok(())")
    lines.append("    }")
    lines.append("}")

    return "\n".join(lines)


def normalize_generated_content(content: str) -> str:
    content = content.lstrip("\n")
    lines = [line.rstrip() for line in content.splitlines()]

    while lines and lines[0].strip() == "":
        _ = lines.pop(0)

    if lines:
        lines[0] = lines[0].lstrip()

    return "\n".join(lines) + "\n"


def ensure_clickhouse_exports(mod_path: Path) -> bool:
    source = mod_path.read_text()
    original = source
    lines = source.splitlines()

    filtered: list[str] = []

    for line in lines:
        stripped = line.strip()
        if stripped in {
            '#[cfg(feature = "clickhouse")]',
            "pub mod clickhouse;",
            ";",
        }:
            continue
        filtered.append(line)

    insert_at = len(filtered)
    for i, line in enumerate(filtered):
        stripped = line.strip()
        if stripped.startswith(
            ('#[cfg(feature = "postgres")', '#[cfg(feature = "graphql")')
        ):
            insert_at = i
            break
        if stripped.startswith("pub mod "):
            insert_at = i
            break
        if stripped.startswith(("#[derive", "pub enum ")):
            insert_at = i
            break

    block = [
        '#[cfg(feature = "clickhouse")]',
        "pub mod clickhouse;",
        "",
    ]
    filtered[insert_at:insert_at] = block

    normalized = normalize_generated_content("\n".join(filtered))
    if normalized != normalize_generated_content(original):
        _ = mod_path.write_text(normalized)
        return True

    return False


def generate_decoder(decoder_dir: Path, overwrite: bool) -> list[Path]:
    generated: list[Path] = []
    prefix = decoder_prefix(decoder_dir)
    app_name = decoder_app_name(decoder_dir)

    accounts_mod = decoder_dir / "src" / "accounts" / "mod.rs"
    if accounts_mod.exists():
        if ensure_clickhouse_exports(accounts_mod):
            generated.append(accounts_mod)
        source = accounts_mod.read_text()
        accounts_enum_name, account_variants = parse_account_variants(source)
        clickhouse_dir = decoder_dir / "src" / "accounts" / "clickhouse"
        clickhouse_dir.mkdir(parents=True, exist_ok=True)

        mod_path = clickhouse_dir / "mod.rs"
        if overwrite or not mod_path.exists():
            _ = mod_path.write_text(
                normalize_generated_content(
                    render_account_mod(app_name, accounts_enum_name, account_variants)
                )
            )
            generated.append(mod_path)

        for variant in account_variants:
            _, row_content = render_account_row(prefix, variant)
            row_path = clickhouse_dir / f"{snake_case(variant.name)}_row.rs"
            if overwrite or not row_path.exists():
                _ = row_path.write_text(normalize_generated_content(row_content))
                generated.append(row_path)

    instructions_mod = decoder_dir / "src" / "instructions" / "mod.rs"
    if instructions_mod.exists():
        if ensure_clickhouse_exports(instructions_mod):
            generated.append(instructions_mod)
        source = instructions_mod.read_text()
        instructions_enum_name, instruction_variants = parse_instruction_variants(
            source
        )
        clickhouse_dir = decoder_dir / "src" / "instructions" / "clickhouse"
        clickhouse_dir.mkdir(parents=True, exist_ok=True)

        mod_path = clickhouse_dir / "mod.rs"
        if overwrite or not mod_path.exists():
            _ = mod_path.write_text(
                normalize_generated_content(
                    render_instruction_mod(
                        app_name, instructions_enum_name, instruction_variants
                    )
                )
            )
            generated.append(mod_path)

        for variant in instruction_variants:
            _, row_content = render_instruction_row(prefix, variant)
            row_path = clickhouse_dir / f"{snake_case(variant.name)}_row.rs"
            if overwrite or not row_path.exists():
                _ = row_path.write_text(normalize_generated_content(row_content))
                generated.append(row_path)

    return generated


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Generate ClickHouse adapters from pre-generated decoder source.",
    )
    _ = parser.add_argument(
        "decoders",
        nargs="*",
        help="Decoder directories to process (defaults to every decoder under ./decoders).",
    )
    _ = parser.add_argument(
        "--overwrite",
        action="store_true",
        help="Overwrite existing ClickHouse files instead of leaving them untouched.",
    )
    args = parser.parse_args()

    if args.decoders:  # pyright: ignore[reportAny]
        decoder_dirs = [Path(path) for path in args.decoders]  # pyright: ignore[reportAny]
    else:
        decoder_dirs = sorted(
            [
                path
                for path in Path("decoders").iterdir()
                if path.is_dir() and path.name.endswith("-decoder")
            ],
            key=lambda path: path.name,
        )

    total = 0
    for decoder_dir in decoder_dirs:
        generated = generate_decoder(decoder_dir, overwrite=args.overwrite)  # pyright: ignore[reportAny]
        if generated:
            total += len(generated)
            print(f"{decoder_dir}: generated {len(generated)} file(s)")

    print(f"Total generated files: {total}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
