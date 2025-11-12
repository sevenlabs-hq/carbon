use {
    crate::models::{RouteInstructionRecord, SwapEventRecord},
    carbon_core::{
        error::{CarbonResult, Error as CarbonError},
        postgres::metadata::InstructionRowMetadata,
    },
    sqlx::{self, types::BigDecimal, PgConnection, PgPool, Row},
    sqlx_migrator::{error::Error as MigratorError, Migration, Operation},
    std::{convert::TryFrom, str::FromStr},
};

const MIGRATION_APP: &str = "jupiter_swap_postgres";
const MIGRATION_NAME: &str = "structured_jupiter_swap";

pub struct JupiterSwapSchemaOperation;

impl JupiterSwapSchemaOperation {
    fn boxed() -> Box<dyn Operation<sqlx::Postgres>> {
        Box::new(Self)
    }
}

#[async_trait::async_trait]
impl Operation<sqlx::Postgres> for JupiterSwapSchemaOperation {
    async fn up(&self, connection: &mut PgConnection) -> Result<(), MigratorError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS jupiter_route_instructions (
                __signature TEXT NOT NULL,
                __instruction_index BIGINT NOT NULL,
                __stack_height BIGINT NOT NULL,
                __slot NUMERIC,
                variant TEXT NOT NULL,
                shared_accounts_id SMALLINT,
                in_amount NUMERIC,
                out_amount NUMERIC,
                quoted_in_amount NUMERIC,
                quoted_out_amount NUMERIC,
                slippage_bps INTEGER,
                platform_fee_bps INTEGER,
                positive_slippage_bps INTEGER,
                source_mint TEXT,
                destination_mint TEXT,
                route_plan JSONB NOT NULL,
                route_plan_version SMALLINT NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                PRIMARY KEY (__signature, __instruction_index, __stack_height)
            )
            "#,
        )
        .execute(&mut *connection)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS jupiter_route_instruction_accounts (
                __signature TEXT NOT NULL,
                __instruction_index BIGINT NOT NULL,
                __stack_height BIGINT NOT NULL,
                position INTEGER NOT NULL,
                pubkey TEXT NOT NULL,
                is_signer BOOLEAN NOT NULL,
                is_writable BOOLEAN NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                PRIMARY KEY (__signature, __instruction_index, __stack_height, position),
                FOREIGN KEY (__signature, __instruction_index, __stack_height)
                    REFERENCES jupiter_route_instructions (__signature, __instruction_index, __stack_height)
                    ON DELETE CASCADE
            )
            "#,
        )
        .execute(&mut *connection)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS jupiter_route_plan_steps (
                __signature TEXT NOT NULL,
                __instruction_index BIGINT NOT NULL,
                __stack_height BIGINT NOT NULL,
                step_index INTEGER NOT NULL,
                swap_variant TEXT NOT NULL,
                swap_json JSONB NOT NULL,
                weight_percent INTEGER,
                weight_bps INTEGER,
                input_index INTEGER NOT NULL,
                output_index INTEGER NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                PRIMARY KEY (__signature, __instruction_index, __stack_height, step_index),
                FOREIGN KEY (__signature, __instruction_index, __stack_height)
                    REFERENCES jupiter_route_instructions (__signature, __instruction_index, __stack_height)
                    ON DELETE CASCADE
            )
            "#,
        )
        .execute(&mut *connection)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS venue_labels (
                variant TEXT PRIMARY KEY,
                label TEXT NOT NULL,
                category TEXT,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
            "#,
        )
        .execute(&mut *connection)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS mint_reference_data (
                mint TEXT PRIMARY KEY,
                decimals INTEGER,
                symbol TEXT,
                last_updated_slot NUMERIC,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
            "#,
        )
        .execute(&mut *connection)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS jupiter_swap_hops (
                __signature TEXT NOT NULL,
                __instruction_index BIGINT NOT NULL,
                __stack_height BIGINT NOT NULL,
                hop_index INTEGER NOT NULL,
                __slot NUMERIC,
                route_instruction_index BIGINT,
                route_stack_height BIGINT,
                route_step_index INTEGER,
                event_type TEXT NOT NULL,
                input_mint TEXT NOT NULL,
                output_mint TEXT NOT NULL,
                input_amount NUMERIC NOT NULL,
                output_amount NUMERIC NOT NULL,
                input_amount_decimal DOUBLE PRECISION,
                output_amount_decimal DOUBLE PRECISION,
                price DOUBLE PRECISION,
                normalization_pending BOOLEAN NOT NULL DEFAULT FALSE,
                swap_variant TEXT,
                swap_json JSONB,
                venue_label TEXT,
                venue_category TEXT,
                amm_pubkey TEXT,
                metadata JSONB,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                PRIMARY KEY (__signature, __instruction_index, __stack_height, hop_index),
                FOREIGN KEY (__signature, route_instruction_index, route_stack_height)
                    REFERENCES jupiter_route_instructions (__signature, __instruction_index, __stack_height)
                    ON DELETE SET NULL
            )
            "#,
        )
        .execute(&mut *connection)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS jupiter_swap_event_envelopes (
                __signature TEXT NOT NULL,
                __instruction_index BIGINT NOT NULL,
                __stack_height BIGINT NOT NULL,
                event_index INTEGER NOT NULL,
                event_type TEXT NOT NULL,
                raw_event JSONB NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                PRIMARY KEY (__signature, __instruction_index, __stack_height, event_index, event_type)
            )
            "#,
        )
        .execute(&mut *connection)
        .await?;

        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_jupiter_swap_hops_pending
                ON jupiter_swap_hops (normalization_pending)
            "#,
        )
        .execute(&mut *connection)
        .await?;

        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_jupiter_swap_hops_route
                ON jupiter_swap_hops (__signature, route_instruction_index, route_stack_height)
            "#,
        )
        .execute(&mut *connection)
        .await?;

        Ok(())
    }

    async fn down(&self, connection: &mut PgConnection) -> Result<(), MigratorError> {
        sqlx::query("DROP TABLE IF EXISTS jupiter_swap_event_envelopes")
            .execute(&mut *connection)
            .await?;
        sqlx::query("DROP TABLE IF EXISTS jupiter_swap_hops")
            .execute(&mut *connection)
            .await?;
        sqlx::query("DROP TABLE IF EXISTS jupiter_route_plan_steps")
            .execute(&mut *connection)
            .await?;
        sqlx::query("DROP TABLE IF EXISTS jupiter_route_instruction_accounts")
            .execute(&mut *connection)
            .await?;
        sqlx::query("DROP TABLE IF EXISTS jupiter_route_instructions")
            .execute(&mut *connection)
            .await?;
        sqlx::query("DROP TABLE IF EXISTS mint_reference_data")
            .execute(&mut *connection)
            .await?;
        sqlx::query("DROP TABLE IF EXISTS venue_labels")
            .execute(&mut *connection)
            .await?;

        Ok(())
    }
}

