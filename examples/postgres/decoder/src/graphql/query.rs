use juniper::{graphql_object, FieldResult};
use std::str::FromStr;

pub struct QueryRoot;

#[graphql_object(context = crate::graphql::context::GraphQLContext)]
impl QueryRoot {
    // Accounts
            async fn bonding_curve(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::BondingCurveGraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(solana_pubkey::Pubkey::from_str(&pubkey).map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?);
        let row = crate::accounts::postgres::BondingCurveRow::lookup(pk, &context.pool).await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.map(Into::into))
    }

    async fn list_bonding_curve(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::BondingCurveGraphQL>> {
        let rows: Vec<crate::accounts::postgres::BondingCurveRow> = sqlx::query_as(
            r#"SELECT * FROM bonding_curve_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
                async fn fee_config(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::FeeConfigGraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(solana_pubkey::Pubkey::from_str(&pubkey).map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?);
        let row = crate::accounts::postgres::FeeConfigRow::lookup(pk, &context.pool).await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.map(Into::into))
    }

    async fn list_fee_config(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::FeeConfigGraphQL>> {
        let rows: Vec<crate::accounts::postgres::FeeConfigRow> = sqlx::query_as(
            r#"SELECT * FROM fee_config_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
                async fn global_config(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::GlobalConfigGraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(solana_pubkey::Pubkey::from_str(&pubkey).map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?);
        let row = crate::accounts::postgres::GlobalConfigRow::lookup(pk, &context.pool).await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.map(Into::into))
    }

    async fn list_global_config(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::GlobalConfigGraphQL>> {
        let rows: Vec<crate::accounts::postgres::GlobalConfigRow> = sqlx::query_as(
            r#"SELECT * FROM global_config_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
                async fn global_volume_accumulator(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::GlobalVolumeAccumulatorGraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(solana_pubkey::Pubkey::from_str(&pubkey).map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?);
        let row = crate::accounts::postgres::GlobalVolumeAccumulatorRow::lookup(pk, &context.pool).await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.map(Into::into))
    }

    async fn list_global_volume_accumulator(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::GlobalVolumeAccumulatorGraphQL>> {
        let rows: Vec<crate::accounts::postgres::GlobalVolumeAccumulatorRow> = sqlx::query_as(
            r#"SELECT * FROM global_volume_accumulator_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
                async fn pool(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::PoolGraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(solana_pubkey::Pubkey::from_str(&pubkey).map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?);
        let row = crate::accounts::postgres::PoolRow::lookup(pk, &context.pool).await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.map(Into::into))
    }

    async fn list_pool(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::PoolGraphQL>> {
        let rows: Vec<crate::accounts::postgres::PoolRow> = sqlx::query_as(
            r#"SELECT * FROM pool_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
                async fn user_volume_accumulator(
        context: &crate::graphql::context::GraphQLContext,
        pubkey: String,
    ) -> FieldResult<Option<crate::accounts::graphql::UserVolumeAccumulatorGraphQL>> {
        use carbon_core::postgres::operations::LookUp;
        use carbon_core::postgres::primitives::Pubkey as PgPubkey;
        let pk = PgPubkey(solana_pubkey::Pubkey::from_str(&pubkey).map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?);
        let row = crate::accounts::postgres::UserVolumeAccumulatorRow::lookup(pk, &context.pool).await
            .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(row.map(Into::into))
    }

    async fn list_user_volume_accumulator(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::accounts::graphql::UserVolumeAccumulatorGraphQL>> {
        let rows: Vec<crate::accounts::postgres::UserVolumeAccumulatorRow> = sqlx::query_as(
            r#"SELECT * FROM user_volume_accumulator_account ORDER BY __slot DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
        
    // Instructions (per-instruction list and lookup by signature+index)
            async fn admin_set_coin_creator(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AdminSetCoinCreatorGraphQL>> {
        let rows: Vec<crate::instructions::postgres::AdminSetCoinCreatorRow> = sqlx::query_as(
            r#"SELECT * FROM admin_set_coin_creator_instruction WHERE __signature = $1 AND __index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_admin_set_coin_creator(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AdminSetCoinCreatorGraphQL>> {
        let rows: Vec<crate::instructions::postgres::AdminSetCoinCreatorRow> = sqlx::query_as(
            r#"SELECT * FROM admin_set_coin_creator_instruction ORDER BY __slot DESC, __signature DESC, __index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
                async fn admin_update_token_incentives(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AdminUpdateTokenIncentivesGraphQL>> {
        let rows: Vec<crate::instructions::postgres::AdminUpdateTokenIncentivesRow> = sqlx::query_as(
            r#"SELECT * FROM admin_update_token_incentives_instruction WHERE __signature = $1 AND __index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_admin_update_token_incentives(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AdminUpdateTokenIncentivesGraphQL>> {
        let rows: Vec<crate::instructions::postgres::AdminUpdateTokenIncentivesRow> = sqlx::query_as(
            r#"SELECT * FROM admin_update_token_incentives_instruction ORDER BY __slot DESC, __signature DESC, __index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
                async fn buy(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::BuyGraphQL>> {
        let rows: Vec<crate::instructions::postgres::BuyRow> = sqlx::query_as(
            r#"SELECT * FROM buy_instruction WHERE __signature = $1 AND __index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_buy(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::BuyGraphQL>> {
        let rows: Vec<crate::instructions::postgres::BuyRow> = sqlx::query_as(
            r#"SELECT * FROM buy_instruction ORDER BY __slot DESC, __signature DESC, __index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
                                        async fn create_config(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreateConfigGraphQL>> {
        let rows: Vec<crate::instructions::postgres::CreateConfigRow> = sqlx::query_as(
            r#"SELECT * FROM create_config_instruction WHERE __signature = $1 AND __index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_create_config(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreateConfigGraphQL>> {
        let rows: Vec<crate::instructions::postgres::CreateConfigRow> = sqlx::query_as(
            r#"SELECT * FROM create_config_instruction ORDER BY __slot DESC, __signature DESC, __index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
                async fn create_pool(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreatePoolGraphQL>> {
        let rows: Vec<crate::instructions::postgres::CreatePoolRow> = sqlx::query_as(
            r#"SELECT * FROM create_pool_instruction WHERE __signature = $1 AND __index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_create_pool(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreatePoolGraphQL>> {
        let rows: Vec<crate::instructions::postgres::CreatePoolRow> = sqlx::query_as(
            r#"SELECT * FROM create_pool_instruction ORDER BY __slot DESC, __signature DESC, __index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
                async fn deposit(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::DepositGraphQL>> {
        let rows: Vec<crate::instructions::postgres::DepositRow> = sqlx::query_as(
            r#"SELECT * FROM deposit_instruction WHERE __signature = $1 AND __index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_deposit(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::DepositGraphQL>> {
        let rows: Vec<crate::instructions::postgres::DepositRow> = sqlx::query_as(
            r#"SELECT * FROM deposit_instruction ORDER BY __slot DESC, __signature DESC, __index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
                async fn disable(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::DisableGraphQL>> {
        let rows: Vec<crate::instructions::postgres::DisableRow> = sqlx::query_as(
            r#"SELECT * FROM disable_instruction WHERE __signature = $1 AND __index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_disable(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::DisableGraphQL>> {
        let rows: Vec<crate::instructions::postgres::DisableRow> = sqlx::query_as(
            r#"SELECT * FROM disable_instruction ORDER BY __slot DESC, __signature DESC, __index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
                                async fn sell(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SellGraphQL>> {
        let rows: Vec<crate::instructions::postgres::SellRow> = sqlx::query_as(
            r#"SELECT * FROM sell_instruction WHERE __signature = $1 AND __index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_sell(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SellGraphQL>> {
        let rows: Vec<crate::instructions::postgres::SellRow> = sqlx::query_as(
            r#"SELECT * FROM sell_instruction ORDER BY __slot DESC, __signature DESC, __index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
                                        async fn update_fee_config(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::UpdateFeeConfigGraphQL>> {
        let rows: Vec<crate::instructions::postgres::UpdateFeeConfigRow> = sqlx::query_as(
            r#"SELECT * FROM update_fee_config_instruction WHERE __signature = $1 AND __index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_update_fee_config(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::UpdateFeeConfigGraphQL>> {
        let rows: Vec<crate::instructions::postgres::UpdateFeeConfigRow> = sqlx::query_as(
            r#"SELECT * FROM update_fee_config_instruction ORDER BY __slot DESC, __signature DESC, __index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
                async fn withdraw(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::WithdrawGraphQL>> {
        let rows: Vec<crate::instructions::postgres::WithdrawRow> = sqlx::query_as(
            r#"SELECT * FROM withdraw_instruction WHERE __signature = $1 AND __index = $2 ORDER BY __stack_height ASC"#,
        )
        .bind(signature)
        .bind(index)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_withdraw(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::WithdrawGraphQL>> {
        let rows: Vec<crate::instructions::postgres::WithdrawRow> = sqlx::query_as(
            r#"SELECT * FROM withdraw_instruction ORDER BY __slot DESC, __signature DESC, __index ASC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*context.pool)
        .await
        .map_err(|e| juniper::FieldError::new(e.to_string(), juniper::Value::null()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
        }

