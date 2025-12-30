use sql2doc::core::ast::*;
use std::str::FromStr;

#[test]
fn test_ddl_statement_creation() {
    let column = ColumnDefinition::new("id".to_string(), DataType::Integer { size: Some(64), unsigned: false })
        .with_constraints(vec![ColumnConstraint::PrimaryKey, ColumnConstraint::AutoIncrement]);

    let create_table = CreateTableStatement::new("users".to_string())
        .with_columns(vec![column]);

    let ddl_stmt = DdlStatement::CreateTable(create_table);

    match ddl_stmt {
        DdlStatement::CreateTable(table) => {
            assert_eq!(table.table_name, "users");
            assert_eq!(table.columns.len(), 1);
            assert!(table.columns[0].is_primary_key());
        }
        _ => panic!("Expected CreateTable statement"),
    }
}

#[test]
fn test_dml_statement_creation() {
    let select = SelectStatement::new()
        .with_select_list(vec![SelectItem::Expression {
            expr: Expression::column("name"),
            alias: None,
        }])
        .with_from(vec![TableReference::table("users")]);

    let dml_stmt = DmlStatement::Select(Box::new(select));

    match dml_stmt {
        DmlStatement::Select(stmt) => {
            assert_eq!(stmt.select_list.len(), 1);
            assert_eq!(stmt.from.len(), 1);
        }
        _ => panic!("Expected Select statement"),
    }
}

#[test]
fn test_types_integration() {
    let identifier = Identifier::new("users");
    assert_eq!(identifier.to_string(), "users");

    let qualified_name = QualifiedName::from_str("public.users").unwrap();
    assert_eq!(qualified_name.len(), 2);
    assert_eq!(qualified_name.to_string(), "public.users");

    let table_meta = TableMetadata::new("users")
        .with_columns(vec![
            ColumnMetadata::new("id", "BIGINT").not_null(),
            ColumnMetadata::new("name", "VARCHAR(255)"),
        ]);

    assert_eq!(table_meta.name, "users");
    assert_eq!(table_meta.columns.len(), 2);
    assert!(!table_meta.columns[0].nullable);
    assert!(table_meta.columns[1].nullable);
}

#[test]
fn test_statement_enum() {
    let column = ColumnDefinition::new("id".to_string(), DataType::BigInt { unsigned: false });
    let create_table = CreateTableStatement::new("test".to_string())
        .with_columns(vec![column]);
    let ddl_stmt = DdlStatement::CreateTable(create_table);

    let statement = sql2doc::core::ast::Statement::Ddl(ddl_stmt);

    match statement {
        sql2doc::core::ast::Statement::Ddl(sql2doc::core::ast::DdlStatement::CreateTable(table)) => {
            assert_eq!(table.table_name, "test");
        }
        _ => panic!("Expected DDL CreateTable statement"),
    }
}
