#[macro_export]
macro_rules! define_schema {
    (instr $instruction_type:expr) => {
        SchemaNode::Instruction($instruction_type)
    };

    (any) => {
        SchemaNode::Any
    };

    (any_unparsed) => {
        SchemaNode::AnyUnparsed
    };

    (seq [ $($inner:tt)* ]) => {
        SchemaNode::Sequence(vec![ $(define_schema!($inner)),* ])
    };

    (one_of [ $($inner:tt)* ]) => {
        SchemaNode::OneOf(vec![ $(define_schema!($inner)),* ])
    };

    (zero_or_more ($($inner:tt)*)) => {
        SchemaNode::ZeroOrMore(Box::new(define_schema!($($inner)*)))
    };

    (one_or_more ($($inner:tt)*)) => {
        SchemaNode::OneOrMore(Box::new(define_schema!($($inner)*)))
    };

    (optional ($($inner:tt)*)) => {
        SchemaNode::Optional(Box::new(define_schema!($($inner)*)))
    };

    (nested ($($inner:tt)*)) => {
        SchemaNode::Nested(Box::new(define_schema!($($inner)*)))
    };
}

#[macro_export]
macro_rules! transaction_schema {
    ($($schema:tt)+) => {
        TransactionSchema {
            root: define_schema!($($schema)+)
        }
    };
}