pub struct JupiterSwapMigration;

impl JupiterSwapMigration {
    pub fn boxed() -> Box<dyn Migration<sqlx::Postgres>> {
        Box::new(Self)
    }
}

#[async_trait::async_trait]
impl Migration<sqlx::Postgres> for JupiterSwapMigration {
    fn app(&self) -> &str {
        MIGRATION_APP
    }

    fn name(&self) -> &str {
        MIGRATION_NAME
    }

    fn operations(&self) -> Vec<Box<dyn Operation<sqlx::Postgres>>> {
        vec![JupiterSwapSchemaOperation::boxed()]
    }

    fn parents(&self) -> Vec<Box<dyn Migration<sqlx::Postgres>>> {
        vec![]
    }
}

pub struct JupiterSwapRepository {
    pool: PgPool,
}

impl JupiterSwapRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn upsert_route_instruction(
        &self,
        record: RouteInstructionRecord,
    ) -> CarbonResult<()> {
        let mut tx = self.pool.begin().await.map_err(to_carbon_error)?;
        let (signature, instruction_index, stack_height, slot) = metadata_parts(&record.metadata);

        sqlx::query(
            r#"
            INSERT INTO jupiter_route_instructions (
                __signature,
                __instruction_index,
                __stack_height,
                __slot,
                variant,
                shared_accounts_id,
                in_amount,
                out_amount,
                quoted_in_amount,
                quoted_out_amount,
                slippage_bps,
                platform_fee_bps,
                positive_slippage_bps,
                source_mint,
                destination_mint,
                route_plan,
                route_plan_version,
                updated_at
            )
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,NOW())
            ON CONFLICT (__signature, __instruction_index, __stack_height)
            DO UPDATE SET
                __slot = EXCLUDED.__slot,
                variant = EXCLUDED.variant,
                shared_accounts_id = EXCLUDED.shared_accounts_id,
                in_amount = EXCLUDED.in_amount,
                out_amount = EXCLUDED.out_amount,
                quoted_in_amount = EXCLUDED.quoted_in_amount,
                quoted_out_amount = EXCLUDED.quoted_out_amount,
                slippage_bps = EXCLUDED.slippage_bps,
                platform_fee_bps = EXCLUDED.platform_fee_bps,
                positive_slippage_bps = EXCLUDED.positive_slippage_bps,
                source_mint = EXCLUDED.source_mint,
                destination_mint = EXCLUDED.destination_mint,
                route_plan = EXCLUDED.route_plan,
                route_plan_version = EXCLUDED.route_plan_version,
                updated_at = NOW()
            "#,
        )
        .bind(&signature)
        .bind(instruction_index)
        .bind(stack_height)
        .bind(slot)
        .bind(&record.variant)
        .bind(record.shared_accounts_id.map(|id| id as i16))
        .bind(record.in_amount.map(decimal_from_u64))
        .bind(record.out_amount.map(decimal_from_u64))
        .bind(record.quoted_in_amount.map(decimal_from_u64))
        .bind(record.quoted_out_amount.map(decimal_from_u64))
        .bind(record.slippage_bps.map(|v| v as i32))
        .bind(record.platform_fee_bps.map(|v| v as i32))
        .bind(record.positive_slippage_bps.map(|v| v as i32))
        .bind(&record.source_mint)
        .bind(&record.destination_mint)
        .bind(&record.route_plan)
        .bind(record.route_plan_version)
        .execute(&mut *tx)
        .await
        .map_err(to_carbon_error)?;

        sqlx::query(
            "DELETE FROM jupiter_route_instruction_accounts WHERE __signature = $1 AND __instruction_index = $2 AND __stack_height = $3",
        )
        .bind(&signature)
        .bind(instruction_index)
        .bind(stack_height)
        .execute(&mut *tx)
        .await
        .map_err(to_carbon_error)?;

        for account in &record.accounts {
            sqlx::query(
                r#"
                INSERT INTO jupiter_route_instruction_accounts (
                    __signature,
                    __instruction_index,
                    __stack_height,
                    position,
                    pubkey,
                    is_signer,
                    is_writable
                ) VALUES ($1,$2,$3,$4,$5,$6,$7)
                ON CONFLICT (__signature, __instruction_index, __stack_height, position)
                DO UPDATE SET
                    pubkey = EXCLUDED.pubkey,
                    is_signer = EXCLUDED.is_signer,
                    is_writable = EXCLUDED.is_writable,
                    created_at = jupiter_route_instruction_accounts.created_at
                "#,
            )
            .bind(&signature)
            .bind(instruction_index)
            .bind(stack_height)
            .bind(account.position)
            .bind(&account.pubkey)
            .bind(account.is_signer)
            .bind(account.is_writable)
            .execute(&mut *tx)
            .await
            .map_err(to_carbon_error)?;
        }

        sqlx::query(
            "DELETE FROM jupiter_route_plan_steps WHERE __signature = $1 AND __instruction_index = $2 AND __stack_height = $3",
        )
        .bind(&signature)
        .bind(instruction_index)
        .bind(stack_height)
        .execute(&mut *tx)
        .await
        .map_err(to_carbon_error)?;

        for step in &record.steps {
            sqlx::query(
                r#"
                INSERT INTO jupiter_route_plan_steps (
                    __signature,
                    __instruction_index,
                    __stack_height,
                    step_index,
                    swap_variant,
                    swap_json,
                    weight_percent,
                    weight_bps,
                    input_index,
                    output_index
                ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
                ON CONFLICT (__signature, __instruction_index, __stack_height, step_index)
                DO UPDATE SET
                    swap_variant = EXCLUDED.swap_variant,
                    swap_json = EXCLUDED.swap_json,
                    weight_percent = EXCLUDED.weight_percent,
                    weight_bps = EXCLUDED.weight_bps,
                    input_index = EXCLUDED.input_index,
                    output_index = EXCLUDED.output_index
                "#,
            )
            .bind(&signature)
            .bind(instruction_index)
            .bind(stack_height)
            .bind(step.step_index)
            .bind(&step.swap_variant)
            .bind(&step.swap_json)
            .bind(step.weight_percent)
            .bind(step.weight_bps)
            .bind(step.input_index)
            .bind(step.output_index)
            .execute(&mut *tx)
            .await
            .map_err(to_carbon_error)?;
        }

        tx.commit().await.map_err(to_carbon_error)
    }

    pub async fn persist_swap_event(&self, record: SwapEventRecord) -> CarbonResult<()> {
        let mut tx = self.pool.begin().await.map_err(to_carbon_error)?;
        let (signature, instruction_index, stack_height, slot) = metadata_parts(&record.metadata);

        let route_link = self
            .find_route_link(&mut tx, &signature, instruction_index, stack_height)
            .await?;

        let (swap_variant, swap_json, route_step_index) = if let Some(link) = &route_link {
            self.fetch_route_step(
                &mut tx,
                &signature,
                link.instruction_index,
                link.stack_height,
                record.event_index,
            )
            .await?
        } else {
            (None, None, None)
        };

        let venue = if let Some(ref variant) = swap_variant {
            Some(self.ensure_venue_label(&mut tx, variant).await?)
        } else {
            None
        };
        let (venue_label, venue_category) = match venue {
            Some(label) => (Some(label.label), label.category),
            None => (None, None),
        };

        let input_decimals = self
            .fetch_mint_decimals(&mut tx, &record.input_mint)
            .await?;
        let output_decimals = self
            .fetch_mint_decimals(&mut tx, &record.output_mint)
            .await?;

        let normalization_pending = input_decimals.is_none() || output_decimals.is_none();
        let (input_amount_decimal, output_amount_decimal, price) = if normalization_pending {
            (None, None, None)
        } else {
            let input_decimals = input_decimals.unwrap();
            let output_decimals = output_decimals.unwrap();
            let input_decimal = normalize_amount(record.input_amount, input_decimals);
            let output_decimal = normalize_amount(record.output_amount, output_decimals);
            let price = if input_decimal > 0.0 {
                Some(output_decimal / input_decimal)
            } else {
                None
            };
            (Some(input_decimal), Some(output_decimal), price)
        };

        let mut metadata = serde_json::json!({
            "event_index": record.event_index,
            "event_type": record.event_type.as_str(),
        });
        if let Some(batch_size) = record.batch_size {
            metadata["batch_size"] = serde_json::json!(batch_size);
        }
        if let Some(link) = &route_link {
            metadata["route_instruction_index"] = serde_json::json!(link.instruction_index);
            metadata["route_stack_height"] = serde_json::json!(link.stack_height);
        }
        metadata["normalization_pending"] = serde_json::json!(normalization_pending);

        sqlx::query(
            r#"
            INSERT INTO jupiter_swap_hops (
                __signature,
                __instruction_index,
                __stack_height,
                hop_index,
                __slot,
                route_instruction_index,
                route_stack_height,
                route_step_index,
                event_type,
                input_mint,
                output_mint,
                input_amount,
                output_amount,
                input_amount_decimal,
                output_amount_decimal,
                price,
                normalization_pending,
                swap_variant,
                swap_json,
                venue_label,
                venue_category,
                amm_pubkey,
                metadata,
                updated_at
            ) VALUES (
                $1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20,$21,$22,$23,NOW()
            )
            ON CONFLICT (__signature, __instruction_index, __stack_height, hop_index)
            DO UPDATE SET
                __slot = EXCLUDED.__slot,
                route_instruction_index = EXCLUDED.route_instruction_index,
                route_stack_height = EXCLUDED.route_stack_height,
                route_step_index = EXCLUDED.route_step_index,
                event_type = EXCLUDED.event_type,
                input_mint = EXCLUDED.input_mint,
                output_mint = EXCLUDED.output_mint,
                input_amount = EXCLUDED.input_amount,
                output_amount = EXCLUDED.output_amount,
                input_amount_decimal = EXCLUDED.input_amount_decimal,
                output_amount_decimal = EXCLUDED.output_amount_decimal,
                price = EXCLUDED.price,
                normalization_pending = EXCLUDED.normalization_pending,
                swap_variant = EXCLUDED.swap_variant,
                swap_json = EXCLUDED.swap_json,
                venue_label = EXCLUDED.venue_label,
                venue_category = EXCLUDED.venue_category,
                amm_pubkey = EXCLUDED.amm_pubkey,
                metadata = EXCLUDED.metadata,
                updated_at = NOW()
            "#,
        )
        .bind(&signature)
        .bind(instruction_index)
        .bind(stack_height)
        .bind(record.event_index)
        .bind(slot)
        .bind(route_link.as_ref().map(|link| link.instruction_index))
        .bind(route_link.as_ref().map(|link| link.stack_height))
        .bind(route_step_index)
        .bind(record.event_type.as_str())
        .bind(&record.input_mint)
        .bind(&record.output_mint)
        .bind(decimal_from_u64(record.input_amount))
        .bind(decimal_from_u64(record.output_amount))
        .bind(input_amount_decimal)
        .bind(output_amount_decimal)
        .bind(price)
        .bind(normalization_pending)
        .bind(swap_variant.as_deref())
        .bind(swap_json.as_ref())
        .bind(venue_label.as_deref())
        .bind(venue_category.as_deref())
        .bind(record.amm.as_deref())
        .bind(metadata)
        .execute(&mut *tx)
        .await
        .map_err(to_carbon_error)?;

        sqlx::query(
            r#"
            INSERT INTO jupiter_swap_event_envelopes (
                __signature,
                __instruction_index,
                __stack_height,
                event_index,
                event_type,
                raw_event,
                updated_at
            ) VALUES ($1,$2,$3,$4,$5,$6,NOW())
            ON CONFLICT (__signature, __instruction_index, __stack_height, event_index, event_type)
            DO UPDATE SET raw_event = EXCLUDED.raw_event, updated_at = NOW()
            "#,
        )
        .bind(&signature)
        .bind(instruction_index)
        .bind(stack_height)
        .bind(record.event_index)
        .bind(record.event_type.as_str())
        .bind(&record.raw_event)
        .execute(&mut *tx)
        .await
        .map_err(to_carbon_error)?;

        tx.commit().await.map_err(to_carbon_error)
    }

    async fn find_route_link(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        signature: &str,
        instruction_index: i64,
        stack_height: i64,
    ) -> CarbonResult<Option<RouteLink>> {
        let rows = sqlx::query(
            r#"
            SELECT __instruction_index, __stack_height
            FROM jupiter_route_instructions
            WHERE __signature = $1
            ORDER BY __instruction_index DESC, __stack_height DESC
            "#,
        )
        .bind(signature)
        .fetch_all(&mut **tx)
        .await
        .map_err(to_carbon_error)?;

        let mut exact = None;
        let mut fallback = None;

        for row in rows {
            let idx: i64 = row.get("__instruction_index");
            let stack: i64 = row.get("__stack_height");
            if idx == instruction_index && stack == stack_height {
                exact = Some(RouteLink {
                    instruction_index: idx,
                    stack_height: stack,
                });
                break;
            }
            if idx <= instruction_index && stack <= stack_height && fallback.is_none() {
                fallback = Some(RouteLink {
                    instruction_index: idx,
                    stack_height: stack,
                });
            }
        }

        Ok(exact.or(fallback))
    }

    async fn fetch_route_step(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        signature: &str,
        instruction_index: i64,
        stack_height: i64,
        event_index: i32,
    ) -> CarbonResult<(Option<String>, Option<serde_json::Value>, Option<i32>)> {
        let steps = sqlx::query(
            r#"
            SELECT step_index, swap_variant, swap_json
            FROM jupiter_route_plan_steps
            WHERE __signature = $1 AND __instruction_index = $2 AND __stack_height = $3
            ORDER BY step_index
            "#,
        )
        .bind(signature)
        .bind(instruction_index)
        .bind(stack_height)
        .fetch_all(&mut **tx)
        .await
        .map_err(to_carbon_error)?;

        if steps.is_empty() {
            return Ok((None, None, None));
        }

        let target_index = event_index as usize;
        let row = steps
            .get(target_index)
            .or_else(|| steps.last())
            .ok_or_else(|| CarbonError::Custom("Failed to resolve route plan step".to_string()))?;

        let step_index: i32 = row.get("step_index");
        let swap_variant: String = row.get("swap_variant");
        let swap_json: serde_json::Value = row.get("swap_json");

        Ok((Some(swap_variant), Some(swap_json), Some(step_index)))
    }

    async fn ensure_venue_label(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        variant: &str,
    ) -> CarbonResult<VenueLabel> {
        if let Some(existing) =
            sqlx::query("SELECT label, category FROM venue_labels WHERE variant = $1")
                .bind(variant)
                .fetch_optional(&mut **tx)
                .await
                .map_err(to_carbon_error)?
        {
            let label: String = existing.get("label");
            let category: Option<String> = existing.try_get("category").ok();
            return Ok(VenueLabel { label, category });
        }

        sqlx::query(
            "INSERT INTO venue_labels (variant, label) VALUES ($1, $2) ON CONFLICT (variant) DO NOTHING",
        )
        .bind(variant)
        .bind(variant)
        .execute(&mut **tx)
        .await
        .map_err(to_carbon_error)?;

        Ok(VenueLabel {
            label: variant.to_string(),
            category: None,
        })
    }

    async fn fetch_mint_decimals(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        mint: &str,
    ) -> CarbonResult<Option<i32>> {
        let result = sqlx::query("SELECT decimals FROM mint_reference_data WHERE mint = $1")
            .bind(mint)
            .fetch_optional(&mut **tx)
            .await
            .map_err(to_carbon_error)?;

        Ok(result.map(|row| row.get::<i32, _>("decimals")))
    }
}

fn metadata_parts(metadata: &InstructionRowMetadata) -> (String, i64, i64, Option<BigDecimal>) {
    let slot = metadata
        .slot
        .clone()
        .and_then(|slot| u64::try_from(slot).ok())
        .map(decimal_from_u64);
    (
        metadata.signature.clone(),
        *metadata.instruction_index,
        *metadata.stack_height,
        slot,
    )
}

fn decimal_from_u64(value: u64) -> BigDecimal {
    BigDecimal::from_str(&value.to_string()).expect("valid decimal from u64")
}

fn normalize_amount(amount: u64, decimals: i32) -> f64 {
    let base: f64 = 10f64.powi(decimals);
    amount as f64 / base
}

fn to_carbon_error(err: impl std::error::Error) -> CarbonError {
    CarbonError::Custom(err.to_string())
}

struct RouteLink {
    instruction_index: i64,
    stack_height: i64,
}

struct VenueLabel {
    label: String,
    category: Option<String>,
}
