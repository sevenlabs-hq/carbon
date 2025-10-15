use juniper::{graphql_object, FieldResult};
use std::str::FromStr;

pub struct QueryRoot;

#[graphql_object(context = crate::graphql::context::GraphQLContext)]
impl QueryRoot {
    // Accounts
            async fn bin_array(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::BinArrayGraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(solana_pubkey::Pubkey::from_str(&pubkey).map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?);
        let row = crate::accounts::postgres::BinArrayRow::lookup(pk, &context.pool).await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.map(|row| row.try_into().ok()).flatten())
    }

    async fn list_bin_array(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::BinArrayGraphQL>> {
        let rows: Vec<crate::accounts::postgres::BinArrayRow> = sqlx::query_as(
            r#"SELECT * FROM bin_array_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn bin_array_bitmap_extension(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::BinArrayBitmapExtensionGraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(solana_pubkey::Pubkey::from_str(&pubkey).map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?);
        let row = crate::accounts::postgres::BinArrayBitmapExtensionRow::lookup(pk, &context.pool).await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.map(|row| row.try_into().ok()).flatten())
    }

    async fn list_bin_array_bitmap_extension(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::BinArrayBitmapExtensionGraphQL>> {
        let rows: Vec<crate::accounts::postgres::BinArrayBitmapExtensionRow> = sqlx::query_as(
            r#"SELECT * FROM bin_array_bitmap_extension_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn claim_fee_operator(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::ClaimFeeOperatorGraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(solana_pubkey::Pubkey::from_str(&pubkey).map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?);
        let row = crate::accounts::postgres::ClaimFeeOperatorRow::lookup(pk, &context.pool).await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.map(|row| row.try_into().ok()).flatten())
    }

    async fn list_claim_fee_operator(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::ClaimFeeOperatorGraphQL>> {
        let rows: Vec<crate::accounts::postgres::ClaimFeeOperatorRow> = sqlx::query_as(
            r#"SELECT * FROM claim_fee_operator_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn dummy_zc_account(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::DummyZcAccountGraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(solana_pubkey::Pubkey::from_str(&pubkey).map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?);
        let row = crate::accounts::postgres::DummyZcAccountRow::lookup(pk, &context.pool).await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.map(|row| row.try_into().ok()).flatten())
    }

    async fn list_dummy_zc_account(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::DummyZcAccountGraphQL>> {
        let rows: Vec<crate::accounts::postgres::DummyZcAccountRow> = sqlx::query_as(
            r#"SELECT * FROM dummy_zc_account_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn lb_pair(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::LbPairGraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(solana_pubkey::Pubkey::from_str(&pubkey).map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?);
        let row = crate::accounts::postgres::LbPairRow::lookup(pk, &context.pool).await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.map(|row| row.try_into().ok()).flatten())
    }

    async fn list_lb_pair(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::LbPairGraphQL>> {
        let rows: Vec<crate::accounts::postgres::LbPairRow> = sqlx::query_as(
            r#"SELECT * FROM lb_pair_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn oracle(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::OracleGraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(solana_pubkey::Pubkey::from_str(&pubkey).map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?);
        let row = crate::accounts::postgres::OracleRow::lookup(pk, &context.pool).await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.map(|row| row.try_into().ok()).flatten())
    }

    async fn list_oracle(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::OracleGraphQL>> {
        let rows: Vec<crate::accounts::postgres::OracleRow> = sqlx::query_as(
            r#"SELECT * FROM oracle_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn position(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::PositionGraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(solana_pubkey::Pubkey::from_str(&pubkey).map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?);
        let row = crate::accounts::postgres::PositionRow::lookup(pk, &context.pool).await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.map(|row| row.try_into().ok()).flatten())
    }

    async fn list_position(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::PositionGraphQL>> {
        let rows: Vec<crate::accounts::postgres::PositionRow> = sqlx::query_as(
            r#"SELECT * FROM position_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn position_v2(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::PositionV2GraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(solana_pubkey::Pubkey::from_str(&pubkey).map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?);
        let row = crate::accounts::postgres::PositionV2Row::lookup(pk, &context.pool).await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.map(|row| row.try_into().ok()).flatten())
    }

    async fn list_position_v2(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::PositionV2GraphQL>> {
        let rows: Vec<crate::accounts::postgres::PositionV2Row> = sqlx::query_as(
            r#"SELECT * FROM position_v2_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn preset_parameter(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::PresetParameterGraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(solana_pubkey::Pubkey::from_str(&pubkey).map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?);
        let row = crate::accounts::postgres::PresetParameterRow::lookup(pk, &context.pool).await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.map(|row| row.try_into().ok()).flatten())
    }

    async fn list_preset_parameter(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::PresetParameterGraphQL>> {
        let rows: Vec<crate::accounts::postgres::PresetParameterRow> = sqlx::query_as(
            r#"SELECT * FROM preset_parameter_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn preset_parameter2(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::PresetParameter2GraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(solana_pubkey::Pubkey::from_str(&pubkey).map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?);
        let row = crate::accounts::postgres::PresetParameter2Row::lookup(pk, &context.pool).await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.map(|row| row.try_into().ok()).flatten())
    }

    async fn list_preset_parameter2(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::PresetParameter2GraphQL>> {
        let rows: Vec<crate::accounts::postgres::PresetParameter2Row> = sqlx::query_as(
            r#"SELECT * FROM preset_parameter2_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn token_badge(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::TokenBadgeGraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(solana_pubkey::Pubkey::from_str(&pubkey).map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?);
        let row = crate::accounts::postgres::TokenBadgeRow::lookup(pk, &context.pool).await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.map(|row| row.try_into().ok()).flatten())
    }

    async fn list_token_badge(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::TokenBadgeGraphQL>> {
        let rows: Vec<crate::accounts::postgres::TokenBadgeRow> = sqlx::query_as(
            r#"SELECT * FROM token_badge_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
        
    // Instructions (per-instruction list and lookup by signature+index)
            async fn add_liquidity(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddLiquidityGraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddLiquidityRow> = sqlx::query_as(
            r#"SELECT * FROM add_liquidity_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_add_liquidity(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddLiquidityGraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddLiquidityRow> = sqlx::query_as(
            r#"SELECT * FROM add_liquidity_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn add_liquidity2(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddLiquidity2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddLiquidity2Row> = sqlx::query_as(
            r#"SELECT * FROM add_liquidity2_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_add_liquidity2(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddLiquidity2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddLiquidity2Row> = sqlx::query_as(
            r#"SELECT * FROM add_liquidity2_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn add_liquidity_by_strategy(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddLiquidityByStrategyGraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddLiquidityByStrategyRow> = sqlx::query_as(
            r#"SELECT * FROM add_liquidity_by_strategy_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_add_liquidity_by_strategy(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddLiquidityByStrategyGraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddLiquidityByStrategyRow> = sqlx::query_as(
            r#"SELECT * FROM add_liquidity_by_strategy_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn add_liquidity_by_strategy2(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddLiquidityByStrategy2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddLiquidityByStrategy2Row> = sqlx::query_as(
            r#"SELECT * FROM add_liquidity_by_strategy2_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_add_liquidity_by_strategy2(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddLiquidityByStrategy2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddLiquidityByStrategy2Row> = sqlx::query_as(
            r#"SELECT * FROM add_liquidity_by_strategy2_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn add_liquidity_by_strategy_one_side(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddLiquidityByStrategyOneSideGraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddLiquidityByStrategyOneSideRow> = sqlx::query_as(
            r#"SELECT * FROM add_liquidity_by_strategy_one_side_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_add_liquidity_by_strategy_one_side(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddLiquidityByStrategyOneSideGraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddLiquidityByStrategyOneSideRow> = sqlx::query_as(
            r#"SELECT * FROM add_liquidity_by_strategy_one_side_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn add_liquidity_by_weight(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddLiquidityByWeightGraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddLiquidityByWeightRow> = sqlx::query_as(
            r#"SELECT * FROM add_liquidity_by_weight_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_add_liquidity_by_weight(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddLiquidityByWeightGraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddLiquidityByWeightRow> = sqlx::query_as(
            r#"SELECT * FROM add_liquidity_by_weight_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn add_liquidity_one_side(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddLiquidityOneSideGraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddLiquidityOneSideRow> = sqlx::query_as(
            r#"SELECT * FROM add_liquidity_one_side_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_add_liquidity_one_side(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddLiquidityOneSideGraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddLiquidityOneSideRow> = sqlx::query_as(
            r#"SELECT * FROM add_liquidity_one_side_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn add_liquidity_one_side_precise(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddLiquidityOneSidePreciseGraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddLiquidityOneSidePreciseRow> = sqlx::query_as(
            r#"SELECT * FROM add_liquidity_one_side_precise_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_add_liquidity_one_side_precise(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddLiquidityOneSidePreciseGraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddLiquidityOneSidePreciseRow> = sqlx::query_as(
            r#"SELECT * FROM add_liquidity_one_side_precise_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn add_liquidity_one_side_precise2(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddLiquidityOneSidePrecise2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddLiquidityOneSidePrecise2Row> = sqlx::query_as(
            r#"SELECT * FROM add_liquidity_one_side_precise2_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_add_liquidity_one_side_precise2(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddLiquidityOneSidePrecise2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddLiquidityOneSidePrecise2Row> = sqlx::query_as(
            r#"SELECT * FROM add_liquidity_one_side_precise2_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                        async fn claim_fee2(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::ClaimFee2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::ClaimFee2Row> = sqlx::query_as(
            r#"SELECT * FROM claim_fee2_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_claim_fee2(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::ClaimFee2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::ClaimFee2Row> = sqlx::query_as(
            r#"SELECT * FROM claim_fee2_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn claim_reward(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::ClaimRewardGraphQL>> {
        let rows: Vec<crate::instructions::postgres::ClaimRewardRow> = sqlx::query_as(
            r#"SELECT * FROM claim_reward_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_claim_reward(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::ClaimRewardGraphQL>> {
        let rows: Vec<crate::instructions::postgres::ClaimRewardRow> = sqlx::query_as(
            r#"SELECT * FROM claim_reward_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn claim_reward2(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::ClaimReward2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::ClaimReward2Row> = sqlx::query_as(
            r#"SELECT * FROM claim_reward2_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_claim_reward2(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::ClaimReward2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::ClaimReward2Row> = sqlx::query_as(
            r#"SELECT * FROM claim_reward2_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                                                                        async fn decrease_position_length(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::DecreasePositionLengthGraphQL>> {
        let rows: Vec<crate::instructions::postgres::DecreasePositionLengthRow> = sqlx::query_as(
            r#"SELECT * FROM decrease_position_length_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_decrease_position_length(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::DecreasePositionLengthGraphQL>> {
        let rows: Vec<crate::instructions::postgres::DecreasePositionLengthRow> = sqlx::query_as(
            r#"SELECT * FROM decrease_position_length_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn for_idl_type_generation_do_not_call(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::ForIdlTypeGenerationDoNotCallGraphQL>> {
        let rows: Vec<crate::instructions::postgres::ForIdlTypeGenerationDoNotCallRow> = sqlx::query_as(
            r#"SELECT * FROM for_idl_type_generation_do_not_call_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_for_idl_type_generation_do_not_call(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::ForIdlTypeGenerationDoNotCallGraphQL>> {
        let rows: Vec<crate::instructions::postgres::ForIdlTypeGenerationDoNotCallRow> = sqlx::query_as(
            r#"SELECT * FROM for_idl_type_generation_do_not_call_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn fund_reward(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::FundRewardGraphQL>> {
        let rows: Vec<crate::instructions::postgres::FundRewardRow> = sqlx::query_as(
            r#"SELECT * FROM fund_reward_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_fund_reward(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::FundRewardGraphQL>> {
        let rows: Vec<crate::instructions::postgres::FundRewardRow> = sqlx::query_as(
            r#"SELECT * FROM fund_reward_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn go_to_a_bin(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::GoToABinGraphQL>> {
        let rows: Vec<crate::instructions::postgres::GoToABinRow> = sqlx::query_as(
            r#"SELECT * FROM go_to_a_bin_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_go_to_a_bin(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::GoToABinGraphQL>> {
        let rows: Vec<crate::instructions::postgres::GoToABinRow> = sqlx::query_as(
            r#"SELECT * FROM go_to_a_bin_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn increase_oracle_length(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::IncreaseOracleLengthGraphQL>> {
        let rows: Vec<crate::instructions::postgres::IncreaseOracleLengthRow> = sqlx::query_as(
            r#"SELECT * FROM increase_oracle_length_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_increase_oracle_length(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::IncreaseOracleLengthGraphQL>> {
        let rows: Vec<crate::instructions::postgres::IncreaseOracleLengthRow> = sqlx::query_as(
            r#"SELECT * FROM increase_oracle_length_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn increase_position_length(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::IncreasePositionLengthGraphQL>> {
        let rows: Vec<crate::instructions::postgres::IncreasePositionLengthRow> = sqlx::query_as(
            r#"SELECT * FROM increase_position_length_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_increase_position_length(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::IncreasePositionLengthGraphQL>> {
        let rows: Vec<crate::instructions::postgres::IncreasePositionLengthRow> = sqlx::query_as(
            r#"SELECT * FROM increase_position_length_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn initialize_bin_array(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializeBinArrayGraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializeBinArrayRow> = sqlx::query_as(
            r#"SELECT * FROM initialize_bin_array_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_initialize_bin_array(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializeBinArrayGraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializeBinArrayRow> = sqlx::query_as(
            r#"SELECT * FROM initialize_bin_array_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                        async fn initialize_customizable_permissionless_lb_pair(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializeCustomizablePermissionlessLbPairGraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializeCustomizablePermissionlessLbPairRow> = sqlx::query_as(
            r#"SELECT * FROM initialize_customizable_permissionless_lb_pair_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_initialize_customizable_permissionless_lb_pair(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializeCustomizablePermissionlessLbPairGraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializeCustomizablePermissionlessLbPairRow> = sqlx::query_as(
            r#"SELECT * FROM initialize_customizable_permissionless_lb_pair_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn initialize_customizable_permissionless_lb_pair2(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializeCustomizablePermissionlessLbPair2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializeCustomizablePermissionlessLbPair2Row> = sqlx::query_as(
            r#"SELECT * FROM initialize_customizable_permissionless_lb_pair2_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_initialize_customizable_permissionless_lb_pair2(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializeCustomizablePermissionlessLbPair2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializeCustomizablePermissionlessLbPair2Row> = sqlx::query_as(
            r#"SELECT * FROM initialize_customizable_permissionless_lb_pair2_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn initialize_lb_pair(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializeLbPairGraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializeLbPairRow> = sqlx::query_as(
            r#"SELECT * FROM initialize_lb_pair_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_initialize_lb_pair(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializeLbPairGraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializeLbPairRow> = sqlx::query_as(
            r#"SELECT * FROM initialize_lb_pair_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn initialize_lb_pair2(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializeLbPair2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializeLbPair2Row> = sqlx::query_as(
            r#"SELECT * FROM initialize_lb_pair2_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_initialize_lb_pair2(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializeLbPair2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializeLbPair2Row> = sqlx::query_as(
            r#"SELECT * FROM initialize_lb_pair2_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn initialize_permission_lb_pair(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializePermissionLbPairGraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializePermissionLbPairRow> = sqlx::query_as(
            r#"SELECT * FROM initialize_permission_lb_pair_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_initialize_permission_lb_pair(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializePermissionLbPairGraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializePermissionLbPairRow> = sqlx::query_as(
            r#"SELECT * FROM initialize_permission_lb_pair_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn initialize_position(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializePositionGraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializePositionRow> = sqlx::query_as(
            r#"SELECT * FROM initialize_position_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_initialize_position(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializePositionGraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializePositionRow> = sqlx::query_as(
            r#"SELECT * FROM initialize_position_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn initialize_position_by_operator(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializePositionByOperatorGraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializePositionByOperatorRow> = sqlx::query_as(
            r#"SELECT * FROM initialize_position_by_operator_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_initialize_position_by_operator(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializePositionByOperatorGraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializePositionByOperatorRow> = sqlx::query_as(
            r#"SELECT * FROM initialize_position_by_operator_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn initialize_position_pda(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializePositionPdaGraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializePositionPdaRow> = sqlx::query_as(
            r#"SELECT * FROM initialize_position_pda_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_initialize_position_pda(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializePositionPdaGraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializePositionPdaRow> = sqlx::query_as(
            r#"SELECT * FROM initialize_position_pda_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn initialize_preset_parameter(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializePresetParameterGraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializePresetParameterRow> = sqlx::query_as(
            r#"SELECT * FROM initialize_preset_parameter_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_initialize_preset_parameter(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializePresetParameterGraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializePresetParameterRow> = sqlx::query_as(
            r#"SELECT * FROM initialize_preset_parameter_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn initialize_preset_parameter2(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializePresetParameter2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializePresetParameter2Row> = sqlx::query_as(
            r#"SELECT * FROM initialize_preset_parameter2_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_initialize_preset_parameter2(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializePresetParameter2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializePresetParameter2Row> = sqlx::query_as(
            r#"SELECT * FROM initialize_preset_parameter2_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn initialize_reward(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializeRewardGraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializeRewardRow> = sqlx::query_as(
            r#"SELECT * FROM initialize_reward_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_initialize_reward(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::InitializeRewardGraphQL>> {
        let rows: Vec<crate::instructions::postgres::InitializeRewardRow> = sqlx::query_as(
            r#"SELECT * FROM initialize_reward_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                                        async fn rebalance_liquidity(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::RebalanceLiquidityGraphQL>> {
        let rows: Vec<crate::instructions::postgres::RebalanceLiquidityRow> = sqlx::query_as(
            r#"SELECT * FROM rebalance_liquidity_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_rebalance_liquidity(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::RebalanceLiquidityGraphQL>> {
        let rows: Vec<crate::instructions::postgres::RebalanceLiquidityRow> = sqlx::query_as(
            r#"SELECT * FROM rebalance_liquidity_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                        async fn remove_liquidity(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::RemoveLiquidityGraphQL>> {
        let rows: Vec<crate::instructions::postgres::RemoveLiquidityRow> = sqlx::query_as(
            r#"SELECT * FROM remove_liquidity_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_remove_liquidity(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::RemoveLiquidityGraphQL>> {
        let rows: Vec<crate::instructions::postgres::RemoveLiquidityRow> = sqlx::query_as(
            r#"SELECT * FROM remove_liquidity_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn remove_liquidity2(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::RemoveLiquidity2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::RemoveLiquidity2Row> = sqlx::query_as(
            r#"SELECT * FROM remove_liquidity2_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_remove_liquidity2(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::RemoveLiquidity2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::RemoveLiquidity2Row> = sqlx::query_as(
            r#"SELECT * FROM remove_liquidity2_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn remove_liquidity_by_range(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::RemoveLiquidityByRangeGraphQL>> {
        let rows: Vec<crate::instructions::postgres::RemoveLiquidityByRangeRow> = sqlx::query_as(
            r#"SELECT * FROM remove_liquidity_by_range_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_remove_liquidity_by_range(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::RemoveLiquidityByRangeGraphQL>> {
        let rows: Vec<crate::instructions::postgres::RemoveLiquidityByRangeRow> = sqlx::query_as(
            r#"SELECT * FROM remove_liquidity_by_range_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn remove_liquidity_by_range2(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::RemoveLiquidityByRange2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::RemoveLiquidityByRange2Row> = sqlx::query_as(
            r#"SELECT * FROM remove_liquidity_by_range2_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_remove_liquidity_by_range2(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::RemoveLiquidityByRange2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::RemoveLiquidityByRange2Row> = sqlx::query_as(
            r#"SELECT * FROM remove_liquidity_by_range2_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn set_activation_point(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SetActivationPointGraphQL>> {
        let rows: Vec<crate::instructions::postgres::SetActivationPointRow> = sqlx::query_as(
            r#"SELECT * FROM set_activation_point_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_set_activation_point(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SetActivationPointGraphQL>> {
        let rows: Vec<crate::instructions::postgres::SetActivationPointRow> = sqlx::query_as(
            r#"SELECT * FROM set_activation_point_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn set_pair_status(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SetPairStatusGraphQL>> {
        let rows: Vec<crate::instructions::postgres::SetPairStatusRow> = sqlx::query_as(
            r#"SELECT * FROM set_pair_status_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_set_pair_status(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SetPairStatusGraphQL>> {
        let rows: Vec<crate::instructions::postgres::SetPairStatusRow> = sqlx::query_as(
            r#"SELECT * FROM set_pair_status_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn set_pair_status_permissionless(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SetPairStatusPermissionlessGraphQL>> {
        let rows: Vec<crate::instructions::postgres::SetPairStatusPermissionlessRow> = sqlx::query_as(
            r#"SELECT * FROM set_pair_status_permissionless_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_set_pair_status_permissionless(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SetPairStatusPermissionlessGraphQL>> {
        let rows: Vec<crate::instructions::postgres::SetPairStatusPermissionlessRow> = sqlx::query_as(
            r#"SELECT * FROM set_pair_status_permissionless_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn set_pre_activation_duration(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SetPreActivationDurationGraphQL>> {
        let rows: Vec<crate::instructions::postgres::SetPreActivationDurationRow> = sqlx::query_as(
            r#"SELECT * FROM set_pre_activation_duration_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_set_pre_activation_duration(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SetPreActivationDurationGraphQL>> {
        let rows: Vec<crate::instructions::postgres::SetPreActivationDurationRow> = sqlx::query_as(
            r#"SELECT * FROM set_pre_activation_duration_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn set_pre_activation_swap_address(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SetPreActivationSwapAddressGraphQL>> {
        let rows: Vec<crate::instructions::postgres::SetPreActivationSwapAddressRow> = sqlx::query_as(
            r#"SELECT * FROM set_pre_activation_swap_address_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_set_pre_activation_swap_address(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SetPreActivationSwapAddressGraphQL>> {
        let rows: Vec<crate::instructions::postgres::SetPreActivationSwapAddressRow> = sqlx::query_as(
            r#"SELECT * FROM set_pre_activation_swap_address_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn swap(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SwapGraphQL>> {
        let rows: Vec<crate::instructions::postgres::SwapRow> = sqlx::query_as(
            r#"SELECT * FROM swap_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_swap(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SwapGraphQL>> {
        let rows: Vec<crate::instructions::postgres::SwapRow> = sqlx::query_as(
            r#"SELECT * FROM swap_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn swap2(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::Swap2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::Swap2Row> = sqlx::query_as(
            r#"SELECT * FROM swap2_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_swap2(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::Swap2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::Swap2Row> = sqlx::query_as(
            r#"SELECT * FROM swap2_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn swap_exact_out(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SwapExactOutGraphQL>> {
        let rows: Vec<crate::instructions::postgres::SwapExactOutRow> = sqlx::query_as(
            r#"SELECT * FROM swap_exact_out_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_swap_exact_out(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SwapExactOutGraphQL>> {
        let rows: Vec<crate::instructions::postgres::SwapExactOutRow> = sqlx::query_as(
            r#"SELECT * FROM swap_exact_out_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn swap_exact_out2(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SwapExactOut2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::SwapExactOut2Row> = sqlx::query_as(
            r#"SELECT * FROM swap_exact_out2_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_swap_exact_out2(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SwapExactOut2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::SwapExactOut2Row> = sqlx::query_as(
            r#"SELECT * FROM swap_exact_out2_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn swap_with_price_impact(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SwapWithPriceImpactGraphQL>> {
        let rows: Vec<crate::instructions::postgres::SwapWithPriceImpactRow> = sqlx::query_as(
            r#"SELECT * FROM swap_with_price_impact_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_swap_with_price_impact(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SwapWithPriceImpactGraphQL>> {
        let rows: Vec<crate::instructions::postgres::SwapWithPriceImpactRow> = sqlx::query_as(
            r#"SELECT * FROM swap_with_price_impact_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn swap_with_price_impact2(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SwapWithPriceImpact2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::SwapWithPriceImpact2Row> = sqlx::query_as(
            r#"SELECT * FROM swap_with_price_impact2_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_swap_with_price_impact2(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SwapWithPriceImpact2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::SwapWithPriceImpact2Row> = sqlx::query_as(
            r#"SELECT * FROM swap_with_price_impact2_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn update_base_fee_parameters(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::UpdateBaseFeeParametersGraphQL>> {
        let rows: Vec<crate::instructions::postgres::UpdateBaseFeeParametersRow> = sqlx::query_as(
            r#"SELECT * FROM update_base_fee_parameters_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_update_base_fee_parameters(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::UpdateBaseFeeParametersGraphQL>> {
        let rows: Vec<crate::instructions::postgres::UpdateBaseFeeParametersRow> = sqlx::query_as(
            r#"SELECT * FROM update_base_fee_parameters_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn update_dynamic_fee_parameters(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::UpdateDynamicFeeParametersGraphQL>> {
        let rows: Vec<crate::instructions::postgres::UpdateDynamicFeeParametersRow> = sqlx::query_as(
            r#"SELECT * FROM update_dynamic_fee_parameters_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_update_dynamic_fee_parameters(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::UpdateDynamicFeeParametersGraphQL>> {
        let rows: Vec<crate::instructions::postgres::UpdateDynamicFeeParametersRow> = sqlx::query_as(
            r#"SELECT * FROM update_dynamic_fee_parameters_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn update_fees_and_reward2(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::UpdateFeesAndReward2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::UpdateFeesAndReward2Row> = sqlx::query_as(
            r#"SELECT * FROM update_fees_and_reward2_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_update_fees_and_reward2(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::UpdateFeesAndReward2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::UpdateFeesAndReward2Row> = sqlx::query_as(
            r#"SELECT * FROM update_fees_and_reward2_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                        async fn update_position_operator(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::UpdatePositionOperatorGraphQL>> {
        let rows: Vec<crate::instructions::postgres::UpdatePositionOperatorRow> = sqlx::query_as(
            r#"SELECT * FROM update_position_operator_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_update_position_operator(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::UpdatePositionOperatorGraphQL>> {
        let rows: Vec<crate::instructions::postgres::UpdatePositionOperatorRow> = sqlx::query_as(
            r#"SELECT * FROM update_position_operator_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn update_reward_duration(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::UpdateRewardDurationGraphQL>> {
        let rows: Vec<crate::instructions::postgres::UpdateRewardDurationRow> = sqlx::query_as(
            r#"SELECT * FROM update_reward_duration_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_update_reward_duration(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::UpdateRewardDurationGraphQL>> {
        let rows: Vec<crate::instructions::postgres::UpdateRewardDurationRow> = sqlx::query_as(
            r#"SELECT * FROM update_reward_duration_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn update_reward_funder(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::UpdateRewardFunderGraphQL>> {
        let rows: Vec<crate::instructions::postgres::UpdateRewardFunderRow> = sqlx::query_as(
            r#"SELECT * FROM update_reward_funder_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_update_reward_funder(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::UpdateRewardFunderGraphQL>> {
        let rows: Vec<crate::instructions::postgres::UpdateRewardFunderRow> = sqlx::query_as(
            r#"SELECT * FROM update_reward_funder_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn withdraw_ineligible_reward(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::WithdrawIneligibleRewardGraphQL>> {
        let rows: Vec<crate::instructions::postgres::WithdrawIneligibleRewardRow> = sqlx::query_as(
            r#"SELECT * FROM withdraw_ineligible_reward_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_withdraw_ineligible_reward(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::WithdrawIneligibleRewardGraphQL>> {
        let rows: Vec<crate::instructions::postgres::WithdrawIneligibleRewardRow> = sqlx::query_as(
            r#"SELECT * FROM withdraw_ineligible_reward_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
                async fn withdraw_protocol_fee(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::WithdrawProtocolFeeGraphQL>> {
        let rows: Vec<crate::instructions::postgres::WithdrawProtocolFeeRow> = sqlx::query_as(
            r#"SELECT * FROM withdraw_protocol_fee_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }

    async fn list_withdraw_protocol_fee(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::WithdrawProtocolFeeGraphQL>> {
        let rows: Vec<crate::instructions::postgres::WithdrawProtocolFeeRow> = sqlx::query_as(
            r#"SELECT * FROM withdraw_protocol_fee_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().filter_map(|row| row.try_into().ok()).collect())
    }
        }

