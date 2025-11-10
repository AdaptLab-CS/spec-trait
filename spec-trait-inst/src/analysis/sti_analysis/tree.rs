use super::distance::Node;
use super::Analyzer;
use rustc_ast::LitKind;
use rustc_hir::{
    AssignOp, AssignOpKind, BinOp, BinOpKind, Block, BodyId, ConstArg, ConstArgKind, Expr,
    ExprKind, ImplItem, ImplItemKind, Item, ItemKind, Mutability, Pat, PatExpr, PatExprKind,
    PatKind, Path, QPath, RangeEnd, Stmt, StmtKind, TraitFn, TraitItem, TraitItemKind, Ty, TyKind,
    UnOp,
};

#[derive(Eq, PartialEq, Debug)]
pub enum VarKind {
    Static,
    Const,
    Local,
    Field,
    LetExpr,
}

#[derive(Eq, PartialEq, Debug)]
pub enum JumpKind {
    Break,
    Continue,
    Return,
    Yield,
}

#[derive(Eq, PartialEq, Debug)]
pub enum CollectionKind {
    Array,
    Tuple,
}

/// Different types of nodes in the body of a function.
#[derive(Eq, PartialEq, Debug)]
pub enum NodeKind {
    /// No children
    Use(String), // use path

    /// No children
    Lit(String), // literal value

    /// Single child (expression)
    VarDef(VarKind, String), // variable / field name

    /// Single child (optional target)
    VarUse(String), // variable / field name

    /// Single child (expression)
    Assign(String), // variable name

    /// Multiple children (operands)
    Op(String), // operator symbol

    /// Multiple children (expressions)
    Block,

    /// Multiple children (items)
    Mod(String), // module name

    /// Single child (body)
    Loop,

    /// Three children (condition, then block, else block)
    If,

    /// Two children (target, match arms block)
    Match,

    /// Two children (guard, body)
    MatchArm(String), // pattern

    /// Single child (expression)
    Jump(JumpKind),

    /// Single child (body)
    FnDef(
        Option<String>, // function name (optional for closures)
        usize,          // number of parameters
    ),

    /// Two children (optional target, arguments tuple)
    FnCall(String), // function / method name

    /// Multiple children (variants)
    Enum(String), // enum name

    /// Multiple children (fields)
    Struct(String), // struct name

    /// Multiple children (elements)
    Collection(CollectionKind),

    // Multiple children (items)
    Impl(
        String,         // type name
        Option<String>, // trait name
    ),

    // Multiple children (items)
    Trait(String), // trait name

    /// Unhandled node
    Unknown,
}

pub struct BodyTree<'tcx, 'a> {
    pub analyzer: &'a Analyzer<'tcx>,

    /// root node of the tree
    pub root: Node<NodeKind>,

    /// stack of indices representing path from root to current node
    /// used to navigate the tree during construction
    path: Vec<usize>,
}

impl<'tcx, 'a> BodyTree<'tcx, 'a> {
    pub fn new(analyzer: &'a Analyzer<'tcx>, id: &BodyId) -> Self {
        let mut tree = Self {
            analyzer,
            root: Node {
                label: NodeKind::Block,
                children: Vec::new(),
            },
            path: Vec::new(),
        };

        tree.visit_body_expr(id);

        if tree.root.children.len() == 1 && tree.root.children[0].label == NodeKind::Block {
            tree.root = tree.root.children.remove(0);
        }

        tree
    }

    /// helper to get mutable reference to current parent node (walks path)
    fn current_parent_mut(&mut self) -> &mut Node<NodeKind> {
        let mut node: *mut Node<NodeKind> = &mut self.root;
        for &idx in &self.path {
            unsafe {
                node = &mut (&mut (*node).children)[idx] as *mut Node<NodeKind>;
            }
        }
        unsafe { &mut *node }
    }

    /// push a new node and descend into it
    fn push_node(&mut self, label: NodeKind) {
        let parent = self.current_parent_mut();
        parent.children.push(Node {
            label,
            children: Vec::new(),
        });
        let new_idx = parent.children.len() - 1;
        self.path.push(new_idx);
    }

