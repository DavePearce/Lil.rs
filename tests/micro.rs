use lil::ast::*;
use lil::parser::Parser;
use lil::typer::TypeChecker;

// ======================================================
// Tests (Type Declarations)
// ======================================================

#[test]
fn test_type_01() {
    check_parse_error("type nat = i32");
}

#[test]
fn test_type_02() {
    check_parse_error("type nat i8;");
}

#[test]
fn test_type_03() {
    let ast = check_parse("type t = bool;");
    assert!(matches!(ast.get(0),Node::BoolType));
}

#[test]
fn test_type_04() {
    let ast = check_parse("type nat = i8;");
    assert_eq!(ast.get(0),&Node::IntType(true,8));
}

#[test]
fn test_type_05() {
    let ast = check_parse("type nat = i16;");
    assert_eq!(ast.get(0),&Node::IntType(true,16));
}

#[test]
fn test_type_06() {
    let ast = check_parse("type nat = i32;");
    assert_eq!(ast.get(0),&Node::IntType(true,32));
}

#[test]
fn test_type_07() {
    let ast = check_parse("type nat = i64;");
    assert_eq!(ast.get(0),&Node::IntType(true,64));
}

#[test]
fn test_type_08() {
    let ast = check_parse("type nat = u8;");
    assert_eq!(ast.get(0),&Node::IntType(false,8));
}

#[test]
fn test_type_09() {
    let ast = check_parse("type nat = u16;");
    assert_eq!(ast.get(0),&Node::IntType(false,16));
}

#[test]
fn test_type_10() {
    let ast = check_parse("type nat = u32;");
    assert_eq!(ast.get(0),&Node::IntType(false,32));
}

#[test]
fn test_type_11() {
    let ast = check_parse("type nat = u64;");
    assert_eq!(ast.get(0),&Node::IntType(false,64));
}

#[test]
fn test_type_12() {
    let ast = check_parse("type nat = i32[];");
    assert_eq!(ast.get(0),&Node::IntType(true,32));
    assert_eq!(ast.get(1),&Node::ArrayType(Type{index:0}));
}

#[test]
fn test_type_13() {
    let ast = check_parse("type nat = i32[][];");
    assert_eq!(ast.get(0),&Node::IntType(true,32));
    assert_eq!(ast.get(1),&Node::ArrayType(Type{index:0}));
    assert_eq!(ast.get(2),&Node::ArrayType(Type{index:1}));
}

#[test]
fn test_type_14() {
    let ast = check_parse("type ref = &i16;");
    assert_eq!(ast.get(0),&Node::IntType(true,16));
    assert_eq!(ast.get(1),&Node::ReferenceType(Type{index:0}));
}

#[test]
fn test_type_15() {
    let ast = check_parse("type ref = &&i16;");
    assert_eq!(ast.get(0),&Node::IntType(true,16));
    assert_eq!(ast.get(1),&Node::ReferenceType(Type{index:0}));
    assert_eq!(ast.get(2),&Node::ReferenceType(Type{index:1}));
}

#[test]
fn test_type_16() {
    let f = "f".to_string();
    //
    let ast = check_parse("type rec = {i64 f};");
    assert_eq!(ast.get(0),&Node::IntType(true,64));
    assert_eq!(ast.get(1),&Node::RecordType(vec![(Type{index:0},f)]));
}

#[test]
fn test_type_17() {
    let f = "f".to_string();
    let g = "g".to_string();
    //
    let ast = check_parse("type rec = {i32 f, u16 g};");
    assert_eq!(ast.get(0),&Node::IntType(true,32));
    assert_eq!(ast.get(1),&Node::IntType(false,16));
    assert_eq!(ast.get(2),&Node::RecordType(vec![(Type{index:0},f),(Type{index:1},g)]));
}

#[test]
fn test_type_18() {
    let ast = check_parse("type rar = (&u32)[];");
    assert_eq!(ast.get(0),&Node::IntType(false,32));
    assert_eq!(ast.get(1),&Node::ReferenceType(Type{index:0}));
    assert_eq!(ast.get(2),&Node::ArrayType(Type{index:1}));
}

#[test]
fn test_type_19() {
    let f = "f".to_string();
    let g = "g".to_string();
    //
    let ast = check_parse("type rec = {&i8 f, u16[] g};");
    assert_eq!(ast.get(0),&Node::IntType(true,8));
    assert_eq!(ast.get(1),&Node::ReferenceType(Type{index:0}));
    assert_eq!(ast.get(2),&Node::IntType(false,16));
    assert_eq!(ast.get(3),&Node::ArrayType(Type{index:2}));
    assert_eq!(ast.get(4),&Node::RecordType(vec![(Type{index:1},f),(Type{index:3},g)]));
}

// ======================================================
// Tests (Method Declarations)
// ======================================================

#[test]
fn test_method_01() {
    check_parse_error("voi");
}

#[test]
fn test_method_02() {
    check_parse_error("void");
}

#[test]
fn test_method_03() {
    check_parse_error("void f");
}

#[test]
fn test_method_04() {
    check_parse_error("void f(");
}

#[test]
fn test_method_05() {
    check_parse_error("void f() {");
}

#[test]
fn test_method_06() {
    let f = "f".to_string();    
    let ast = check_parse("void f() {}");
    assert_eq!(ast.get(0),&Node::VoidType);
    assert_eq!(ast.get(1),&Node::BlockStmt(vec![]));
    assert_eq!(ast.get(2),&Node::MethodDecl(f,Type{index:0},vec![],Stmt{index:1}));
}

