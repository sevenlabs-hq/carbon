#[macro_export]
macro_rules! ix {
    (
        name = $name:expr,
        type = $type:expr,
        iixs = [ $( ix!( $( $iixs:tt )* ) ),* ]
    ) => {
        SchemaNode {
            name: String::from($name),
            ix_type: $type,
            inner_instructions: vec![
                $( ix!( $( $iixs )* ) ),*
            ]
        }
    };

    (
        name = $name:expr,
        type = $type:expr
    ) => {
        SchemaNode {
            name: String::from($name),
            ix_type: $type,
            inner_instructions: vec![]
        }
    };
}

#[macro_export]
macro_rules! schema {
    ( $( $ix:expr ),* ) => {
        TransactionSchema {
            root: vec![
                $( $ix ),*
            ]
        }
    };
}