    /// add a leaf node (no descent)
    fn add_leaf(&mut self, label: NodeKind) {
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

    fn assign_op_to_binary_op(&self, op: &AssignOp) -> BinOp {
        let node = match op.node {
            AssignOpKind::AddAssign => BinOpKind::Add,
            AssignOpKind::SubAssign => BinOpKind::Sub,
            AssignOpKind::MulAssign => BinOpKind::Mul,
            AssignOpKind::DivAssign => BinOpKind::Div,
            AssignOpKind::RemAssign => BinOpKind::Rem,
            AssignOpKind::BitXorAssign => BinOpKind::BitXor,
            AssignOpKind::BitAndAssign => BinOpKind::BitAnd,
            AssignOpKind::BitOrAssign => BinOpKind::BitOr,
            AssignOpKind::ShlAssign => BinOpKind::Shl,
            AssignOpKind::ShrAssign => BinOpKind::Shr,
        };
        BinOp {
            span: op.span,
            node,
        }
    }

    fn pat_to_string(&self, pat: &Pat<'tcx>) -> String {
        match &pat.kind {
            PatKind::Missing => "...".to_string(),
            PatKind::Binding(_, _, ident, sub) => {
                if let Some(sub) = sub {
                    let sub_name = self.pat_to_string(sub);
                    return format!("{} @ {}", ident, sub_name);
                }
                ident.to_string()
            }
            PatKind::Wild => "_".to_string(),
            PatKind::Struct(qpath, fields, _) => {
                let struct_name = self.qpath_to_string(qpath);
                let field_names = fields
                    .iter()
                    .map(|f| self.pat_to_string(f.pat))
                    .collect::<Vec<_>>();
                format!("{} {{ {} }}", struct_name, field_names.join(", "))
            }
            PatKind::TupleStruct(qpath, fields, _) => {
                let struct_name = self.qpath_to_string(qpath);
                let field_names = fields
                    .iter()
                    .map(|f| self.pat_to_string(f))
                    .collect::<Vec<_>>();
                format!("{}({})", struct_name, field_names.join(", "))
            }
            PatKind::Or(pats) => {
                let pat_names = pats
                    .iter()
                    .map(|p| self.pat_to_string(p))
                    .collect::<Vec<_>>();
                pat_names.join(" | ")
            }
            PatKind::Never => "!".to_string(),
            PatKind::Tuple(fields, _) => {
                let field_names = fields
                    .iter()
                    .map(|f| self.pat_to_string(f))
                    .collect::<Vec<_>>();
                format!("({})", field_names.join(", "))
            }
            PatKind::Box(sub) => {
                let sub_name = self.pat_to_string(sub);
                format!("Box<{}>", sub_name)
            }
            PatKind::Deref(sub) => {
                let sub_name = self.pat_to_string(sub);
                format!("*{}", sub_name)
            }
            PatKind::Ref(sub, _) => {
                let sub_name = self.pat_to_string(sub);
                format!("&{}", sub_name)
            }
            PatKind::Expr(expr) => self.pat_expr_to_string(expr),
            PatKind::Guard(pat, expr) => {
                let pat_name = self.pat_to_string(pat);
                let expr_name = self.expr_to_string(expr);
                format!("{} if {}", pat_name, expr_name)
            }
            PatKind::Range(start, end, end_included) => {
                let start_name = start
                    .map(|s| self.pat_expr_to_string(s))
                    .unwrap_or("_".to_string());
                let end_name = end
                    .map(|e| self.pat_expr_to_string(e))
                    .unwrap_or("_".to_string());
                let sep = if *end_included == RangeEnd::Included {
                    "..="
                } else {
                    ".."
                };
                format!("{} {} {}", start_name, sep, end_name)
            }
            PatKind::Slice(start, mid, end) => {
                let mut parts = Vec::new();
                for s in *start {
                    parts.push(self.pat_to_string(s));
                }
                if let Some(m) = mid {
                    parts.push(format!("..{}", self.pat_to_string(m)));
                }
                for e in *end {
                    parts.push(self.pat_to_string(e));
                }
                format!("[{}]", parts.join(", "))
            }
            _ => format!("{:?}", pat),
        }
    }

    fn pat_expr_to_string(&self, expr: &PatExpr<'tcx>) -> String {
        match &expr.kind {
            PatExprKind::Lit { lit, negated: _ } => self.lit_to_string(lit.node),
            PatExprKind::Path(qpath) => self.qpath_to_string(qpath),
            _ => format!("{:?}", expr.kind),
        }
    }

    fn qpath_to_string(&self, qpath: &QPath<'tcx>) -> String {
        match qpath {
            QPath::Resolved(_, path) => self.path_to_string(path),
            QPath::TypeRelative(ty, segment) => {
                format!("{}::{}", self.type_to_string(ty), segment.ident)
            }
        }
    }

    fn type_to_string(&self, ty: &Ty<'tcx>) -> String {
        match ty.kind {
            TyKind::Path(qpath) => self.qpath_to_string(&qpath),
            _ => format!("{:?}", ty.kind),
        }
    }

    fn bin_op_to_string(&self, op: &BinOp) -> String {
        op.node.as_str().to_string()
    }

    fn un_op_to_string(&self, op: &UnOp) -> String {
        op.as_str().to_string()
    }

    fn lit_to_string(&self, lit: LitKind) -> String {
        match lit {
            LitKind::Str(s, _) => format!("\"{}\"", s),
            LitKind::Int(i, _) => format!("{}", i),
            LitKind::Bool(b) => format!("{}", b),
            LitKind::Char(c) => format!("'{}'", c),
            LitKind::Float(f, _) => format!("{}", f),
            LitKind::ByteStr(bs, _) => format!("b\"{:?}\"", bs),
            LitKind::CStr(bs, _) => format!("c\"{:?}\"", bs),
            LitKind::Byte(b) => format!("b'{}'", b),
            _ => format!("{:?}", lit),
        }
    }

    fn expr_to_string(&self, expr: &Expr<'tcx>) -> String {
        match &expr.kind {
            ExprKind::Lit(lit) => self.lit_to_string(lit.node),
            ExprKind::Path(qpath) => self.qpath_to_string(qpath),
            ExprKind::Field(base, ident) => format!("{}.{}", self.expr_to_string(base), ident),
            _ => format!("{:?}", expr.kind),
        }
    }

    fn path_to_string<T>(&self, path: &Path<'tcx, T>) -> String {
        path.segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<_>>()
            .join("::")
    }

    fn visit_body_expr(&mut self, id: &BodyId) {
        let body = self.analyzer.tcx.hir_body(*id);
        self.visit_expr(body.value);
    }

    fn visit_block_expr(&mut self, block: &'tcx Block<'tcx>) {
        for stmt in block.stmts {
            self.visit_stmt(stmt);
        }
        if let Some(expr) = &block.expr {
            self.push_node(NodeKind::Jump(JumpKind::Return));
            self.visit_expr(expr);
            self.pop_node();
        }
    }

    fn visit_stmt(&mut self, stmt: &'tcx Stmt<'tcx>) {
        match &stmt.kind {
            StmtKind::Let(local) => {
                let name = self.pat_to_string(local.pat);
                self.push_node(NodeKind::VarDef(VarKind::Local, name));
                if let Some(init) = local.init {
                    self.visit_expr(init);
                }
                self.pop_node();
            }
            StmtKind::Expr(expr) | StmtKind::Semi(expr) => {
                self.visit_expr(expr);
            }
            StmtKind::Item(id) => {
                let item = self.analyzer.tcx.hir_item(*id);
                self.visit_item(item);
            }
        }
    }

    fn visit_const_arg(&mut self, c: &'tcx ConstArg<'tcx>) {
        match &c.kind {
            ConstArgKind::Path(qpath) => {
                let name = self.qpath_to_string(qpath);
                self.add_leaf(NodeKind::VarUse(name));
            }
            ConstArgKind::Anon(anon_const) => {
                self.push_node(NodeKind::Block);
                self.visit_body_expr(&anon_const.body);
                self.pop_node();
            }
            ConstArgKind::Infer(_, _) => {
                log::debug!("Unhandled const arg: {:?}", c);
                self.add_leaf(NodeKind::Unknown);
            }
        }
    }

    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        match &expr.kind {
            ExprKind::Binary(op, left, right) => {
                self.push_node(NodeKind::Op(self.bin_op_to_string(op)));
                self.visit_expr(left);
                self.visit_expr(right);
                self.pop_node();
            }
            ExprKind::Unary(op, sub) => {
                self.push_node(NodeKind::Op(self.un_op_to_string(op)));
                self.visit_expr(sub);
                self.pop_node();
            }
            ExprKind::Lit(lit) => {
                let s = self.lit_to_string(lit.node);
                self.add_leaf(NodeKind::Lit(s));
            }
            ExprKind::Path(qpath) => {
                let name = self.qpath_to_string(qpath);
                self.add_leaf(NodeKind::VarUse(name));
            }
            ExprKind::Call(callee, args) => {
                let name = self.expr_to_string(callee);
                self.push_node(NodeKind::FnCall(name));

                self.push_node(NodeKind::Collection(CollectionKind::Tuple));
                for arg in *args {
                    self.visit_expr(arg);
                }
                self.pop_node();

                self.pop_node();
            }
            ExprKind::MethodCall(segment, target, args, _) => {
                let name = segment.ident.to_string();
                self.push_node(NodeKind::FnCall(name));

                self.visit_expr(target);

                self.push_node(NodeKind::Collection(CollectionKind::Tuple));
                for arg in *args {
                    self.visit_expr(arg);
                }
                self.pop_node();

                self.pop_node();
            }
            ExprKind::Block(block, _) => {
                self.push_node(NodeKind::Block);
                self.visit_block_expr(block);
                self.pop_node();
            }
            ExprKind::If(cond, then_block, else_opt) => {
                self.push_node(NodeKind::If);
                self.visit_expr(cond);

                self.visit_expr(then_block);

                if let Some(else_expr) = else_opt {
                    self.visit_expr(else_expr);
                } else {
                    self.add_leaf(NodeKind::Block);
                }

                self.pop_node();
            }
            ExprKind::Ret(opt) => {
                self.push_node(NodeKind::Jump(JumpKind::Return));
                if let Some(ret_expr) = opt {
                    self.visit_expr(ret_expr);
                }
                self.pop_node();
            }
            ExprKind::Assign(lhs, rhs, _) => {
                let name = self.expr_to_string(lhs);
                self.push_node(NodeKind::Assign(name));
                self.visit_expr(rhs);
                self.pop_node();
            }
            ExprKind::AssignOp(assign_op, lhs, rhs) => {
                let name = self.expr_to_string(lhs);
                let op = self.assign_op_to_binary_op(assign_op);

                self.push_node(NodeKind::Assign(name.to_string()));
                self.push_node(NodeKind::Op(self.bin_op_to_string(&op)));
                self.add_leaf(NodeKind::VarUse(name));
                self.visit_expr(rhs);
                self.pop_node();
                self.pop_node();
            }
            ExprKind::Match(target, arms, _) => {
                self.push_node(NodeKind::Match);
                self.visit_expr(target);

                self.push_node(NodeKind::Block);
                for arm in *arms {
                    let pat_text = self.pat_to_string(arm.pat);
                    self.push_node(NodeKind::MatchArm(pat_text));

                    if let Some(guard) = arm.guard {
                        self.visit_expr(guard);
                    } else {
                        self.add_leaf(NodeKind::Lit("true".to_string()));
                    }

                    self.visit_expr(arm.body);

                    self.pop_node();
                }
                self.pop_node();

                self.pop_node();
            }
            ExprKind::AddrOf(_, m, sub) => {
                let str = match m {
                    Mutability::Not => "&",
                    Mutability::Mut => "&mut",
                };
                self.push_node(NodeKind::Op(str.to_string()));
                self.visit_expr(sub);
                self.pop_node();
            }
            ExprKind::DropTemps(sub) => {
                self.visit_expr(sub);
            }
            ExprKind::Struct(qpath, fields, _) => {
                self.push_node(NodeKind::Struct(self.qpath_to_string(qpath)));
                for field in *fields {
                    self.push_node(NodeKind::VarDef(VarKind::Field, field.ident.to_string()));
                    self.visit_expr(field.expr);
                    self.pop_node();
                }
                self.pop_node();
            }
            ExprKind::Loop(body, _, _, _) => {
                self.push_node(NodeKind::Loop);

                self.push_node(NodeKind::Block);
                self.visit_block_expr(body);
                self.pop_node();

                self.pop_node();
            }
            ExprKind::Break(_, expr) => {
                self.push_node(NodeKind::Jump(JumpKind::Break));
                if let Some(break_expr) = expr {
                    self.visit_expr(break_expr);
                }
                self.pop_node();
            }
            ExprKind::Yield(expr, _) => {
                self.push_node(NodeKind::Jump(JumpKind::Yield));
                self.visit_expr(expr);
                self.pop_node();
            }
            ExprKind::Continue(_) => {
                self.add_leaf(NodeKind::Jump(JumpKind::Continue));
            }
            ExprKind::Array(elements) => {
                self.push_node(NodeKind::Collection(CollectionKind::Array));
                for elem in *elements {
                    self.visit_expr(elem);
                }
                self.pop_node();
            }
            ExprKind::Tup(elements) => {
                self.push_node(NodeKind::Collection(CollectionKind::Tuple));
                for elem in *elements {
                    self.visit_expr(elem);
                }
                self.pop_node();
            }
            ExprKind::Use(path, _) => {
                let name = self.expr_to_string(path);
                self.add_leaf(NodeKind::Use(name));
            }
            ExprKind::Closure(closure) => {
                let param_count = closure.fn_decl.inputs.len();
                self.push_node(NodeKind::FnDef(None, param_count));

                self.visit_body_expr(&closure.body);

                self.pop_node();
            }
            ExprKind::Field(base, field) => {
                self.push_node(NodeKind::VarUse(field.to_string()));
                self.visit_expr(base);
                self.pop_node();
            }
            ExprKind::Index(base, index, _) => {
                // x[i] ==> x.get(i).unwrap()
                self.push_node(NodeKind::FnCall("unwrap".to_string()));

                self.push_node(NodeKind::FnCall("get".to_string()));

                self.visit_expr(base);

                self.push_node(NodeKind::Collection(CollectionKind::Tuple));
                self.visit_expr(index);
                self.pop_node();

                self.pop_node();

                self.add_leaf(NodeKind::Collection(CollectionKind::Tuple));

                self.pop_node();
            }
            ExprKind::Repeat(element, count) => {
                self.push_node(NodeKind::Collection(CollectionKind::Array));
                self.push_node(NodeKind::Op("repeat".to_string()));
                self.visit_expr(element);
                self.visit_const_arg(count);
                self.pop_node();
                self.pop_node();
            }
            ExprKind::Let(expr) => {
                let name = self.pat_to_string(expr.pat);
                self.push_node(NodeKind::VarDef(VarKind::LetExpr, name));
                self.visit_expr(expr.init);
                self.pop_node();
            }
            ExprKind::Cast(expr, _) => {
                self.visit_expr(expr);
            }
            other => {
                log::debug!("Unhandled expr: {:?}", other);
                self.add_leaf(NodeKind::Unknown);
            }
        }
    }

