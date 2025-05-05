use juniper_codegen::graphql_object;


pub struct Immutable;

#[graphql_object]
impl Immutable {
    fn api_version() -> &'static str {
        "1.0"
    }
}
