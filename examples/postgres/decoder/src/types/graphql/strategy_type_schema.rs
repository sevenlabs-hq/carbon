use juniper::GraphQLEnum;


#[derive(Debug, Clone, GraphQLEnum)]
#[graphql(name = "StrategyType")]
pub enum StrategyTypeGraphQL {
        SpotOneSide,
        CurveOneSide,
        BidAskOneSide,
        SpotBalanced,
        CurveBalanced,
        BidAskBalanced,
        SpotImBalanced,
        CurveImBalanced,
        BidAskImBalanced,
}

impl From<crate::types::StrategyType> for StrategyTypeGraphQL {
    fn from(original: crate::types::StrategyType) -> Self {
        match original {
            crate::types::StrategyType::SpotOneSide => Self::SpotOneSide,
            crate::types::StrategyType::CurveOneSide => Self::CurveOneSide,
            crate::types::StrategyType::BidAskOneSide => Self::BidAskOneSide,
            crate::types::StrategyType::SpotBalanced => Self::SpotBalanced,
            crate::types::StrategyType::CurveBalanced => Self::CurveBalanced,
            crate::types::StrategyType::BidAskBalanced => Self::BidAskBalanced,
            crate::types::StrategyType::SpotImBalanced => Self::SpotImBalanced,
            crate::types::StrategyType::CurveImBalanced => Self::CurveImBalanced,
            crate::types::StrategyType::BidAskImBalanced => Self::BidAskImBalanced,
        }
    }
}