    fn visit_item(&mut self, item: &'tcx Item<'tcx>) {
        match &item.kind {
            ItemKind::Use(path, _) => {
                let name = self.path_to_string(*path);
                self.add_leaf(NodeKind::Use(name));
            }
            ItemKind::Static(_, ident, _, body_id) => {
                let name = ident.to_string();
                self.push_node(NodeKind::VarDef(VarKind::Static, name));
                self.visit_body_expr(body_id);
                self.pop_node();
            }
            ItemKind::Const(ident, _, _, body_id) => {
                let name = ident.to_string();
                self.push_node(NodeKind::VarDef(VarKind::Const, name));
                self.visit_body_expr(body_id);
                self.pop_node();
            }
            ItemKind::Fn {
                sig,
                ident,
                body: body_id,
                has_body,
                ..
            } => {
                let name = ident.to_string();
                let param_count = sig.decl.inputs.len();
                self.push_node(NodeKind::FnDef(Some(name), param_count));

                if *has_body {
                    self.visit_body_expr(body_id);
                } else {
                    self.add_leaf(NodeKind::Block);
                }

                self.pop_node();
            }
            ItemKind::Enum(ident, _, def) => {
                let name = ident.to_string();

                for variant in def.variants {
                    self.add_leaf(NodeKind::VarDef(VarKind::Field, variant.ident.to_string()));
                }

                self.add_leaf(NodeKind::Enum(name));
            }
            ItemKind::Struct(ident, _, variant_data) | ItemKind::Union(ident, _, variant_data) => {
                let name = ident.to_string();
                self.push_node(NodeKind::Struct(name));

                for field in variant_data.fields() {
                    self.push_node(NodeKind::VarDef(VarKind::Field, field.ident.to_string()));

                    if let Some(init) = field.default {
                        self.visit_body_expr(&init.body);
                    }

                    self.pop_node();
                }

                self.pop_node();
            }
            ItemKind::Mod(ident, body) => {
                let name = ident.to_string();
                self.push_node(NodeKind::Mod(name));
                for item_id in body.item_ids {
                    let item = self.analyzer.tcx.hir_item(*item_id);
                    self.visit_item(item);
                }
                self.pop_node();
            }
            ItemKind::Trait(_, _, _, ident, _, _, items) => {
                let name = ident.to_string();
                self.push_node(NodeKind::Trait(name));

                for item_id in *items {
                    let trait_item = self.analyzer.tcx.hir_trait_item(*item_id);
                    self.visit_trait_item(trait_item);
                }

                self.pop_node();
            }
            ItemKind::Impl(def) => {
                let self_ty = self.type_to_string(def.self_ty);
                let trait_name = def
                    .of_trait
                    .as_ref()
                    .map(|trait_impl| self.path_to_string(trait_impl.trait_ref.path));

                self.push_node(NodeKind::Impl(self_ty, trait_name));

                for item_id in def.items {
                    let impl_item = self.analyzer.tcx.hir_impl_item(*item_id);
                    self.visit_impl_item(impl_item);
                }

                self.pop_node();
            }
            other => {
                log::debug!("Unhandled item: {:?}", other);
                self.add_leaf(NodeKind::Unknown);
            }
        }
    }