#[test]
fn test_method_07() {
    let f = "f".to_string();
    let x = "x".to_string();    
    let ast = check_parse("void f(i32 x) {}");
    assert_eq!(ast.get(0),&Node::VoidType);
    assert_eq!(ast.get(1),&Node::IntType(true,32));    
    assert_eq!(ast.get(2),&Node::BlockStmt(vec![]));
    let params = vec![Parameter{declared:Type{index:1},name:x}];
    assert_eq!(ast.get(3),&Node::MethodDecl(f,Type{index:0},params,Stmt{index:2}));
}

#[test]
fn test_method_08() {
    let f = "f".to_string();
    let i = "i".to_string();
    let b = "b".to_string(); 
    let ast = check_parse("bool f(i32 i, bool b) {}");
    assert_eq!(ast.get(0),&Node::BoolType);
    assert_eq!(ast.get(1),&Node::IntType(true,32));
    assert_eq!(ast.get(2),&Node::BoolType);    
    assert_eq!(ast.get(3),&Node::BlockStmt(vec![]));
    let params = vec![Parameter{declared:Type{index:1},name:i},Parameter{declared:Type{index:2},name:b}];
    assert_eq!(ast.get(4),&Node::MethodDecl(f,Type{index:0},params,Stmt{index:3}));
}

// ======================================================
// Tests (Skip)
// ======================================================

#[test]
fn test_skip_01() {
    let ast = check_parse_error("void f() { ski }");
}

#[test]
fn test_skip_02() {
    let ast = check_parse_error("void f() { skip }");    
}

#[test]
fn test_skip_03() {
    let ast = check_parse("void f() { skip; }");
    assert_eq!(ast.get(0),&Node::VoidType);    
    assert_eq!(ast.get(1),&Node::SkipStmt);    
}

// ======================================================
// Tests (Assert)
// ======================================================

#[test]
fn test_assert_04() {
    let ast = check_parse_error("void f() { asse");
}

#[test]
fn test_assert_05() {
    let ast = check_parse_error("void f() { assert");
}

#[test]
fn test_assert_06() {
    let ast = check_parse_error("void f() { assert true }");
}

#[test]
fn test_assert_07() {
    let ast = check_parse("void f() { assert true; }");
    assert_eq!(ast.get(1),&Node::BoolExpr(true));        
    assert_eq!(ast.get(2),&Node::AssertStmt(Expr{index:1}));    
}

#[test]
fn test_assert_08() {
    let ast = check_parse("void f() { assert false; }");
    assert_eq!(ast.get(1),&Node::BoolExpr(false));        
    assert_eq!(ast.get(2),&Node::AssertStmt(Expr{index:1}));    
}

#[test]
fn test_assert_09() {
    let ast = check_parse("void f() { assert (false); }");
    assert_eq!(ast.get(1),&Node::BoolExpr(false));        
    assert_eq!(ast.get(2),&Node::AssertStmt(Expr{index:1}));    
}

#[test]
fn test_assert_10() {
    let b = "b".to_string();    
    let ast = check_type_error("void f() { assert b; }");
}

#[test]
fn test_assert_11() {
    let b = "b".to_string();    
    let ast = check_parse("void f(bool b) { assert b; }");
    assert_eq!(ast.get(1),&Node::BoolType);    
    assert_eq!(ast.get(2),&Node::VarExpr(b.to_string()));        
    assert_eq!(ast.get(3),&Node::AssertStmt(Expr{index:2}));    
}

#[test]
fn test_assert_12() {
    let i = "i".to_string();    
    let ast = check_parse("void f(i32 i) { assert i < 0; }");
    assert_eq!(ast.get(1),&Node::IntType(true,32));    
    assert_eq!(ast.get(2),&Node::VarExpr(i.to_string()));
    assert_eq!(ast.get(3),&Node::IntExpr(0));
    //assert_eq!(ast.get(3),&Node::IntExpr(0));
    //assert_eq!(ast.get(3),&Node::AssertStmt(Expr{index:2}));    
}

// ======================================================
// Tests (Statements)
// ======================================================

/// A dummy source mapper which does nothing.
fn source_mapper<'a>(_: usize, _: &'a str) { }

/// A dummy type mapper which does nothing.
fn type_mapper<'a>(_: usize, _: Type) { }

#[cfg(test)]
fn check_parse(input: &str) -> Box<AbstractSyntaxTree> {
    let mut ast = AbstractSyntaxTree::new();
    let mut parser = Parser::new(input,&mut ast, source_mapper);
    // Parse input
    let d = parser.parse_decl();
    println!("PARSED {:?}",d);
    assert!(!d.is_err());
    // Type input
    let mut typer = TypeChecker::new(&mut ast, type_mapper);
    let r = typer.check(d.unwrap());
    assert!(!r.is_err());
    // Done
    Box::new(ast)
}

#[cfg(test)]
fn check_parse_error(input: &str) {
    let mut ast = AbstractSyntaxTree::new();
    let mut p = Parser::new(input,&mut ast, source_mapper);
    let d = p.parse_decl();
    assert!(d.is_err());
}

#[cfg(test)]
fn check_type_error(input: &str) {
    let mut ast = AbstractSyntaxTree::new();
    let mut parser = Parser::new(input,&mut ast, source_mapper);
    // Parse input
    let d = parser.parse_decl();
    println!("PARSED {:?}",d);
    assert!(!d.is_err());
    // Type input
    let mut typer = TypeChecker::new(&mut ast, type_mapper);
    let r = typer.check(d.unwrap());
    assert!(r.is_err());    
}
