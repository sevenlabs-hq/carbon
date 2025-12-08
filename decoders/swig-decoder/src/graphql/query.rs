//! GraphQL QueryRoot for swig-decoder
//! Provides queries for all 13 instruction types (no accounts or instruction arguments)
use juniper::{graphql_object, FieldResult};

pub struct QueryRoot;

#[graphql_object(context = crate::graphql::context::GraphQLContext)]
impl QueryRoot {
    // Instructions - CreateV1
    async fn create_v1(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreateV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::create_v1_row::CreateV1Row> = sqlx::query_as(
            r#"SELECT * FROM create_v1_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
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

    async fn list_create_v1(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreateV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::create_v1_row::CreateV1Row> = sqlx::query_as(
            r#"SELECT * FROM create_v1_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
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

    // Instructions - AddAuthorityV1
    async fn add_authority_v1(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddAuthorityV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::add_authority_v1_row::AddAuthorityV1Row> = sqlx::query_as(
            r#"SELECT * FROM add_authority_v1_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
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

    async fn list_add_authority_v1(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::AddAuthorityV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::add_authority_v1_row::AddAuthorityV1Row> = sqlx::query_as(
            r#"SELECT * FROM add_authority_v1_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
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

    // Instructions - RemoveAuthorityV1
    async fn remove_authority_v1(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::RemoveAuthorityV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::remove_authority_v1_row::RemoveAuthorityV1Row> = sqlx::query_as(
            r#"SELECT * FROM remove_authority_v1_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
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

    async fn list_remove_authority_v1(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::RemoveAuthorityV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::remove_authority_v1_row::RemoveAuthorityV1Row> = sqlx::query_as(
            r#"SELECT * FROM remove_authority_v1_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
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

    // Instructions - UpdateAuthorityV1
    async fn update_authority_v1(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::UpdateAuthorityV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::update_authority_v1_row::UpdateAuthorityV1Row> = sqlx::query_as(
            r#"SELECT * FROM update_authority_v1_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
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

    async fn list_update_authority_v1(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::UpdateAuthorityV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::update_authority_v1_row::UpdateAuthorityV1Row> = sqlx::query_as(
            r#"SELECT * FROM update_authority_v1_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
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

    // Instructions - SignV1
    async fn sign_v1(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SignV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::sign_v1_row::SignV1Row> = sqlx::query_as(
            r#"SELECT * FROM sign_v1_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
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

    async fn list_sign_v1(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SignV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::sign_v1_row::SignV1Row> = sqlx::query_as(
            r#"SELECT * FROM sign_v1_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
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

    // Instructions - SignV2
    async fn sign_v2(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SignV2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::sign_v2_row::SignV2Row> = sqlx::query_as(
            r#"SELECT * FROM sign_v2_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
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

    async fn list_sign_v2(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SignV2GraphQL>> {
        let rows: Vec<crate::instructions::postgres::sign_v2_row::SignV2Row> = sqlx::query_as(
            r#"SELECT * FROM sign_v2_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
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

    // Instructions - CreateSessionV1
    async fn create_session_v1(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreateSessionV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::create_session_v1_row::CreateSessionV1Row> = sqlx::query_as(
            r#"SELECT * FROM create_session_v1_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
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

    async fn list_create_session_v1(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreateSessionV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::create_session_v1_row::CreateSessionV1Row> = sqlx::query_as(
            r#"SELECT * FROM create_session_v1_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
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

    // Instructions - CreateSubAccountV1
    async fn create_sub_account_v1(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreateSubAccountV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::create_sub_account_v1_row::CreateSubAccountV1Row> = sqlx::query_as(
            r#"SELECT * FROM create_sub_account_v1_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
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

    async fn list_create_sub_account_v1(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::CreateSubAccountV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::create_sub_account_v1_row::CreateSubAccountV1Row> = sqlx::query_as(
            r#"SELECT * FROM create_sub_account_v1_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
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

    // Instructions - WithdrawFromSubAccountV1
    async fn withdraw_from_sub_account_v1(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::WithdrawFromSubAccountV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::withdraw_from_sub_account_v1_row::WithdrawFromSubAccountV1Row> = sqlx::query_as(
            r#"SELECT * FROM withdraw_from_sub_account_v1_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
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

    async fn list_withdraw_from_sub_account_v1(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::WithdrawFromSubAccountV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::withdraw_from_sub_account_v1_row::WithdrawFromSubAccountV1Row> = sqlx::query_as(
            r#"SELECT * FROM withdraw_from_sub_account_v1_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
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

    // Instructions - SubAccountSignV1
    async fn sub_account_sign_v1(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SubAccountSignV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::sub_account_sign_v1_row::SubAccountSignV1Row> = sqlx::query_as(
            r#"SELECT * FROM sub_account_sign_v1_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
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

    async fn list_sub_account_sign_v1(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::SubAccountSignV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::sub_account_sign_v1_row::SubAccountSignV1Row> = sqlx::query_as(
            r#"SELECT * FROM sub_account_sign_v1_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
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

    // Instructions - ToggleSubAccountV1
    async fn toggle_sub_account_v1(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::ToggleSubAccountV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::toggle_sub_account_v1_row::ToggleSubAccountV1Row> = sqlx::query_as(
            r#"SELECT * FROM toggle_sub_account_v1_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
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

    async fn list_toggle_sub_account_v1(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::ToggleSubAccountV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::toggle_sub_account_v1_row::ToggleSubAccountV1Row> = sqlx::query_as(
            r#"SELECT * FROM toggle_sub_account_v1_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
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

    // Instructions - MigrateToWalletAddressV1
    async fn migrate_to_wallet_address_v1(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::MigrateToWalletAddressV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::migrate_to_wallet_address_v1_row::MigrateToWalletAddressV1Row> = sqlx::query_as(
            r#"SELECT * FROM migrate_to_wallet_address_v1_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
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

    async fn list_migrate_to_wallet_address_v1(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::MigrateToWalletAddressV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::migrate_to_wallet_address_v1_row::MigrateToWalletAddressV1Row> = sqlx::query_as(
            r#"SELECT * FROM migrate_to_wallet_address_v1_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
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

    // Instructions - TransferAssetsV1
    async fn transfer_assets_v1(
        context: &crate::graphql::context::GraphQLContext,
        signature: String,
        instruction_index: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::TransferAssetsV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::transfer_assets_v1_row::TransferAssetsV1Row> = sqlx::query_as(
            r#"SELECT * FROM transfer_assets_v1_instruction WHERE __signature = $1 AND __instruction_index = $2 ORDER BY __stack_height ASC"#,
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

    async fn list_transfer_assets_v1(
        context: &crate::graphql::context::GraphQLContext,
        limit: i32,
        offset: i32,
    ) -> FieldResult<Vec<crate::instructions::graphql::TransferAssetsV1GraphQL>> {
        let rows: Vec<crate::instructions::postgres::transfer_assets_v1_row::TransferAssetsV1Row> = sqlx::query_as(
            r#"SELECT * FROM transfer_assets_v1_instruction ORDER BY __slot DESC, __signature DESC, __instruction_index ASC LIMIT $1 OFFSET $2"#,
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