    fn visit_impl_item(&mut self, item: &'tcx ImplItem<'tcx>) {
        let name = item.ident.to_string();
        match &item.kind {
            ImplItemKind::Const(_, body) => {
                self.push_node(NodeKind::VarDef(VarKind::Const, name));
                self.visit_body_expr(body);
                self.pop_node();
            }
            ImplItemKind::Fn(sig, body_id) => {
                let param_count = sig.decl.inputs.len();
                self.push_node(NodeKind::FnDef(Some(name), param_count));
                self.visit_body_expr(body_id);
                self.pop_node();
            }
            ImplItemKind::Type(_) => {}
        }
    }

    fn visit_trait_item(&mut self, item: &'tcx TraitItem<'tcx>) {
        let name = item.ident.to_string();
        match &item.kind {
            TraitItemKind::Const(_, body) => {
                self.push_node(NodeKind::VarDef(VarKind::Const, name));
                if let Some(body) = body {
                    self.visit_body_expr(body);
                }
                self.pop_node();
            }
            TraitItemKind::Fn(sig, trait_fn) => {
                let param_count = sig.decl.inputs.len();

                self.push_node(NodeKind::FnDef(Some(name), param_count));

                if let TraitFn::Provided(body_id) = trait_fn {
                    self.visit_body_expr(body_id);
                } else {
                    self.add_leaf(NodeKind::Block);
                }

                self.pop_node();
            }
            TraitItemKind::Type(_, _) => {}
        }
    }
}
