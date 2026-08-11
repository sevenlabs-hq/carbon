#!/usr/bin/env python3

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
    return escape_rust_keywords(f"crate::instructions::{snake_case(variant.name)}::{source}")


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
                &self,
                client: &clickhouse::Client,
                rows: &[Self],
            ) -> carbon_core::error::CarbonResult<()> {{
                if rows.is_empty() {{
                    return Ok(());
                }}

                #[cfg(feature = "clickhouse-cluster")]
                let table = <Self as carbon_core::clickhouse::ClusterTable>::distributed_table();
                #[cfg(not(feature = "clickhouse-cluster"))]
                let table = <Self as carbon_core::clickhouse::Table>::table();

                let mut insert = client
                    .insert::<Self>(table)
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
                &self,
                client: &clickhouse::Client,
                rows: &[Self],
            ) -> carbon_core::error::CarbonResult<()> {{
                if rows.is_empty() {{
                    return Ok(());
                }}

                #[cfg(feature = "clickhouse-cluster")]
                let table = <Self as carbon_core::clickhouse::ClusterTable>::distributed_table();
                #[cfg(not(feature = "clickhouse-cluster"))]
                let table = <Self as carbon_core::clickhouse::Table>::table();

                let mut insert = client
                    .insert::<Self>(table)
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

    if not variants:
        return dedent(
            f"""\
            //! This code was AUTOGENERATED using the Codama library.
            use super::{base_name}Account;

            pub struct {base_name}AccountsMigration;

            impl carbon_core::clickhouse::Migration for {base_name}AccountsMigration {{
                fn app(&self) -> &str {{
                    "{app_name}"
                }}

                fn name(&self) -> &str {{
                    "{snake_case(base_name)}_accounts"
                }}

                fn operations(&self) -> Vec<Box<dyn carbon_core::clickhouse::Operation>> {{
                    vec![]
                }}
            }}

            pub enum {base_name}AccountRow {{}}

            pub struct {base_name}AccountMetadata(
                pub carbon_core::account::AccountMetadata,
                pub {base_name}Account,
            );

            #[async_trait::async_trait]
            impl carbon_core::clickhouse::BatchInsert for {base_name}AccountMetadata {{
                type Row = {base_name}AccountRow;

                async fn batch_insert(
                    &self,
                    rows: &mut Vec<Self::Row>,
                ) -> carbon_core::error::CarbonResult<()> {{
                    let _ = rows;
                    let Self(_metadata, _account) = self;
                    unreachable!("BatchInsert called for program with no account row variants");
                }}
            }}

            #[async_trait::async_trait]
            impl carbon_core::clickhouse::BatchCommit for {base_name}AccountRow {{
                async fn batch_commit(
                    &self,
                    client: &clickhouse::Client,
                    rows: &[Self],
                ) -> carbon_core::error::CarbonResult<()> {{
                    let _ = (client, rows);
                    unreachable!("BatchCommit called for program with no account row variants");
                }}
            }}
            """
        )

    row_modules = [f"{snake_case(variant.name)}_row" for variant in variants]
    row_imports = "\n".join(f"pub mod {module};" for module in row_modules)
    row_uses = "\n".join(f"pub use self::{module}::*;" for module in row_modules)

    row_enum = "\n".join(
        f"    {variant.name}({variant.name}Row)," for variant in variants
    )
    batch_insert_arms = []
    for variant in variants:
        row_name = f"{variant.name}Row"
        account_expr = "*account.clone()" if variant.is_boxed else "account.clone()"
        batch_insert_arms.append(
            f"            {base_name}Account::{variant.name}(account) => {{\n"
            f"                rows.push({base_name}AccountRow::{variant.name}(\n"
            f"                    {row_name}::try_from(({account_expr}, metadata.clone()))?,\n"
            f"                ));\n"
            f"            }}"
        )
    batch_insert_body = "\n".join(batch_insert_arms)

    batch_commit_arms = []
    for variant in variants:
        row_name = f"{variant.name}Row"
        batch_commit_arms.append(f"        commit_branch!({variant.name}, {row_name});")
    batch_commit_body = "\n".join(batch_commit_arms)

    migration_ops = ",\n            ".join(
        f"Box::new({variant.name}RowMigrationOperation)" for variant in variants
    )

    return dedent(
        f"""\
        //! This code was AUTOGENERATED using the Codama library.
        {row_imports}

        {row_uses}
        use super::{base_name}Account;

        pub struct {base_name}AccountsMigration;

        impl carbon_core::clickhouse::Migration for {base_name}AccountsMigration {{
            fn app(&self) -> &str {{
                "{app_name}"
            }}

            fn name(&self) -> &str {{
                "{snake_case(base_name)}_accounts"
            }}

            fn operations(&self) -> Vec<Box<dyn carbon_core::clickhouse::Operation>> {{
                vec![
                    {migration_ops}
                ]
            }}
        }}

        pub enum {base_name}AccountRow {{
{row_enum}
        }}

        pub struct {base_name}AccountMetadata(
            pub carbon_core::account::AccountMetadata,
            pub {base_name}Account,
        );

        #[async_trait::async_trait]
        impl carbon_core::clickhouse::BatchInsert for {base_name}AccountMetadata {{
            type Row = {base_name}AccountRow;

            async fn batch_insert(
                &self,
                rows: &mut Vec<Self::Row>,
            ) -> carbon_core::error::CarbonResult<()> {{
                let Self(metadata, account) = self;

                match account {{
{batch_insert_body}
                }}

                Ok(())
            }}
        }}

        #[async_trait::async_trait]
        impl carbon_core::clickhouse::BatchCommit for {base_name}AccountRow {{
            async fn batch_commit(
                &self,
                client: &clickhouse::Client,
                rows: &[Self],
            ) -> carbon_core::error::CarbonResult<()> {{
                macro_rules! commit_branch {{
                    ($variant:ident, $row:ty) => {{
                        if let Self::$variant(source) = self {{
                            let branch_rows: Vec<$row> = rows
                                .iter()
                                .filter_map(|row| match row {{
                                    Self::$variant(row) => Some(row.clone()),
                                    _ => None,
                                }})
                                .collect();
                            return <$row as carbon_core::clickhouse::Insert>::insert(
                                source,
                                client,
                                &branch_rows,
                            )
                            .await;
                        }}
                    }};
                }}

{batch_commit_body}
                Ok(())
            }}
        }}
        """
    )


def render_instruction_mod(
    app_name: str,
    enum_name_value: str,
    variants: list[InstructionVariant],
) -> str:
    base_name = enum_name_value.removesuffix("Instruction")

    if not variants:
        return dedent(
            f"""\
            //! This code was AUTOGENERATED using the Codama library.
            use super::{base_name}Instruction;

            pub struct {base_name}InstructionsMigration;

            impl carbon_core::clickhouse::Migration for {base_name}InstructionsMigration {{
                fn app(&self) -> &str {{
                    "{app_name}"
                }}

                fn name(&self) -> &str {{
                    "{snake_case(base_name)}_instructions"
                }}

                fn operations(&self) -> Vec<Box<dyn carbon_core::clickhouse::Operation>> {{
                    vec![]
                }}
            }}
            """
        )

    row_modules = [f"{snake_case(variant.name)}_row" for variant in variants]
    row_imports = "\n".join(f"pub mod {module};" for module in row_modules)
    row_uses = "\n".join(f"pub use self::{module}::*;" for module in row_modules)

    row_enum = "\n".join(
        f"    {variant.name}({variant.name}Row)," for variant in variants
    )
    batch_insert_arms = []
    for variant in variants:
        row_name = f"{variant.name}Row"
        batch_insert_arms.append(
            f"            {base_name}Instruction::{variant.name} {{ data, accounts, .. }} => {{\n"
            f"                rows.push({base_name}InstructionRow::{variant.name}(\n"
            f"                    {row_name}::try_from((data.clone(), metadata.clone(), accounts.clone()))?,\n"
            f"                ));\n"
            f"            }}"
        )
    batch_insert_body = "\n".join(batch_insert_arms)

    batch_commit_arms = []
    for variant in variants:
        row_name = f"{variant.name}Row"
        batch_commit_arms.append(f"        commit_branch!({variant.name}, {row_name});")
    batch_commit_body = "\n".join(batch_commit_arms)

    migration_ops = ",\n            ".join(
        f"Box::new({variant.name}RowMigrationOperation)" for variant in variants
    )

    return dedent(
        f"""\
        //! This code was AUTOGENERATED using the Codama library.
        {row_imports}

        {row_uses}
        use super::{base_name}Instruction;

        pub struct {base_name}InstructionsMigration;

        impl carbon_core::clickhouse::Migration for {base_name}InstructionsMigration {{
            fn app(&self) -> &str {{
                "{app_name}"
            }}

            fn name(&self) -> &str {{
                "{snake_case(base_name)}_instructions"
            }}

            fn operations(&self) -> Vec<Box<dyn carbon_core::clickhouse::Operation>> {{
                vec![
                    {migration_ops}
                ]
            }}
        }}

        pub enum {base_name}InstructionRow {{
{row_enum}
        }}

        pub struct {base_name}InstructionMetadata(
            pub carbon_core::instruction::InstructionMetadata,
            pub {base_name}Instruction,
        );

        #[async_trait::async_trait]
        impl carbon_core::clickhouse::BatchInsert for {base_name}InstructionMetadata {{
            type Row = {base_name}InstructionRow;

            async fn batch_insert(
                &self,
                rows: &mut Vec<Self::Row>,
            ) -> carbon_core::error::CarbonResult<()> {{
                let Self(metadata, instruction) = self;

                match instruction {{
{batch_insert_body}
                }}

                Ok(())
            }}
        }}

        #[async_trait::async_trait]
        impl carbon_core::clickhouse::BatchCommit for {base_name}InstructionRow {{
            async fn batch_commit(
                &self,
                client: &clickhouse::Client,
                rows: &[Self],
            ) -> carbon_core::error::CarbonResult<()> {{
                macro_rules! commit_branch {{
                    ($variant:ident, $row:ty) => {{
                        if let Self::$variant(source) = self {{
                            let branch_rows: Vec<$row> = rows
                                .iter()
                                .filter_map(|row| match row {{
                                    Self::$variant(row) => Some(row.clone()),
                                    _ => None,
                                }})
                                .collect();
                            return <$row as carbon_core::clickhouse::Insert>::insert(
                                source,
                                client,
                                &branch_rows,
                            )
                            .await;
                        }}
                    }};
                }}

{batch_commit_body}
                Ok(())
            }}
        }}
        """
    )


def normalize_generated_content(content: str) -> str:
    content = content.lstrip("\n")
    lines = [line.rstrip() for line in content.splitlines()]

    while lines and lines[0].strip() == "":
        lines.pop(0)

    if lines:
        lines[0] = lines[0].lstrip()

    return "\n".join(lines) + "\n"


def generate_decoder(decoder_dir: Path, overwrite: bool) -> list[Path]:
    generated: list[Path] = []
    prefix = decoder_prefix(decoder_dir)
    app_name = decoder_app_name(decoder_dir)

    accounts_mod = decoder_dir / "src" / "accounts" / "mod.rs"
    if accounts_mod.exists():
        source = accounts_mod.read_text()
        accounts_enum_name, account_variants = parse_account_variants(source)
        clickhouse_dir = decoder_dir / "src" / "accounts" / "clickhouse"
        clickhouse_dir.mkdir(parents=True, exist_ok=True)

        mod_path = clickhouse_dir / "mod.rs"
        if overwrite or not mod_path.exists():
            mod_path.write_text(
                normalize_generated_content(
                    render_account_mod(app_name, accounts_enum_name, account_variants)
                )
            )
            generated.append(mod_path)

        for variant in account_variants:
            _, row_content = render_account_row(prefix, variant)
            row_path = clickhouse_dir / f"{snake_case(variant.name)}_row.rs"
            if overwrite or not row_path.exists():
                row_path.write_text(normalize_generated_content(row_content))
                generated.append(row_path)

    instructions_mod = decoder_dir / "src" / "instructions" / "mod.rs"
    if instructions_mod.exists():
        source = instructions_mod.read_text()
        instructions_enum_name, instruction_variants = parse_instruction_variants(
            source
        )
        clickhouse_dir = decoder_dir / "src" / "instructions" / "clickhouse"
        clickhouse_dir.mkdir(parents=True, exist_ok=True)

        mod_path = clickhouse_dir / "mod.rs"
        if overwrite or not mod_path.exists():
            mod_path.write_text(
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
                row_path.write_text(normalize_generated_content(row_content))
                generated.append(row_path)

    return generated


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Generate ClickHouse adapters from pre-generated decoder source.",
    )
    parser.add_argument(
        "decoders",
        nargs="*",
        help="Decoder directories to process (defaults to every decoder under ./decoders).",
    )
    parser.add_argument(
        "--overwrite",
        action="store_true",
        help="Overwrite existing ClickHouse files instead of leaving them untouched.",
    )
    args = parser.parse_args()

    if args.decoders:
        decoder_dirs = [Path(path) for path in args.decoders]
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
        generated = generate_decoder(decoder_dir, overwrite=args.overwrite)
        if generated:
            total += len(generated)
            print(f"{decoder_dir}: generated {len(generated)} file(s)")

    print(f"Total generated files: {total}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
