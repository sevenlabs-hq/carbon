use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{Datasource, DatasourceId, Update, UpdateType},
        error::CarbonResult,
    },
    carbon_helius_gpa_v2_datasource::{HeliusGpaV2Config, HeliusGpaV2Datasource},
    carbon_rpc_gpa_datasource::GpaDatasource,
    solana_pubkey::Pubkey,
    std::env,
    tokio::sync::mpsc::Sender,
    tokio_util::sync::CancellationToken,
};

pub enum GpaVariant {
    Rpc(GpaDatasource),
    HeliusGpaV2(HeliusGpaV2Datasource),
}

pub fn from_env(program: Pubkey) -> GpaVariant {
    match env::var("GPA_SOURCE")
        .unwrap_or_else(|_| "rpc".to_string())
        .as_str()
    {
        "helius-gpa-v2" => GpaVariant::HeliusGpaV2(helius_gpa_v2(program)),
        "rpc" => GpaVariant::Rpc(rpc(program)),
        other => panic!("unsupported GPA_SOURCE={other}; use rpc or helius-gpa-v2"),
    }
}

fn rpc(program: Pubkey) -> GpaDatasource {
    GpaDatasource::new(env::var("RPC_URL").expect("RPC_URL must be set"), program)
}

fn helius_gpa_v2(program: Pubkey) -> HeliusGpaV2Datasource {
    let config = HeliusGpaV2Config::new(None, None, Some(1000));

    HeliusGpaV2Datasource::new_with_config(
        env::var("HELIUS_RPC_URL").expect("HELIUS_RPC_URL must be set"),
        program,
        config,
    )
}

#[async_trait]
impl Datasource for GpaVariant {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
    ) -> CarbonResult<()> {
        match self {
            Self::Rpc(datasource) => datasource.consume(id, sender, cancellation_token).await,
            Self::HeliusGpaV2(datasource) => {
                datasource.consume(id, sender, cancellation_token).await
            }
        }
    }

    fn update_types(&self) -> Vec<UpdateType> {
        match self {
            Self::Rpc(datasource) => datasource.update_types(),
            Self::HeliusGpaV2(datasource) => datasource.update_types(),
        }
    }
}
