// -------------------------------------------- Works without struct generation

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

// ------------------------------------ work from friday

// #[macro_export]
// macro_rules! generate_output_struct {
//     ($struct_name:ident, $( $nodes:tt )* ) => {
//         #[derive(serde::Deserialize)]
//         pub struct $struct_name {
//             $(
//                 collect_instruction_names!( $nodes )
//             )*
//         }
//     };
// }

// #[macro_export]
// macro_rules! collect_instruction_names {
//     (any) => {};
//     ([$ix_type:expr, $name:ident, [$($inner:tt)*]]) => {
//         pub $name: (AllInstructions, DecodedInstruction),
//         $(
//             $crate::collect_instruction_names!( $inner )
//         )*
//     };
//     ([$ix_type:expr, $name:ident, []]) => {
//         pub $name: (AllInstructions, DecodedInstruction),
//     };
// }

// #[macro_export]
// macro_rules! generate_output_struct {
//     ($struct_name:ident, $($tt:tt)*) => {
//         generate_output_struct_inner!($struct_name, (), $($tt)*);
//     };
// }

// #[macro_export]
// macro_rules! generate_output_struct_inner {
//     ($struct_name:ident, ($($fields:tt)*), ) => {
//         #[derive(Debug, Clone, serde::Deserialize)]
//         pub struct $struct_name {
//             $($fields)*
//         }
//     };

//     ($struct_name:ident, ($($fields:tt)*), any $($rest:tt)*) => {
//         generate_output_struct_inner!($struct_name, ($($fields)*), $($rest)*);
//     };

//     ($struct_name:ident, ($($fields:tt)*), [$ix_type:expr, $name:ident] $($rest:tt)*) => {
//         generate_output_struct_inner!(
//             $struct_name,
//             ($($fields)* pub $name: (AllInstructionTypes, DecodedInstruction),),
//             $($rest)*
//         );
//     };

//     ($struct_name:ident, ($($fields:tt)*), [$ix_type:expr, $name:ident, [$($inner:tt)*]] $($rest:tt)*) => {{
//         generate_output_struct_inner!(
//             $struct_name,
//             ($($fields)*),
//             $($inner)*
//         );
//         generate_output_struct_inner!(
//             $struct_name,
//             ($($fields)* pub $name: (AllInstructionTypes, DecodedInstruction),),
//             $($rest)*
//         );
//     }};

//     ($struct_name:ident, ($($fields:tt)*), [] $($rest:tt)*) => {
//         generate_output_struct_inner!($struct_name, ($($fields)*), $($rest)*);
//     };

//     ($struct_name:ident, ($($fields:tt)*), $unexpected:tt $($rest:tt)*) => {
//         generate_output_struct_inner!($struct_name, ($($fields)*), $($rest)*);
//     };
// }

// -------------------------------------------- this kinda would work possibly but it would create many structs with 1 field

// #[macro_export]
// macro_rules! schema {
//     // Handle struct creation with schema
//     ($struct_name:ident, $($tt:tt)*) => {{
//         let mut nodes = Vec::new();
//         schema_inner!(&mut nodes, $struct_name, $($tt)*);

//         // Return the TransactionSchema
//         TransactionSchema { root: nodes }
//     }};

//     // Case without struct creation
//     ($($tt:tt)*) => {{
//         let mut nodes = Vec::new();
//         schema_inner!(&mut nodes, _, $($tt)*);
//         TransactionSchema { root: nodes }
//     }};
// }

// #[macro_export]
// macro_rules! schema_inner {
//     // Base case for the end of recursion
//     ($nodes:expr, $struct_name:ident, ) => {};

//     // Handle `any` node
//     ($nodes:expr, $struct_name:ident, any $($rest:tt)*) => {
//         $nodes.push(SchemaNode::Any);
//         schema_inner!($nodes, $struct_name, $($rest)*);
//     };

//     // Collect instruction info and generate field for the struct
//     ($nodes:expr, $struct_name:ident, [$ix_type:expr, $name:ident] $($rest:tt)*) => {
//         $nodes.push(SchemaNode::Instruction(InstructionSchemaNode {
//             ix_type: $ix_type,
//             name: stringify!($name).to_string(),
//             inner_instructions: Vec::new(),
//         }));

//         // Generate the field for the struct
//         schema_inner_struct_field!($struct_name, $name);

//         schema_inner!($nodes, $struct_name, $($rest)*);
//     };

//     // Handle nested instructions
//     ($nodes:expr, $struct_name:ident, [$ix_type:expr, $name:ident, [$($inner:tt)*]] $($rest:tt)*) => {{
//         let mut inner_nodes = Vec::new();
//         schema_inner!(&mut inner_nodes, $struct_name, $($inner)*);
//         $nodes.push(SchemaNode::Instruction(InstructionSchemaNode {
//             ix_type: $ix_type,
//             name: stringify!($name).to_string(),
//             inner_instructions: inner_nodes,
//         }));

//         // Generate the field for the struct
//         schema_inner_struct_field!($struct_name, $name);

//         schema_inner!($nodes, $struct_name, $($rest)*);
//     }};
// }

