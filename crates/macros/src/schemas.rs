#[macro_export]
macro_rules! schema {
    ($($tt:tt)*) => {{
        let mut nodes = Vec::new();
        schema_inner!(&mut nodes, $($tt)*);
        TransactionSchema { root: nodes }
    }};
}

#[macro_export]
macro_rules! schema_inner {
    ($nodes:expr, ) => {};

    ($nodes:expr, any $($rest:tt)*) => {
        $nodes.push(SchemaNode::Any);
        schema_inner!($nodes, $($rest)*);
    };

    ($nodes:expr, [$ix_type:expr, $name:expr] $($rest:tt)*) => {
        $nodes.push(SchemaNode::Instruction(InstructionSchemaNode {
            ix_type: $ix_type,
            name: $name.to_string(),
            inner_instructions: Vec::new(),
        }));
        schema_inner!($nodes, $($rest)*);
    };

    ($nodes:expr, [$ix_type:expr, $name:expr, [$($inner:tt)*]] $($rest:tt)*) => {{
        let mut inner_nodes = Vec::new();
        schema_inner!(&mut inner_nodes, $($inner)*);
        $nodes.push(SchemaNode::Instruction(InstructionSchemaNode {
            ix_type: $ix_type,
            name: $name.to_string(),
            inner_instructions: inner_nodes,
        }));
        schema_inner!($nodes, $($rest)*);
    }};
}
