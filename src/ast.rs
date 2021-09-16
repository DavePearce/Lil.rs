/// Represents a top-level declaration, such as a type alias or a
/// method declaration.
pub enum Decl {
    Alias(String,Type),
    Method(String,Type,Vec<Type>,Stmt)
}

/// Represents a statement in the source code of a Lil program. Many
/// standard statement kinds are provided, including `if`, `while`,
/// `for`, etc.
pub enum Stmt {
    Assert(Expr),
    Block(Vec<Stmt>),
    If(Expr,Box<Stmt>,Option<Box<Stmt>>)
}


/// Represents an expression in the source code of a Lil program. Many
/// standard expression kinds are provided, including unary operations
/// (e.g.  `!e`, `-e`, `|e|`), binary operations (e.g.  `x==y`,
/// `x!=y`, `x+y`, etc), list expressions (e.g. `ls[i]`, `[1,2,3]`,
/// etc), record expressions (e.g. `r.f`, `{x: 1, y: 2}`, etc).
pub enum Expr {
    Variable(String),
    BoolLiteral(bool),
    IntLiteral(i32),
    IntAddition(Box<Expr>,Box<Expr>),
}

/// Represents a type descriptor in the source code of a Lil program.
/// For example, `int` or `int|null` or `{int f}`, etc.
pub enum Type {
    Null,
    Bool,
    Int,
    Ref(Box<Type>),
    Array(Box<Type>)
}
