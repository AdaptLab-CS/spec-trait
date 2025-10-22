use super::distance::Node;
use super::Analyzer;
use rustc_ast::LitKind;
use rustc_hir::{BinOpKind, Block, Body, Expr, ExprKind, PatKind, QPath, Stmt, StmtKind};

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

    // No children
    Var(String), // variable / parameter name

    // Multiple children (operands)
    Op(String), // operator symbol

    // Multiple children (expressions)
    Block,

    // Multiple children (expressions)
    Loop,

    // Multiple children (expressions)
    If,

    // Multiple children (expressions / if)
    Else,

    // Multiple children (match arms)
    Match,

    // Multiple children (expressions)
    MatchArm(String), // pattern

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
    MethodCall(String), // method name

    // No children
    Enum(
        String, // enum name
        usize,  // number of variants
    ),

    // Multiple children (fields)
    Struct(String), // struct name

    // Multiple children (fields)
    Union(String), // union name

    // Two children (action, target)
    Action, // method call (target) / match (target) / match arm (guard) / if (condition) / loop (condition)

    Unknown,
}

pub struct FnTree<'tcx, 'a> {
    pub analyzer: &'a Analyzer<'tcx>,

    pub root: Node<NodeType>,

    // stack of indices representing path from root to current node
    path: Vec<usize>,
}

impl<'tcx, 'a> FnTree<'tcx, 'a> {
    pub fn new(analyzer: &'a Analyzer<'tcx>) -> Self {
        Self {
            analyzer,
            root: Node {
                label: NodeType::Block,
                children: Vec::new(),
            },
            path: Vec::new(),
        }
    }

    /// helper to get mutable reference to current parent node (walks path)
    fn current_parent_mut(&mut self) -> &mut Node<NodeType> {
        let mut node: *mut Node<NodeType> = &mut self.root;
        for &idx in &self.path {
            unsafe {
                node = &mut (&mut (*node).children)[idx] as *mut Node<NodeType>;
            }
        }
        unsafe { &mut *node }
    }

    /// push a new node and descend into it
    fn push_node(&mut self, label: NodeType) {
        let parent = self.current_parent_mut();
        parent.children.push(Node {
            label,
            children: Vec::new(),
        });
        let new_idx = parent.children.len() - 1;
        self.path.push(new_idx);
    }

    /// add a leaf node (no descent)
    fn add_leaf(&mut self, label: NodeType) {
        let parent = self.current_parent_mut();
        parent.children.push(Node {
            label,
            children: Vec::new(),
        });
    }

    /// pop current node (ascend)
    fn pop_node(&mut self) {
        self.path.pop();
    }

    fn pat_to_name(&self, pat: &rustc_hir::Pat<'tcx>) -> String {
        match &pat.kind {
            PatKind::Binding(_, _, ident, _) => ident.name.to_string(),
            PatKind::Wild => "_".to_string(),
            _ => format!("{:?}", pat),
        }
    }

    fn qpath_to_name(&self, qpath: &QPath<'tcx>) -> String {
        match qpath {
            QPath::Resolved(_, path) => path
                .segments
                .last()
                .map(|s| s.ident.to_string())
                .unwrap_or(format!("{:?}", path)),
            QPath::TypeRelative(_, segment) => segment.ident.to_string(),
            QPath::LangItem(_, _) => format!("{:?}", qpath),
        }
    }

    fn binop_to_string(&self, op: &rustc_hir::BinOp) -> String {
        match op.node {
            BinOpKind::Add => "+".to_string(),
            BinOpKind::Sub => "-".to_string(),
            BinOpKind::Mul => "*".to_string(),
            BinOpKind::Div => "/".to_string(),
            BinOpKind::Rem => "%".to_string(),
            BinOpKind::BitXor => "^".to_string(),
            BinOpKind::BitAnd => "&".to_string(),
            BinOpKind::BitOr => "|".to_string(),
            BinOpKind::Shl => "<<".to_string(),
            BinOpKind::Shr => ">>".to_string(),
            BinOpKind::Eq => "==".to_string(),
            BinOpKind::Lt => "<".to_string(),
            BinOpKind::Le => "<=".to_string(),
            BinOpKind::Ne => "!=".to_string(),
            BinOpKind::Ge => ">=".to_string(),
            BinOpKind::Gt => ">".to_string(),
            BinOpKind::And => "&&".to_string(),
            BinOpKind::Or => "||".to_string(),
            _ => format!("{:?}", op.node),
        }
    }

    fn lit_to_string(&self, lit: LitKind) -> String {
        match lit {
            LitKind::Str(s, _) => format!("\"{}\"", s),
            LitKind::Int(i, _) => format!("{}", i),
            LitKind::Bool(b) => format!("{}", b),
            LitKind::Char(c) => format!("'{}'", c),
            _ => format!("{:?}", lit),
        }
    }

    fn expr_to_string(&self, expr: &Expr<'tcx>) -> String {
        match &expr.kind {
            ExprKind::Lit(lit) => self.lit_to_string(lit.node),
            ExprKind::Path(qpath) => self.qpath_to_name(qpath),
            ExprKind::Field(base, ident) => format!("{}.{}", self.expr_to_string(base), ident),
            _ => format!("{:?}", expr.kind),
        }
    }

