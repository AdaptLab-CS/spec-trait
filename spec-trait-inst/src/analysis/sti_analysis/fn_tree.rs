use super::distance::Node;
use super::Analyzer;
use rustc_hir::intravisit::Visitor;

/// Different types of nodes in the body of a function.
#[derive(Eq, PartialEq, Debug)]
pub enum NodeType {
    // No children
    Use(String), // use path

    // Single child (expression)
    Static(String), // static name

    // Single child (expression)
    Const(String), // constant name

    // Single child (expression)
    Let(String), // variable name

    // Single child (expression)
    Assign(String), // variable name

    // No children
    Lit(String), // literal value

    // Multiple children (operands)
    Op(String), // operator symbol

    // Multiple children (expressions)
    Block,

    // Multiple children (expressions)
    Loop(Box<NodeType>), // loop condition

    // Multiple children (expressions)
    If(Box<NodeType>), // expression condition

    // Multiple children (expressions / if)
    Else,

    // Multiple children (match arms)
    Match(Box<NodeType>), // expression being matched

    // Multiple children (expressions)
    MatchArm(
        String,        // pattern description
        Box<NodeType>, // expression guard
    ),

    // Single child (expression)
    Jump(String), // break / continue / return / yield

    // Multiple children (body, optional for declarations)
    FnDef(
        String, // function name (optional for closures)
        usize,  // number of parameters
    ),

    // Multiple children (arguments)
    FnCall(String), // function name

    // Multiple children (arguments)
    MethodCall(
        String,        // method name
        Box<NodeType>, // receiver
    ),

    // No children
    Enum(
        String, // enum name
        usize,  // number of variants
    ),

    // No children
    Struct(
        String, // struct name
        usize,  // number of fields
    ),

    // No children
    Union(
        String, // union name
        usize,  // number of fields
    ),

    // Multiple children (items)
    Trait(String), // trait name

    // Multiple children (items)
    Impl(
        String, // trait name (without generics)
        String, // type name
    ),

    // Multiple children (expressions)
    Macro(String), // macro name
}

pub struct FnTree<'tcx, 'a> {
    pub analyzer: &'a Analyzer<'tcx>,

    pub root: Node<NodeType>,
}

impl<'tcx> Visitor<'tcx> for FnTree<'tcx, '_> {}