// // Separate macro to generate fields in the struct
// #[macro_export]
// macro_rules! schema_inner_struct_field {
//     ($struct_name:ident, $field:ident) => {
//         pub struct $struct_name {
//             pub $field: AllInstructions,
//         }
//     };
// }

// ---------------------------

// #[macro_export]
// macro_rules! schema {
//     // Main entry point: Generate both the struct and the schema
//     ($struct_name:ident, $($schema_nodes:tt)*) => {{
//         // First, collect the fields and define the struct
//         schema_struct!($struct_name, $($schema_nodes)*);

//         // Then, generate the schema
//         let mut nodes = Vec::new();
//         schema_inner!(&mut nodes, $($schema_nodes)*);
//         TransactionSchema { root: nodes }
//     }};

//     // Case without struct creation
//     ($($schema_nodes:tt)*) => {{
//         let mut nodes = Vec::new();
//         schema_inner!(&mut nodes, $($schema_nodes)*);
//         TransactionSchema { root: nodes }
//     }};
// }

// #[macro_export]
// macro_rules! schema_struct {
//     ($struct_name:ident, $($schema_nodes:tt)*) => {
//         pub struct $struct_name {
//             schema_collect_fields!($($schema_nodes)*)
//         }
//     };
// }

// // Helper macro to collect fields from schema nodes
// #[macro_export]
// macro_rules! schema_collect_fields {
//     () => {};  // Base case: no more fields

//     // Skip 'any' nodes for struct field generation
//     (any $($rest:tt)*) => {
//         schema_collect_fields!($($rest)*);
//     };

//     // Add a field for each instruction node
//     ([$_ix_type:expr, $field_name:ident] $($rest:tt)*) => {
//         pub $field_name: (AllInstructions, DecodedInstruction),
//         schema_collect_fields!($($rest)*);
//     };

//     // Handle nested instruction nodes and collect their fields
//     ([$_ix_type:expr, $field_name:ident, [$($inner:tt)*]] $($rest:tt)*) => {
//         pub $field_name: (AllInstructions, DecodedInstruction),
//         schema_collect_fields!($($inner)*);
//         schema_collect_fields!($($rest)*);
//     };
// }

// // Macro to generate the schema
// #[macro_export]
// macro_rules! schema_inner {
//     () => {};  // Base case for recursion

//     // Handle `any` nodes
//     ($nodes:expr, any $($rest:tt)*) => {
//         $nodes.push(SchemaNode::Any);
//         schema_inner!($nodes, $($rest)*);
//     };

//     // Handle instruction nodes
//     ($nodes:expr, [$ix_type:expr, $name:ident] $($rest:tt)*) => {
//         $nodes.push(SchemaNode::Instruction(InstructionSchemaNode {
//             ix_type: $ix_type,
//             name: stringify!($name).to_string(),
//             inner_instructions: Vec::new(),
//         }));
//         schema_inner!($nodes, $($rest)*);
//     };

//     // Handle nested instructions
//     ($nodes:expr, [$ix_type:expr, $name:ident, [$($inner:tt)*]] $($rest:tt)*) => {{
//         let mut inner_nodes = Vec::new();
//         schema_inner!(&mut inner_nodes, $($inner)*);
//         $nodes.push(SchemaNode::Instruction(InstructionSchemaNode {
//             ix_type: $ix_type,
//             name: stringify!($name).to_string(),
//             inner_instructions: inner_nodes,
//         }));
//         schema_inner!($nodes, $($rest)*);
//     }};

// }

// Call proc macro from macro

// #[macro_export]
// macro_rules! schema {
//     ($($tt:tt)*) => {{
//         let mut nodes = Vec::new();
//         let mut names = Vec::new();
//         schema_inner!(&mut nodes, &mut names, $($tt)*);
//         carbon_proc_macros::generate_struct!([names]);
//         TransactionSchema { root: nodes }
//     }};
// }
// #[macro_export]
// macro_rules! schema_inner {
//     ($nodes:expr, $names:expr, ) => {};

//     ($nodes:expr, $names:expr, any $($rest:tt)*) => {
//         $nodes.push(SchemaNode::Any);
//         schema_inner!($nodes, $names, $($rest)*);
//     };

//     ($nodes:expr, $names:expr, [$ix_type:expr, $name:expr] $($rest:tt)*) => {
//         $nodes.push(SchemaNode::Instruction(InstructionSchemaNode {
//             ix_type: $ix_type,
//             name: $name.to_string(),
//             inner_instructions: Vec::new(),
//         }));
//         $names.push($name);
//         schema_inner!($nodes, $names, $($rest)*);
//     };

//     ($nodes:expr, $names:expr, [$ix_type:expr, $name:expr, [$($inner:tt)*]] $($rest:tt)*) => {{
//         let mut inner_nodes = Vec::new();
//         let mut inner_names: Vec<&str> = Vec::new();
//         schema_inner!(&mut inner_nodes, &mut inner_names, $($inner)*);
//         $nodes.push(SchemaNode::Instruction(InstructionSchemaNode {
//             ix_type: $ix_type,
//             name: $name.to_string(),
//             inner_instructions: inner_nodes,
//         }));
//         $names.push($name);
//         schema_inner!($nodes, $names, $($rest)*);
//     }};
// }