    /// entry point to (re)build the tree from a Body
    pub fn build_from_body(&mut self, body: &'tcx Body<'tcx>) {
        self.root = Node {
            label: NodeType::Block,
            children: Vec::new(),
        };
        self.path.clear();

        self.visit_body_expr(body);
    }

    fn visit_body_expr(&mut self, body: &'tcx Body<'tcx>) {
        self.visit_expr(&body.value);
    }

    fn visit_block_expr(&mut self, block: &'tcx Block<'tcx>) {
        for stmt in block.stmts {
            self.visit_stmt(stmt);
        }
        if let Some(expr) = &block.expr {
            self.push_node(NodeType::Jump("return".to_string()));
            self.visit_expr(expr);
            self.pop_node();
        }
    }

    fn visit_stmt(&mut self, stmt: &'tcx Stmt<'tcx>) {
        match &stmt.kind {
            StmtKind::Let(local) => {
                let name = self.pat_to_name(local.pat);
                self.push_node(NodeType::Let(name));
                if let Some(init) = local.init {
                    self.visit_expr(init);
                }
                self.pop_node();
            }
            StmtKind::Expr(expr) | StmtKind::Semi(expr) => {
                self.visit_expr(expr);
            }
            StmtKind::Item(_) => {
                self.add_leaf(NodeType::Unknown);
            }
            StmtKind::Semi(expr) => {
                self.visit_expr(expr);
            }
        }
    }

    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        match &expr.kind {
            ExprKind::Binary(op, left, right) => {
                self.push_node(NodeType::Op(self.binop_to_string(op)));
                self.visit_expr(left);
                self.visit_expr(right);
                self.pop_node();
            }
            ExprKind::Lit(lit) => {
                let s = self.lit_to_string(lit.node);
                self.add_leaf(NodeType::Lit(s));
            }
            ExprKind::Path(qpath) => {
                let name = self.qpath_to_name(qpath);
                self.add_leaf(NodeType::Var(name));
            }
            ExprKind::Call(callee, args) => {
                let name = self.expr_to_string(callee);
                self.push_node(NodeType::FnCall(name));
                for arg in *args {
                    self.visit_expr(arg);
                }
                self.pop_node();
            }
            ExprKind::MethodCall(segment, target, args, _) => {
                let name = segment.ident.to_string();
                self.push_node(NodeType::MethodCall(name));
                self.visit_expr(target);
                for arg in *args {
                    self.visit_expr(arg);
                }
                self.pop_node();
            }
            ExprKind::Block(block, _) => {
                self.push_node(NodeType::Block);
                self.visit_block_expr(block);
                self.pop_node();
            }
            ExprKind::If(cond, then_block, else_opt) => {
                self.push_node(NodeType::Action);

                self.push_node(NodeType::If);
                self.visit_expr(then_block);
                if let Some(else_expr) = else_opt {
                    self.visit_expr(else_expr);
                }
                self.pop_node();

                self.visit_expr(cond);

                self.pop_node();
            }
            ExprKind::Ret(opt) => {
                self.push_node(NodeType::Jump("return".to_string()));
                if let Some(ret_expr) = opt {
                    self.visit_expr(ret_expr);
                }
                self.pop_node();
            }
            ExprKind::Assign(lhs, rhs, _) => {
                let name = self.expr_to_string(lhs);
                self.push_node(NodeType::Assign(name));
                self.visit_expr(rhs);
                self.pop_node();
            }
            ExprKind::Match(scrutinee, arms, _) => {
                self.push_node(NodeType::Match);
                self.visit_expr(scrutinee);
                for arm in *arms {
                    let pat_text = self.pat_to_name(&arm.pat);
                    self.push_node(NodeType::MatchArm(pat_text));
                    if let Some(guard) = &arm.guard {
                        self.visit_expr(guard);
                    }
                    self.visit_expr(&arm.body);
                    self.pop_node();
                }
                self.pop_node();
            }
            ExprKind::Unary(_, sub) => {
                self.push_node(NodeType::Op("unary".to_string()));
                self.visit_expr(sub);
                self.pop_node();
            }
            ExprKind::AddrOf(_, _, sub) => {
                self.push_node(NodeType::Op("addr_of".to_string()));
                self.visit_expr(sub);
                self.pop_node();
            }
            ExprKind::Cast(sub, _) => {
                self.push_node(NodeType::Op("cast".to_string()));
                self.visit_expr(sub);
                self.pop_node();
            }
            ExprKind::DropTemps(sub) => {
                self.visit_expr(sub);
            }
            ExprKind::Struct(qpath, fields, _) => {
                self.push_node(NodeType::Struct(self.qpath_to_name(qpath)));
                for field in *fields {
                    self.push_node(NodeType::Var(field.ident.to_string()));
                    self.visit_expr(&field.expr);
                }
                self.pop_node();
            }
            other => {
                self.add_leaf(NodeType::Unknown);
            }
        }
    }
}
