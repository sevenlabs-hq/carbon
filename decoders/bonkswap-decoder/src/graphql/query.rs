use juniper::{graphql_object, FieldResult};
use std::str::FromStr;

pub struct QueryRoot;

#[graphql_object(context = crate::graphql::context::GraphQLContext)]
impl QueryRoot {
    // Accounts
    async fn farm(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::FarmGraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(
            solana_pubkey::Pubkey::from_str(&pubkey)
                .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?,
        );
        let row = crate::accounts::postgres::FarmRow::lookup(pk, &context.pool)
            .await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.and_then(|row| row.try_into().ok()))
    }

    async fn list_farm(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::FarmGraphQL>> {
        let rows: Vec<crate::accounts::postgres::FarmRow> =
            sqlx::query_as(r#"SELECT * FROM farm_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#)
                .bind(limit)
                .bind(offset)
                .fetch_all(&*context.pool)
                .await
                .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }
    async fn pool_v2(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::PoolV2GraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(
            solana_pubkey::Pubkey::from_str(&pubkey)
                .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?,
        );
        let row = crate::accounts::postgres::PoolV2Row::lookup(pk, &context.pool)
            .await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.and_then(|row| row.try_into().ok()))
    }

    async fn list_pool_v2(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::PoolV2GraphQL>> {
        let rows: Vec<crate::accounts::postgres::PoolV2Row> = sqlx::query_as(
            r#"SELECT * FROM pool_v2_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }
    async fn pool(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::PoolGraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(
            solana_pubkey::Pubkey::from_str(&pubkey)
                .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?,
        );
        let row = crate::accounts::postgres::PoolRow::lookup(pk, &context.pool)
            .await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.and_then(|row| row.try_into().ok()))
    }

    async fn list_pool(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::PoolGraphQL>> {
        let rows: Vec<crate::accounts::postgres::PoolRow> =
            sqlx::query_as(r#"SELECT * FROM pool_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#)
                .bind(limit)
                .bind(offset)
                .fetch_all(&*context.pool)
                .await
                .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }
    async fn provider(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::ProviderGraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(
            solana_pubkey::Pubkey::from_str(&pubkey)
                .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?,
        );
        let row = crate::accounts::postgres::ProviderRow::lookup(pk, &context.pool)
            .await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.and_then(|row| row.try_into().ok()))
    }

    async fn list_provider(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::ProviderGraphQL>> {
        let rows: Vec<crate::accounts::postgres::ProviderRow> = sqlx::query_as(
            r#"SELECT * FROM provider_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }
    async fn state(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::StateGraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(
            solana_pubkey::Pubkey::from_str(&pubkey)
                .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?,
        );
        let row = crate::accounts::postgres::StateRow::lookup(pk, &context.pool)
            .await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.and_then(|row| row.try_into().ok()))
    }

    async fn list_state(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::StateGraphQL>> {
        let rows: Vec<crate::accounts::postgres::StateRow> = sqlx::query_as(
            r#"SELECT * FROM state_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }

    // Instructions (per-instruction list and lookup by signature+index)
    async fn create_pool(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreatePoolGraphQL>> {
        let rows: Vec<crate::instructions::postgres::CreatePoolRow> = sqlx::query_as(
            r#"SELECT * FROM create_pool_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }

    async fn list_create_pool(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreatePoolGraphQL>> {
        let rows: Vec<crate::instructions::postgres::CreatePoolRow> = sqlx::query_as(
            r#"SELECT * FROM create_pool_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }

    async fn create_provider(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreateProviderGraphQL>> {
        let rows: Vec<crate::instructions::postgres::CreateProviderRow> = sqlx::query_as(
            r#"SELECT * FROM create_provider_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }

    async fn list_create_provider(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreateProviderGraphQL>> {
        let rows: Vec<crate::instructions::postgres::CreateProviderRow> = sqlx::query_as(
            r#"SELECT * FROM create_provider_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }

    async fn create_state(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreateStateGraphQL>> {
        let rows: Vec<crate::instructions::postgres::CreateStateRow> = sqlx::query_as(
            r#"SELECT * FROM create_state_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }

    async fn list_create_state(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreateStateGraphQL>> {
        let rows: Vec<crate::instructions::postgres::CreateStateRow> = sqlx::query_as(
            r#"SELECT * FROM create_state_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }

    async fn add_tokens(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddTokensGraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddTokensRow> = sqlx::query_as(
            r#"SELECT * FROM add_tokens_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }

    async fn list_add_tokens(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddTokensGraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddTokensRow> = sqlx::query_as(
            r#"SELECT * FROM add_tokens_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
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
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
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
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }

    async fn withdraw_shares(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::WithdrawSharesGraphQL>> {
        let rows: Vec<crate::instructions::postgres::WithdrawSharesRow> = sqlx::query_as(
            r#"SELECT * FROM withdraw_shares_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }

    async fn list_withdraw_shares(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::WithdrawSharesGraphQL>> {
        let rows: Vec<crate::instructions::postgres::WithdrawSharesRow> = sqlx::query_as(
            r#"SELECT * FROM withdraw_shares_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }

    async fn create_farm(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreateFarmGraphQL>> {
        let rows: Vec<crate::instructions::postgres::CreateFarmRow> = sqlx::query_as(
            r#"SELECT * FROM create_farm_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }

    async fn list_create_farm(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreateFarmGraphQL>> {
        let rows: Vec<crate::instructions::postgres::CreateFarmRow> = sqlx::query_as(
            r#"SELECT * FROM create_farm_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }

    async fn create_dual_farm(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreateDualFarmGraphQL>> {
        let rows: Vec<crate::instructions::postgres::CreateDualFarmRow> = sqlx::query_as(
            r#"SELECT * FROM create_dual_farm_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }

    async fn list_create_dual_farm(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreateDualFarmGraphQL>> {
        let rows: Vec<crate::instructions::postgres::CreateDualFarmRow> = sqlx::query_as(
            r#"SELECT * FROM create_dual_farm_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }

    async fn create_triple_farm(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreateTripleFarmGraphQL>> {
        let rows: Vec<crate::instructions::postgres::CreateTripleFarmRow> = sqlx::query_as(
            r#"SELECT * FROM create_triple_farm_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }

    async fn list_create_triple_farm(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreateTripleFarmGraphQL>> {
        let rows: Vec<crate::instructions::postgres::CreateTripleFarmRow> = sqlx::query_as(
            r#"SELECT * FROM create_triple_farm_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }

    async fn add_supply(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddSupplyGraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddSupplyRow> = sqlx::query_as(
            r#"SELECT * FROM add_supply_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }

    async fn list_add_supply(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddSupplyGraphQL>> {
        let rows: Vec<crate::instructions::postgres::AddSupplyRow> = sqlx::query_as(
            r#"SELECT * FROM add_supply_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }

    async fn update_fees(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::UpdateFeesGraphQL>> {
        let rows: Vec<crate::instructions::postgres::UpdateFeesRow> = sqlx::query_as(
            r#"SELECT * FROM update_fees_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(instruction_index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }

    async fn list_update_fees(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::UpdateFeesGraphQL>> {
        let rows: Vec<crate::instructions::postgres::UpdateFeesRow> = sqlx::query_as(
            r#"SELECT * FROM update_fees_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect())
    }
}
