//! # Schema Module
//!
//! The `schema` module provides macros for constructing and organizing
//! transaction schemas in a hierarchical manner. These macros are designed to
//! simplify the creation of complex transaction structures by allowing inline
//! specification of schema elements, which represent instructions
//! within transactions.
//!
//! ## Overview
//!
//! This module includes two primary macros:
//! - **`schema!`**: The main macro for constructing a `TransactionSchema`,
//!   which is a hierarchical schema representation.
//! - **`schema_inner!`**: A helper macro, used internally by `schema!` for
//!   recursive schema node construction.
//!
//! Together, these macros enable you to define schema nodes in a flexible and
//! intuitive manner, allowing for combinations of `Any` and `Instruction` nodes
//! with optional nested instructions.
//!
//! ## Key Macros
//!
//! ### `schema!`
//!
//! The `schema!` macro is the primary entry point for constructing a
//! `TransactionSchema`. It parses provided tokens into a `TransactionSchema`
//! object, allowing inline definition of various schema elements in a tree-like
//! structure. This macro supports keywords like `any` to create branches that
//! can match multiple instruction types.
//!
//! #### Example
//!
//! ```ignore
//! use your_crate::schema;
//!
//! let transaction_schema = schema![
//!     any,
//!     [
//!         AllInstructionTypes::JupSwap(JupiterInstructionType::SwapEvent),
//!         "jup_swap_event",
//!         []
//!     ],
//!     any,
//! ];
//! ```
//!
//! This example defines a schema with an `any` branch, an `Instruction` node
//! with nested instructions, and another `any` branch, creating a flexible
//! transaction structure. In practical terms, this means that the schema
//! represents a transaction that has a Jupiter Swap Event instruction anywhere
//! within the transaction.
//!
//! ### `schema_inner!`
//!
//! This macro is used internally by `schema!` to build out individual
//! `SchemaNode` elements. It supports three main syntax patterns: `any`, single
//! `Instruction` nodes, and nested `Instruction` nodes. Users typically don’t
//! need to interact with this macro directly, as it’s invoked by `schema!`
//! to handle recursive node construction.
//!
//! #### Supported Syntax Patterns
//!
//! 1. **`any`**: Adds an `Any` node, which can match any instruction type,
//!    multiple times.
//! 2. **`[$ix_type:expr, $name:expr]`**: Adds an `Instruction` node without
//!    nested instructions.
//! 3. **`[$ix_type:expr, $name:expr, [$($inner:tt)*]]`**: Adds an `Instruction`
//!    node with nested inner instructions.
//!
//! ## Notes
//!
//! - The `schema!` macro relies on `schema_inner!` for recursive parsing and
//!   node creation.
//! - When using the `schema!` macro, ensure that all instruction types and
//!   identifiers correspond to valid values expected by the `TransactionSchema`
//!   to avoid compilation errors.
//!
//! ## Crate Dependencies
//!
//! This module relies on components like `SchemaNode` and
//! `InstructionSchemaNode` to build the schema tree. Make sure these types are
//! defined and accessible within the scope of the module’s usage.

/// Constructs a `TransactionSchema` from provided tokens.
///
/// The `schema!` macro facilitates the creation of a `TransactionSchema` by
/// parsing the provided tokens and assembling the structure. This macro
/// simplifies schema construction by allowing inline specification of schema
/// elements, including instruction types and associated names.
///
/// # Syntax
///
/// The `schema!` macro uses a flexible token-based syntax, enabling you to
/// define schema nodes inline. Nodes are defined in a hierarchical manner, with
/// keywords like `any` indicating schema branches, followed by instruction
/// types, string identifiers, and additional attributes or sub-nodes as needed.
///
/// # Example
///
/// ```ignore
/// use carbon_macros::schema;
/// use carbon_macros::schema_inner;
///
/// let transaction_schema = schema![
///     any
///     [
///         AllInstructionTypes::JupSwap(JupiterInstructionType::SwapEvent),
///         "jup_swap_event",
///         []
///     ]
///     any
/// ];
/// ```
///
/// # Parameters
///
/// - `$tt`: The token stream passed to the macro, which may include multiple
///   schema nodes and attributes. These tokens are parsed to construct the
///   schema tree, forming a `TransactionSchema` object with a root node
///   containing the specified elements.
///
/// # Returns
///
/// This macro returns a `TransactionSchema` instance with the constructed
/// schema tree based on the tokens provided. The schema is organized with nodes
/// added to the root, allowing for complex, multi-layered transaction
/// structures, which represent real transaction instructions, in order.
///
/// # Notes
///
/// - This macro requires that the inner macro `schema_inner!` is also defined,
///   as it handles the recursive parsing and node addition to the schema.
/// - Ensure that types and keywords used within the schema correspond to valid
///   identifiers and instructions expected by `TransactionSchema`. Inconsistent
///   tokens may lead to compilation errors or incorrect schema construction.
#[macro_export]
macro_rules! schema {
    ($($tt:tt)*) => {{
        let mut nodes = Vec::new();
        schema_inner!(&mut nodes, $($tt)*);
        carbon_core::schema::TransactionSchema { root: nodes }
    }};
}

