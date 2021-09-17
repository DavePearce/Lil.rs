/// Represents a top-level declaration, such as a type alias or a
/// method declaration.
#[derive(Debug,PartialEq)]
pub enum Decl {
    Error,
    TypeAlias(String,Type),
    Method(String,Type,Vec<Type>,Stmt)
}

/// Represents a statement in the source code of a Lil program. Many
/// standard statement kinds are provided, including `if`, `while`,
/// `for`, etc.
#[derive(Debug,PartialEq)]
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
#[derive(Debug,PartialEq)]
pub enum Expr {
    Variable(String),
    BoolLiteral(bool),
    IntLiteral(i32),
    IntAddition(Box<Expr>,Box<Expr>),
}

/// Represents a type descriptor in the source code of a Lil program.
/// For example, `int` or `int|null` or `{int f}`, etc.
#[derive(Clone,Debug,PartialEq)]
pub enum Type {
    Array(Box<Type>),
    Bool,
    Null,    
    Int8,
    Int16,
    Int32,
    Int64,
    Record(Vec<(Type,String)>),
    Ref(Box<Type>),
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Void
}

/// Constructor for array types
pub fn Array(element : Type) -> Type {
    Type::Array(Box::new(element))
}

/// Constructor for record types
pub fn Record(fields : &[(Type,String)]) -> Type {
    Type::Record(fields.to_vec())
}

/// Constructor for reference types
pub fn Ref(target : Type) -> Type {
    Type::Ref(Box::new(target))
}
