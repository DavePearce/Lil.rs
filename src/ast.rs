/// Represents a statement in the source code of a Lil program. Many
/// standard statement kinds are provided, including `if`, `while`,
/// `for`, etc.
pub enum Stmt {
    Assert(Expr)	
}

/// Represents an expression in the source code of a While
/// program. Many standard expression kinds are provided, including
/// unary operations (e.g.  `!e`, `-e`, `|e|`), binary operations
/// (e.g.  `x==y`, `x!=y`, `x+y`, etc), list expressions
/// (e.g. `ls[i]`, `[1,2,3]`, etc), record expressions (e.g. `r.f`,
/// `{x: 1, y: 2}`, etc).
pub enum Expr {
    Variable(String),
    BooleanLiteral(bool),
    IntegerLiteral(i32),
    IntegerAddition(Box<Expr>,Box<Expr>),    
}