/// Recursively constructs schema nodes within the `schema!` macro.
///
/// The `schema_inner!` macro is utilized internally by the `schema!` macro to
/// build a `TransactionSchema` structure by parsing tokens into individual
/// `SchemaNode` elements. This macro supports multiple node types, including
/// `Any` nodes and `Instruction` nodes with optional nested instructions. Each
/// parsed node is appended to the provided vector, forming a hierarchical
/// schema structure.
///
/// # Syntax
///
/// This macro supports three main syntax patterns:
///
/// 1. `any`: Adds an `Any` node to the schema, indicating a branch point that
///    accepts multiple instructions of any type.
/// 2. `[$ix_type:expr, $name:expr]`: Adds an `Instruction` node with the
///    specified instruction type and name, without nested instructions.
/// 3. `[$ix_type:expr, $name:expr, [$($inner:tt)*]]`: Adds an `Instruction`
///    node with the specified instruction type and name, including inner
///    instructions which are parsed recursively.
///
/// # Parameters
///
/// - `$nodes`: A mutable reference to a `Vec<SchemaNode>`. This vector
///   accumulates the constructed schema nodes, which can include both `Any` and
///   `Instruction` node variants with nested instructions.
/// - `$ix_type`: The instruction type for the node, provided as an expression.
///   This identifies the specific instruction type in the schema.
/// - `$name`: A string literal representing the name of the instruction. This
///   name is associated with the `Instruction` node for identification.
///
/// # Returns
///
/// This macro modifies the `Vec<SchemaNode>` passed as `$nodes`, adding each
/// parsed `SchemaNode` to the vector. Nodes can be nested, with the structure
/// reflecting any provided inner instructions.
///
/// # Notes
///
/// - This macro is intended for internal use within the `schema!` macro and is
///   not typically called directly by users.
/// - Ensure that each `$ix_type` corresponds to a valid instruction type and
///   that `$name` is a string literal for compatibility with
///   `InstructionSchemaNode`.
/// - The recursive structure allows for complex, multi-level instruction
///   schemas suitable for detailed transaction validation and processing.
#[macro_export]
macro_rules! schema_inner {
    ($nodes:expr, ) => {};

    ($nodes:expr, any $($rest:tt)*) => {
        $nodes.push(carbon_core::schema::SchemaNode::Any);
        schema_inner!($nodes, $($rest)*);
    };

    ($nodes:expr, [$ix_type:expr, $name:expr] $($rest:tt)*) => {
        $nodes.push(carbon_core::schema::SchemaNode::Instruction(carbon_core::schema::InstructionSchemaNode {
            ix_type: $ix_type,
            name: $name.to_string(),
            inner_instructions: Vec::new(),
        }));
        schema_inner!($nodes, $($rest)*);
    };

    ($nodes:expr, [$ix_type:expr, $name:expr, [$($inner:tt)*]] $($rest:tt)*) => {{
        let mut inner_nodes = Vec::new();
        schema_inner!(&mut inner_nodes, $($inner)*);
        $nodes.push(carbon_core::schema::SchemaNode::Instruction(carbon_core::schema::InstructionSchemaNode {
            ix_type: $ix_type,
            name: $name.to_string(),
            inner_instructions: inner_nodes,
            }));
        schema_inner!($nodes, $($rest)*);
    }};
}
