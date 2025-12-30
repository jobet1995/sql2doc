use std::collections::HashMap;
use std::collections::HashSet;

use crate::core::ast::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SqlDialect {
    PostgreSQL,
    MySQL,
    SQLite,
    MSSQL,
    Oracle,
    Standard,
}

#[derive(Debug, Clone)]
pub struct DialectConfig {
    pub name: String,
    pub keywords: HashSet<String>,
    pub data_types: HashMap<String, DataType>,
    pub functions: HashSet<String>,
    pub operators: HashMap<String, BinaryOperator>,
    pub quote_chars: Vec<char>,
    pub identifier_quote: char,
    pub string_quote: char,
    pub supports_auto_increment: bool,
    pub auto_increment_keyword: String,
    pub supports_identity: bool,
    pub identity_keyword: String,
    pub limit_keyword: String,
    pub offset_keyword: String,
    pub case_sensitive_identifiers: bool,
    pub supports_cte: bool,
    pub supports_window_functions: bool,
    pub supports_recursive_cte: bool,
}

impl SqlDialect {
    pub fn config(&self) -> DialectConfig {
        match self {
            SqlDialect::PostgreSQL => DialectConfig {
                name: "PostgreSQL".to_string(),
                keywords: Self::postgresql_keywords(),
                data_types: Self::postgresql_data_types(),
                functions: Self::postgresql_functions(),
                operators: Self::standard_operators(),
                quote_chars: vec!['"', '`'],
                identifier_quote: '"',
                string_quote: '\'',
                supports_auto_increment: false,
                auto_increment_keyword: "".to_string(),
                supports_identity: true,
                identity_keyword: "GENERATED ALWAYS AS IDENTITY".to_string(),
                limit_keyword: "LIMIT".to_string(),
                offset_keyword: "OFFSET".to_string(),
                case_sensitive_identifiers: true,
                supports_cte: true,
                supports_window_functions: true,
                supports_recursive_cte: true,
            },
            SqlDialect::MySQL => DialectConfig {
                name: "MySQL".to_string(),
                keywords: Self::mysql_keywords(),
                data_types: Self::mysql_data_types(),
                functions: Self::mysql_functions(),
                operators: Self::standard_operators(),
                quote_chars: vec!['`', '"'],
                identifier_quote: '`',
                string_quote: '\'',
                supports_auto_increment: true,
                auto_increment_keyword: "AUTO_INCREMENT".to_string(),
                supports_identity: false,
                identity_keyword: "".to_string(),
                limit_keyword: "LIMIT".to_string(),
                offset_keyword: "OFFSET".to_string(),
                case_sensitive_identifiers: false,
                supports_cte: true,
                supports_window_functions: true,
                supports_recursive_cte: true,
            },
            SqlDialect::SQLite => DialectConfig {
                name: "SQLite".to_string(),
                keywords: Self::sqlite_keywords(),
                data_types: Self::sqlite_data_types(),
                functions: Self::sqlite_functions(),
                operators: Self::standard_operators(),
                quote_chars: vec!['"', '`', '['],
                identifier_quote: '"',
                string_quote: '\'',
                supports_auto_increment: true,
                auto_increment_keyword: "AUTOINCREMENT".to_string(),
                supports_identity: false,
                identity_keyword: "".to_string(),
                limit_keyword: "LIMIT".to_string(),
                offset_keyword: "OFFSET".to_string(),
                case_sensitive_identifiers: false,
                supports_cte: true,
                supports_window_functions: true,
                supports_recursive_cte: true,
            },
            SqlDialect::MSSQL => DialectConfig {
                name: "SQL Server".to_string(),
                keywords: Self::mssql_keywords(),
                data_types: Self::mssql_data_types(),
                functions: Self::mssql_functions(),
                operators: Self::standard_operators(),
                quote_chars: vec!['[', '"'],
                identifier_quote: '[',
                string_quote: '\'',
                supports_auto_increment: true,
                auto_increment_keyword: "IDENTITY(1,1)".to_string(),
                supports_identity: true,
                identity_keyword: "IDENTITY".to_string(),
                limit_keyword: "TOP".to_string(),
                offset_keyword: "OFFSET".to_string(),
                case_sensitive_identifiers: false,
                supports_cte: true,
                supports_window_functions: true,
                supports_recursive_cte: true,
            },
            SqlDialect::Oracle => DialectConfig {
                name: "Oracle".to_string(),
                keywords: Self::oracle_keywords(),
                data_types: Self::oracle_data_types(),
                functions: Self::oracle_functions(),
                operators: Self::standard_operators(),
                quote_chars: vec!['"', '`'],
                identifier_quote: '"',
                string_quote: '\'',
                supports_auto_increment: false,
                auto_increment_keyword: "".to_string(),
                supports_identity: true,
                identity_keyword: "GENERATED ALWAYS AS IDENTITY".to_string(),
                limit_keyword: "ROWNUM".to_string(),
                offset_keyword: "OFFSET".to_string(),
                case_sensitive_identifiers: true,
                supports_cte: true,
                supports_window_functions: true,
                supports_recursive_cte: true,
            },
            SqlDialect::Standard => DialectConfig {
                name: "Standard SQL".to_string(),
                keywords: Self::standard_keywords(),
                data_types: Self::standard_data_types(),
                functions: Self::standard_functions(),
                operators: Self::standard_operators(),
                quote_chars: vec!['"'],
                identifier_quote: '"',
                string_quote: '\'',
                supports_auto_increment: false,
                auto_increment_keyword: "".to_string(),
                supports_identity: false,
                identity_keyword: "".to_string(),
                limit_keyword: "LIMIT".to_string(),
                offset_keyword: "OFFSET".to_string(),
                case_sensitive_identifiers: true,
                supports_cte: true,
                supports_window_functions: true,
                supports_recursive_cte: true,
            },
        }
    }

    fn postgresql_keywords() -> HashSet<String> {
        let mut keywords = Self::standard_keywords();
        let pg_specific = vec![
            "ABORT", "ABSOLUTE", "ACCESS", "ACTION", "ADD", "ADMIN", "AFTER", "AGGREGATE",
            "ALSO", "ALTER", "ALWAYS", "ASSERTION", "ASSIGNMENT", "AT", "AUTHORIZATION",
            "BACKWARD", "BEFORE", "BEGIN", "BY", "CACHE", "CALLED", "CASCADE", "CASCADED",
            "CATALOG", "CHAIN", "CHARACTERISTICS", "CHECKPOINT", "CLASS", "CLOSE", "CLUSTER",
            "COMMENT", "COMMENTS", "COMMIT", "COMMITTED", "CONFIGURATION", "CONNECTION",
            "CONSTRAINTS", "CONTENT", "CONTINUE", "CONVERSION", "COPY", "COST", "CSV",
            "CUBE", "CURRENT", "CURSOR", "CYCLE", "DATA", "DATABASE", "DAY", "DEALLOCATE",
            "DECLARE", "DEFAULTS", "DEFER", "DEFERRED", "DEFINER", "DELETE", "DELIMITER",
            "DELIMITERS", "DICTIONARY", "DISABLE", "DISCARD", "DOCUMENT", "DOMAIN", "DOUBLE",
            "DROP", "EACH", "ENABLE", "ENCODING", "ENCRYPTED", "ENUM", "ESCAPE", "EVENT",
            "EXCLUDE", "EXCLUDING", "EXCLUSIVE", "EXECUTE", "EXPLAIN", "EXTENSION", "EXTERNAL",
            "FAMILY", "FILTER", "FIRST", "FOLLOWING", "FORCE", "FORWARD", "FUNCTION", "FUNCTIONS",
            "GLOBAL", "GRANTED", "HANDLER", "HEADER", "HOLD", "HOUR", "IDENTITY", "IF", "IMMEDIATE",
            "IMMUTABLE", "IMPLICIT", "IMPORT", "INCLUDING", "INCREMENT", "INDEX", "INDEXES",
            "INHERIT", "INHERITS", "INLINE", "INSENSITIVE", "INSERT", "INSTEAD", "INVOKER",
            "ISOLATION", "KEY", "LABEL", "LANGUAGE", "LARGE", "LAST", "LEAKPROOF", "LEVEL",
            "LISTEN", "LOAD", "LOCAL", "LOCATION", "LOCK", "MAPPING", "MATCH", "MATERIALIZED",
            "MAXVALUE", "MINUTE", "MINVALUE", "MODE", "MONTH", "MOVE", "NAME", "NAMES",
            "NEXT", "NO", "NOTHING", "NOTIFY", "NOWAIT", "NULLS", "OBJECT", "OF", "OIDS",
            "OPERATOR", "OPTION", "OPTIONS", "ORDINALITY", "OUT", "OVER", "OWNED", "OWNER",
            "PARSER", "PARTIAL", "PARTITION", "PASSING", "PASSWORD", "PLANS", "POLICY",
            "PRECEDING", "PREPARE", "PREPARED", "PRESERVE", "PRIOR", "PRIVILEGES", "PROCEDURAL",
            "PROCEDURE", "PROGRAM", "QUOTE", "RANGE", "READ", "REASSIGN", "RECHECK", "RECURSIVE",
            "REF", "REFRESH", "REINDEX", "RELATIVE", "RELEASE", "RENAME", "REPEATABLE",
            "REPLACE", "REPLICA", "RESET", "RESTART", "RESTRICT", "RETURNING", "RETURNS",
            "REVOKE", "ROLE", "ROLLBACK", "ROLLUP", "ROW", "RULE", "SAVEPOINT", "SCHEMA",
            "SCROLL", "SEARCH", "SECOND", "SECURITY", "SEQUENCE", "SEQUENCES", "SERIALIZABLE",
            "SERVER", "SESSION", "SET", "SETS", "SHARE", "SHOW", "SIMPLE", "SNAPSHOT", "SQL",
            "STABLE", "STANDALONE", "START", "STATEMENT", "STATISTICS", "STDIN", "STDOUT",
            "STORAGE", "STRICT", "STRIP", "SYSID", "SYSTEM", "TABLES", "TABLESPACE", "TEMP",
            "TEMPLATE", "TEMPORARY", "TEXT", "TRANSACTION", "TRIGGER", "TRIM", "TRUNCATE",
            "TRUSTED", "TYPE", "TYPES", "UNBOUNDED", "UNCOMMITTED", "UNENCRYPTED", "UNKNOWN",
            "UNLISTEN", "UNLOGGED", "UNTIL", "UPDATE", "VACUUM", "VALID", "VALIDATE", "VALIDATOR",
            "VALUE", "VARYING", "VERSION", "VIEW", "VOLATILE", "WHITESPACE", "WITHIN", "WITHOUT",
            "WORK", "WRAPPER", "WRITE", "XML", "XMLATTRIBUTES", "XMLCONCAT", "XMLELEMENT",
            "XMLEXISTS", "XMLFOREST", "XMLPARSE", "XMLPI", "XMLROOT", "XMLSERIALIZE", "YEAR",
            "YES", "ZONE"
        ];

        for keyword in pg_specific {
            keywords.insert(keyword.to_string());
        }
        keywords
    }

    fn mysql_keywords() -> HashSet<String> {
        let mut keywords = Self::standard_keywords();
        let mysql_specific = vec![
            "ACCESSIBLE", "ADD", "ALL", "ALTER", "ANALYZE", "AND", "AS", "ASC", "ASENSITIVE",
            "BEFORE", "BETWEEN", "BIGINT", "BINARY", "BLOB", "BOTH", "BY", "CALL", "CASCADE",
            "CASE", "CHANGE", "CHAR", "CHARACTER", "CHECK", "COLLATE", "COLUMN", "CONDITION",
            "CONSTRAINT", "CONTINUE", "CONVERT", "CREATE", "CROSS", "CURRENT_DATE", "CURRENT_TIME",
            "CURRENT_TIMESTAMP", "CURRENT_USER", "CURSOR", "DATABASE", "DATABASES", "DAY_HOUR",
            "DAY_MICROSECOND", "DAY_MINUTE", "DAY_SECOND", "DEC", "DECIMAL", "DECLARE", "DEFAULT",
            "DELAYED", "DELETE", "DESC", "DESCRIBE", "DETERMINISTIC", "DISTINCT", "DISTINCTROW",
            "DIV", "DOUBLE", "DROP", "DUAL", "EACH", "ELSE", "ELSEIF", "ENCLOSED", "ESCAPED",
            "EXISTS", "EXIT", "EXPLAIN", "FALSE", "FETCH", "FLOAT", "FLOAT4", "FLOAT8", "FOR",
            "FORCE", "FOREIGN", "FROM", "FULLTEXT", "GENERATED", "GET", "GRANT", "GROUP", "HAVING",
            "HIGH_PRIORITY", "HOUR_MICROSECOND", "HOUR_MINUTE", "HOUR_SECOND", "IF", "IGNORE",
            "IN", "INDEX", "INFILE", "INNER", "INOUT", "INSENSITIVE", "INSERT", "INT", "INT1",
            "INT2", "INT3", "INT4", "INT8", "INTEGER", "INTERVAL", "INTO", "IO_AFTER_GTIDS",
            "IO_BEFORE_GTIDS", "IS", "ITERATE", "JOIN", "KEY", "KEYS", "KILL", "LEADING", "LEAVE",
            "LEFT", "LIKE", "LIMIT", "LINEAR", "LINES", "LOAD", "LOCALTIME", "LOCALTIMESTAMP",
            "LOCK", "LONG", "LONGBLOB", "LONGTEXT", "LOOP", "LOW_PRIORITY", "MASTER_BIND",
            "MASTER_SSL_VERIFY_SERVER_CERT", "MATCH", "MAXVALUE", "MEDIUMBLOB", "MEDIUMINT",
            "MEDIUMTEXT", "MIDDLEINT", "MINUTE_MICROSECOND", "MINUTE_MINUTE", "MINUTE_SECOND",
            "MOD", "MODIFIES", "NATURAL", "NOT", "NO_WRITE_TO_BINLOG", "NULL", "NUMERIC",
            "ON", "OPTIMIZE", "OPTIMIZER_COSTS", "OPTION", "OPTIONALLY", "OR", "ORDER", "OUT",
            "OUTER", "OUTFILE", "PARTITION", "PRECISION", "PRIMARY", "PROCEDURE", "PURGE",
            "RANGE", "READ", "READS", "READ_WRITE", "REAL", "REFERENCES", "REGEXP", "RELEASE",
            "RENAME", "REPEAT", "REPLACE", "REQUIRE", "RESIGNAL", "RESTRICT", "RETURN", "REVOKE",
            "RIGHT", "RLIKE", "SCHEMA", "SCHEMAS", "SECOND_MICROSECOND", "SELECT", "SENSITIVE",
            "SEPARATOR", "SET", "SHOW", "SIGNAL", "SMALLINT", "SPATIAL", "SPECIFIC", "SQL",
            "SQLEXCEPTION", "SQLSTATE", "SQLWARNING", "SQL_BIG_RESULT", "SQL_CALC_FOUND_ROWS",
            "SQL_SMALL_RESULT", "SSL", "STARTING", "STORED", "STRAIGHT_JOIN", "TABLE", "TERMINATED",
            "THEN", "TINYBLOB", "TINYINT", "TINYTEXT", "TO", "TRAILING", "TRIGGER", "TRUE",
            "UNDO", "UNION", "UNIQUE", "UNLOCK", "UNSIGNED", "UPDATE", "USAGE", "USE", "USING",
            "UTC_DATE", "UTC_TIME", "UTC_TIMESTAMP", "VALUES", "VARBINARY", "VARCHAR", "VARCHARACTER",
            "VARYING", "VIRTUAL", "WHEN", "WHERE", "WHILE", "WITH", "WRITE", "XOR", "YEAR_MONTH",
            "ZEROFILL"
        ];

        for keyword in mysql_specific {
            keywords.insert(keyword.to_string());
        }
        keywords
    }

    fn sqlite_keywords() -> HashSet<String> {
        let mut keywords = Self::standard_keywords();
        let sqlite_specific = vec![
            "ABORT", "ACTION", "ADD", "AFTER", "ALL", "ALTER", "ANALYZE", "AND", "AS", "ASC",
            "ATTACH", "AUTOINCREMENT", "BEFORE", "BEGIN", "BETWEEN", "BY", "CASCADE", "CASE",
            "CAST", "CHECK", "COLLATE", "COLUMN", "COMMIT", "CONFLICT", "CONSTRAINT", "CREATE",
            "CROSS", "CURRENT_DATE", "CURRENT_TIME", "CURRENT_TIMESTAMP", "DATABASE", "DEFAULT",
            "DEFERRABLE", "DEFERRED", "DELETE", "DESC", "DETACH", "DISTINCT", "DROP", "EACH",
            "ELSE", "END", "ESCAPE", "EXCEPT", "EXCLUSIVE", "EXISTS", "EXPLAIN", "FAIL", "FOR",
            "FOREIGN", "FROM", "FULL", "GLOB", "GROUP", "HAVING", "IF", "IGNORE", "IMMEDIATE",
            "IN", "INDEX", "INDEXED", "INITIALLY", "INNER", "INSERT", "INSTEAD", "INTERSECT",
            "INTO", "IS", "ISNULL", "JOIN", "KEY", "LEFT", "LIKE", "LIMIT", "MATCH", "NATURAL",
            "NO", "NOT", "NOTNULL", "NULL", "OF", "OFFSET", "ON", "OR", "ORDER", "OUTER", "PLAN",
            "PRAGMA", "PRIMARY", "QUERY", "RAISE", "RECURSIVE", "REFERENCES", "REGEXP", "REINDEX",
            "RELEASE", "RENAME", "REPLACE", "RESTRICT", "RIGHT", "ROLLBACK", "ROW", "SAVEPOINT",
            "SELECT", "SET", "TABLE", "TEMP", "TEMPORARY", "THEN", "TO", "TRANSACTION", "TRIGGER",
            "UNION", "UNIQUE", "UPDATE", "USING", "VACUUM", "VALUES", "VIEW", "VIRTUAL", "WHEN",
            "WHERE", "WITH", "WITHOUT"
        ];

        for keyword in sqlite_specific {
            keywords.insert(keyword.to_string());
        }
        keywords
    }

    fn mssql_keywords() -> HashSet<String> {
        let mut keywords = Self::standard_keywords();
        let mssql_specific = vec![
            "ADD", "ALL", "ALTER", "AND", "ANY", "AS", "ASC", "AUTHORIZATION", "BACKUP", "BEGIN",
            "BETWEEN", "BREAK", "BROWSE", "BULK", "BY", "CASCADE", "CASE", "CHECK", "CHECKPOINT",
            "CLOSE", "CLUSTERED", "COALESCE", "COLLATE", "COLUMN", "COMMIT", "COMPUTE", "CONSTRAINT",
            "CONTAINS", "CONTAINSTABLE", "CONTINUE", "CONVERT", "CREATE", "CROSS", "CURRENT",
            "CURRENT_DATE", "CURRENT_TIME", "CURRENT_TIMESTAMP", "CURRENT_USER", "CURSOR", "DATABASE",
            "DBCC", "DEALLOCATE", "DECLARE", "DEFAULT", "DELETE", "DENY", "DESC", "DISK", "DISTINCT",
            "DISTRIBUTED", "DOUBLE", "DROP", "DUMP", "ELSE", "END", "ERRLVL", "ESCAPE", "EXCEPT",
            "EXEC", "EXECUTE", "EXISTS", "EXIT", "EXTERNAL", "FETCH", "FILE", "FILLFACTOR", "FOR",
            "FOREIGN", "FREETEXT", "FREETEXTTABLE", "FROM", "FULL", "FUNCTION", "GOTO", "GRANT",
            "GROUP", "HAVING", "HOLDLOCK", "IDENTITY", "IDENTITYCOL", "IDENTITY_INSERT", "IF",
            "IN", "INDEX", "INNER", "INSERT", "INTERSECT", "INTO", "IS", "JOIN", "KEY", "KILL",
            "LEFT", "LIKE", "LINENO", "LOAD", "MERGE", "NATIONAL", "NOCHECK", "NONCLUSTERED",
            "NOT", "NULL", "NULLIF", "OF", "OFF", "OFFSETS", "ON", "OPEN", "OPENDATASOURCE",
            "OPENQUERY", "OPENROWSET", "OPENXML", "OPTION", "OR", "ORDER", "OUTER", "OVER",
            "PERCENT", "PIVOT", "PLAN", "PRECISION", "PRIMARY", "PRINT", "PROC", "PROCEDURE",
            "PUBLIC", "RAISERROR", "READ", "READTEXT", "RECONFIGURE", "REFERENCES", "REPLICATION",
            "RESTORE", "RESTRICT", "RETURN", "REVERT", "REVOKE", "RIGHT", "ROLLBACK", "ROWCOUNT",
            "ROWGUIDCOL", "RULE", "SAVE", "SCHEMA", "SECURITYAUDIT", "SELECT", "SEMANTICKEYPHRASETABLE",
            "SEMANTICSIMILARITYDETAILSTABLE", "SEMANTICSIMILARITYTABLE", "SESSION_USER", "SET",
            "SETUSER", "SHUTDOWN", "SOME", "STATISTICS", "SYSTEM_USER", "TABLE", "TABLESAMPLE",
            "TEXTSIZE", "THEN", "TO", "TOP", "TRAN", "TRANSACTION", "TRIGGER", "TRUNCATE", "TRY_CONVERT",
            "TSEQUAL", "UNION", "UNIQUE", "UNPIVOT", "UPDATE", "UPDATETEXT", "USE", "USER", "VALUES",
            "VARYING", "VIEW", "WAITFOR", "WHEN", "WHERE", "WHILE", "WITH", "WITHIN", "WRITETEXT", "XML"
        ];

        for keyword in mssql_specific {
            keywords.insert(keyword.to_string());
        }
        keywords
    }

    fn oracle_keywords() -> HashSet<String> {
        let mut keywords = Self::standard_keywords();
        let oracle_specific = vec![
            "ACCESS", "ADD", "ALL", "ALTER", "AND", "ANY", "AS", "ASC", "AUDIT", "BETWEEN", "BY",
            "CHAR", "CHECK", "CLUSTER", "COLUMN", "COMMENT", "COMPRESS", "CONNECT", "CREATE", "CURRENT",
            "DATE", "DECIMAL", "DEFAULT", "DELETE", "DESC", "DISTINCT", "DROP", "ELSE", "EXCLUSIVE",
            "EXISTS", "FILE", "FLOAT", "FOR", "FROM", "GRANT", "GROUP", "HAVING", "IDENTIFIED", "IMMEDIATE",
            "IN", "INCREMENT", "INDEX", "INITIAL", "INSERT", "INTEGER", "INTERSECT", "INTO", "IS", "LEVEL",
            "LIKE", "LOCK", "LONG", "MAXEXTENTS", "MINUS", "MLSLABEL", "MODE", "MODIFY", "NOAUDIT",
            "NOCOMPRESS", "NOT", "NOWAIT", "NULL", "NUMBER", "OF", "OFFLINE", "ON", "ONLINE", "OPTION",
            "OR", "ORDER", "PCTFREE", "PRIOR", "PRIVILEGES", "PUBLIC", "RAW", "RENAME", "RESOURCE",
            "REVOKE", "ROW", "ROWID", "ROWNUM", "ROWS", "SELECT", "SESSION", "SET", "SHARE", "SIZE",
            "SMALLINT", "START", "SUCCESSFUL", "SYNONYM", "SYSDATE", "TABLE", "THEN", "TO", "TRIGGER",
            "UID", "UNION", "UNIQUE", "UPDATE", "USER", "VALIDATE", "VALUES", "VARCHAR", "VARCHAR2",
            "VIEW", "WHENEVER", "WHERE", "WITH"
        ];

        for keyword in oracle_specific {
            keywords.insert(keyword.to_string());
        }
        keywords
    }

    fn standard_keywords() -> HashSet<String> {
        let keywords = vec![
            "SELECT", "FROM", "WHERE", "JOIN", "INNER", "LEFT", "RIGHT", "FULL", "OUTER", "CROSS",
            "ON", "USING", "GROUP", "BY", "HAVING", "ORDER", "ASC", "DESC", "LIMIT", "OFFSET",
            "INSERT", "INTO", "VALUES", "UPDATE", "SET", "DELETE", "CREATE", "TABLE", "INDEX",
            "ALTER", "DROP", "ADD", "MODIFY", "RENAME", "COLUMN", "CONSTRAINT", "PRIMARY", "FOREIGN",
            "KEY", "UNIQUE", "CHECK", "DEFAULT", "NOT", "NULL", "AUTO_INCREMENT", "AUTOINCREMENT",
            "IF", "EXISTS", "DISTINCT", "AS", "AND", "OR", "IN", "BETWEEN", "LIKE", "ILIKE", "IS",
            "UNION", "ALL", "INTERSECT", "EXCEPT", "WITH", "RECURSIVE", "CASE", "WHEN", "THEN",
            "ELSE", "END", "CAST"
        ];

        keywords.into_iter().map(|s| s.to_string()).collect()
    }

    fn postgresql_data_types() -> HashMap<String, DataType> {
        let mut types = HashMap::new();
        types.insert("SERIAL".to_string(), DataType::Integer { size: Some(32), unsigned: false });
        types.insert("BIGSERIAL".to_string(), DataType::BigInt { unsigned: false });
        types.insert("SMALLSERIAL".to_string(), DataType::SmallInt { unsigned: false });
        types.insert("MONEY".to_string(), DataType::Decimal { precision: Some(10), scale: Some(2) });
        types.insert("INET".to_string(), DataType::Custom("INET".to_string()));
        types.insert("CIDR".to_string(), DataType::Custom("CIDR".to_string()));
        types.insert("MACADDR".to_string(), DataType::Custom("MACADDR".to_string()));
        types.insert("TSVECTOR".to_string(), DataType::Custom("TSVECTOR".to_string()));
        types.insert("TSQUERY".to_string(), DataType::Custom("TSQUERY".to_string()));
        types.insert("UUID".to_string(), DataType::Uuid);
        types.insert("JSON".to_string(), DataType::Json);
        types.insert("JSONB".to_string(), DataType::Json);
        types.insert("XML".to_string(), DataType::Custom("XML".to_string()));
        types.insert("POINT".to_string(), DataType::Custom("POINT".to_string()));
        types.insert("LINE".to_string(), DataType::Custom("LINE".to_string()));
        types.insert("LSEG".to_string(), DataType::Custom("LSEG".to_string()));
        types.insert("BOX".to_string(), DataType::Custom("BOX".to_string()));
        types.insert("PATH".to_string(), DataType::Custom("PATH".to_string()));
        types.insert("POLYGON".to_string(), DataType::Custom("POLYGON".to_string()));
        types.insert("CIRCLE".to_string(), DataType::Custom("CIRCLE".to_string()));
        types
    }

    fn mysql_data_types() -> HashMap<String, DataType> {
        let mut types = HashMap::new();
        types.insert("TINYINT".to_string(), DataType::TinyInt { unsigned: false });
        types.insert("MEDIUMINT".to_string(), DataType::Custom("MEDIUMINT".to_string()));
        types.insert("BIGINT".to_string(), DataType::BigInt { unsigned: false });
        types.insert("FLOAT".to_string(), DataType::Float { precision: None });
        types.insert("DOUBLE".to_string(), DataType::Double);
        types.insert("DECIMAL".to_string(), DataType::Decimal { precision: None, scale: None });
        types.insert("NUMERIC".to_string(), DataType::Decimal { precision: None, scale: None });
        types.insert("BIT".to_string(), DataType::Custom("BIT".to_string()));
        types.insert("YEAR".to_string(), DataType::Custom("YEAR".to_string()));
        types.insert("TINYTEXT".to_string(), DataType::Custom("TINYTEXT".to_string()));
        types.insert("MEDIUMTEXT".to_string(), DataType::Custom("MEDIUMTEXT".to_string()));
        types.insert("LONGTEXT".to_string(), DataType::Custom("LONGTEXT".to_string()));
        types.insert("TINYBLOB".to_string(), DataType::Custom("TINYBLOB".to_string()));
        types.insert("MEDIUMBLOB".to_string(), DataType::Custom("MEDIUMBLOB".to_string()));
        types.insert("LONGBLOB".to_string(), DataType::Custom("LONGBLOB".to_string()));
        types.insert("ENUM".to_string(), DataType::Custom("ENUM".to_string()));
        types.insert("SET".to_string(), DataType::Custom("SET".to_string()));
        types.insert("GEOMETRY".to_string(), DataType::Custom("GEOMETRY".to_string()));
        types.insert("POINT".to_string(), DataType::Custom("POINT".to_string()));
        types.insert("LINESTRING".to_string(), DataType::Custom("LINESTRING".to_string()));
        types.insert("POLYGON".to_string(), DataType::Custom("POLYGON".to_string()));
        types.insert("MULTIPOINT".to_string(), DataType::Custom("MULTIPOINT".to_string()));
        types.insert("MULTILINESTRING".to_string(), DataType::Custom("MULTILINESTRING".to_string()));
        types.insert("MULTIPOLYGON".to_string(), DataType::Custom("MULTIPOLYGON".to_string()));
        types.insert("GEOMETRYCOLLECTION".to_string(), DataType::Custom("GEOMETRYCOLLECTION".to_string()));
        types.insert("JSON".to_string(), DataType::Json);
        types
    }

    fn sqlite_data_types() -> HashMap<String, DataType> {
        let mut types = HashMap::new();
        types.insert("INTEGER".to_string(), DataType::Integer { size: Some(64), unsigned: false });
        types.insert("REAL".to_string(), DataType::Float { precision: Some(53) });
        types.insert("NUMERIC".to_string(), DataType::Decimal { precision: None, scale: None });
        types
    }

    fn mssql_data_types() -> HashMap<String, DataType> {
        let mut types = HashMap::new();
        types.insert("BIT".to_string(), DataType::Custom("BIT".to_string()));
        types.insert("TINYINT".to_string(), DataType::TinyInt { unsigned: true });
        types.insert("SMALLINT".to_string(), DataType::SmallInt { unsigned: true });
        types.insert("INT".to_string(), DataType::Integer { size: Some(32), unsigned: true });
        types.insert("BIGINT".to_string(), DataType::BigInt { unsigned: true });
        types.insert("SMALLMONEY".to_string(), DataType::Custom("SMALLMONEY".to_string()));
        types.insert("MONEY".to_string(), DataType::Custom("MONEY".to_string()));
        types.insert("REAL".to_string(), DataType::Float { precision: Some(24) });
        types.insert("FLOAT".to_string(), DataType::Float { precision: Some(53) });
        types.insert("DECIMAL".to_string(), DataType::Decimal { precision: None, scale: None });
        types.insert("NUMERIC".to_string(), DataType::Decimal { precision: None, scale: None });
        types.insert("CHAR".to_string(), DataType::Char { length: None });
        types.insert("VARCHAR".to_string(), DataType::Varchar { length: None });
        types.insert("TEXT".to_string(), DataType::Text);
        types.insert("NCHAR".to_string(), DataType::Custom("NCHAR".to_string()));
        types.insert("NVARCHAR".to_string(), DataType::Custom("NVARCHAR".to_string()));
        types.insert("NTEXT".to_string(), DataType::Custom("NTEXT".to_string()));
        types.insert("BINARY".to_string(), DataType::Binary { length: None });
        types.insert("VARBINARY".to_string(), DataType::Varbinary { length: None });
        types.insert("IMAGE".to_string(), DataType::Blob);
        types.insert("UNIQUEIDENTIFIER".to_string(), DataType::Uuid);
        types.insert("XML".to_string(), DataType::Custom("XML".to_string()));
        types.insert("GEOGRAPHY".to_string(), DataType::Custom("GEOGRAPHY".to_string()));
        types.insert("GEOMETRY".to_string(), DataType::Custom("GEOMETRY".to_string()));
        types
    }

    fn oracle_data_types() -> HashMap<String, DataType> {
        let mut types = HashMap::new();
        types.insert("NUMBER".to_string(), DataType::Decimal { precision: None, scale: None });
        types.insert("PLS_INTEGER".to_string(), DataType::Integer { size: Some(32), unsigned: false });
        types.insert("BINARY_INTEGER".to_string(), DataType::Integer { size: Some(32), unsigned: false });
        types.insert("SIMPLE_INTEGER".to_string(), DataType::Integer { size: Some(32), unsigned: false });
        types.insert("NATURAL".to_string(), DataType::Integer { size: Some(32), unsigned: true });
        types.insert("POSITIVE".to_string(), DataType::Integer { size: Some(32), unsigned: true });
        types.insert("SIGNTYPE".to_string(), DataType::TinyInt { unsigned: false });
        types.insert("VARCHAR2".to_string(), DataType::Varchar { length: None });
        types.insert("NVARCHAR2".to_string(), DataType::Custom("NVARCHAR2".to_string()));
        types.insert("CLOB".to_string(), DataType::Text);
        types.insert("NCLOB".to_string(), DataType::Custom("NCLOB".to_string()));
        types.insert("BLOB".to_string(), DataType::Blob);
        types.insert("BFILE".to_string(), DataType::Custom("BFILE".to_string()));
        types.insert("RAW".to_string(), DataType::Varbinary { length: None });
        types.insert("ROWID".to_string(), DataType::Custom("ROWID".to_string()));
        types.insert("UROWID".to_string(), DataType::Custom("UROWID".to_string()));
        types.insert("DATE".to_string(), DataType::DateTime);
        types.insert("TIMESTAMP".to_string(), DataType::Timestamp);
        types.insert("INTERVAL".to_string(), DataType::Custom("INTERVAL".to_string()));
        types
    }

    fn standard_data_types() -> HashMap<String, DataType> {
        let mut types = HashMap::new();
        types.insert("BOOLEAN".to_string(), DataType::Boolean);
        types.insert("BOOL".to_string(), DataType::Boolean);
        types.insert("SMALLINT".to_string(), DataType::SmallInt { unsigned: false });
        types.insert("INTEGER".to_string(), DataType::Integer { size: Some(32), unsigned: false });
        types.insert("INT".to_string(), DataType::Integer { size: Some(32), unsigned: false });
        types.insert("BIGINT".to_string(), DataType::BigInt { unsigned: false });
        types.insert("FLOAT".to_string(), DataType::Float { precision: None });
        types.insert("REAL".to_string(), DataType::Float { precision: Some(24) });
        types.insert("DOUBLE".to_string(), DataType::Double);
        types.insert("DECIMAL".to_string(), DataType::Decimal { precision: None, scale: None });
        types.insert("NUMERIC".to_string(), DataType::Decimal { precision: None, scale: None });
        types.insert("CHAR".to_string(), DataType::Char { length: None });
        types.insert("CHARACTER".to_string(), DataType::Char { length: None });
        types.insert("VARCHAR".to_string(), DataType::Varchar { length: None });
        types.insert("CHARACTER VARYING".to_string(), DataType::Varchar { length: None });
        types.insert("TEXT".to_string(), DataType::Text);
        types.insert("CLOB".to_string(), DataType::Text);
        types.insert("BINARY".to_string(), DataType::Binary { length: None });
        types.insert("VARBINARY".to_string(), DataType::Varbinary { length: None });
        types.insert("BLOB".to_string(), DataType::Blob);
        types.insert("DATE".to_string(), DataType::Date);
        types.insert("TIME".to_string(), DataType::Time);
        types.insert("TIMESTAMP".to_string(), DataType::Timestamp);
        types.insert("DATETIME".to_string(), DataType::DateTime);
        types.insert("UUID".to_string(), DataType::Uuid);
        types.insert("JSON".to_string(), DataType::Json);
        types
    }

    fn postgresql_functions() -> HashSet<String> {
        let functions = vec![
            "ABS", "ACOS", "ASIN", "ATAN", "ATAN2", "CEIL", "CEILING", "COS", "COT", "DEGREES",
            "DIV", "EXP", "FLOOR", "LN", "LOG", "LOG10", "MOD", "PI", "POWER", "RADIANS", "ROUND",
            "SIGN", "SIN", "SQRT", "TAN", "TRUNC", "WIDTH_BUCKET", "RANDOM", "SETSEED",
            "ARRAY_AGG", "AVG", "BIT_AND", "BIT_OR", "BOOL_AND", "BOOL_OR", "COUNT", "EVERY",
            "MAX", "MIN", "STDDEV", "STDDEV_POP", "STDDEV_SAMP", "SUM", "VAR_POP", "VAR_SAMP",
            "VARIANCE", "RANK", "DENSE_RANK", "PERCENT_RANK", "CUME_DIST", "NTILE", "ROW_NUMBER",
            "FIRST_VALUE", "LAST_VALUE", "LAG", "LEAD", "NTH_VALUE", "RATIO_TO_REPORT",
            "COALESCE", "NULLIF", "GREATEST", "LEAST", "CONCAT", "CONCAT_WS", "FORMAT",
            "LEFT", "RIGHT", "LENGTH", "LOWER", "UPPER", "LPAD", "RPAD", "LTRIM", "RTRIM",
            "TRIM", "SUBSTRING", "SUBSTR", "POSITION", "STRPOS", "REPLACE", "REGEXP_MATCH",
            "REGEXP_REPLACE", "REGEXP_SPLIT", "SPLIT_PART", "NOW", "CURRENT_DATE", "CURRENT_TIME",
            "CURRENT_TIMESTAMP", "EXTRACT", "DATE_PART", "DATE_TRUNC", "AGE", "TO_CHAR", "TO_DATE",
            "TO_TIMESTAMP", "GENERATE_SERIES", "UNNEST", "ARRAY_LENGTH", "CARDINALITY"
        ];
        functions.into_iter().map(|s| s.to_string()).collect()
    }

    fn mysql_functions() -> HashSet<String> {
        let functions = vec![
            "ABS", "ACOS", "ASIN", "ATAN", "ATAN2", "CEIL", "CEILING", "COS", "COT", "CRC32",
            "DEGREES", "EXP", "FLOOR", "LN", "LOG", "LOG10", "LOG2", "MOD", "PI", "POW", "POWER",
            "RADIANS", "RAND", "ROUND", "SIGN", "SIN", "SQRT", "TAN", "TRUNCATE",
            "BIT_AND", "BIT_OR", "BIT_XOR", "COUNT", "GROUP_CONCAT", "MAX", "MIN", "STD", "STDDEV",
            "STDDEV_POP", "STDDEV_SAMP", "SUM", "VAR_POP", "VAR_SAMP", "VARIANCE",
            "ROW_NUMBER", "RANK", "DENSE_RANK", "PERCENT_RANK", "CUME_DIST", "NTILE", "LAG", "LEAD",
            "FIRST_VALUE", "LAST_VALUE", "NTH_VALUE", "COALESCE", "IF", "IFNULL", "NULLIF",
            "GREATEST", "LEAST", "CONCAT", "CONCAT_WS", "ELT", "FIELD", "FIND_IN_SET", "FORMAT",
            "INSERT", "INSTR", "LCASE", "LEFT", "LENGTH", "LOAD_FILE", "LOCATE", "LOWER", "LPAD",
            "LTRIM", "MID", "POSITION", "QUOTE", "REPEAT", "REPLACE", "REVERSE", "RIGHT", "RPAD",
            "RTRIM", "SOUNDEX", "SPACE", "STRCMP", "SUBSTRING", "SUBSTR", "TRIM", "UCASE", "UNHEX",
            "UPPER", "WEIGHT_STRING", "NOW", "CURDATE", "CURTIME", "DATE", "DATEDIFF", "DATE_ADD",
            "DATE_SUB", "DATE_FORMAT", "DAY", "DAYNAME", "DAYOFMONTH", "DAYOFWEEK", "DAYOFYEAR",
            "EXTRACT", "FROM_DAYS", "FROM_UNIXTIME", "HOUR", "LAST_DAY", "MAKEDATE", "MAKETIME",
            "MICROSECOND", "MINUTE", "MONTH", "MONTHNAME", "QUARTER", "SECOND", "SEC_TO_TIME",
            "STR_TO_DATE", "TIME", "TIME_TO_SEC", "TIMEDIFF", "TIMESTAMP", "TIMESTAMPADD",
            "TIMESTAMPDIFF", "TO_DAYS", "TO_SECONDS", "UNIX_TIMESTAMP", "WEEK", "WEEKDAY",
            "WEEKOFYEAR", "YEAR", "YEARWEEK", "ADDDATE", "SUBDATE", "ADDTIME", "SUBTIME",
            "CONVERT_TZ", "GET_FORMAT", "INET_ATON", "INET_NTOA", "INET6_ATON", "INET6_NTOA",
            "IS_IPV4", "IS_IPV4_COMPAT", "IS_IPV4_MAPPED", "IS_IPV6", "UUID", "UUID_SHORT"
        ];
        functions.into_iter().map(|s| s.to_string()).collect()
    }

    fn sqlite_functions() -> HashSet<String> {
        let functions = vec![
            "ABS", "CHANGES", "CHAR", "COALESCE", "GLOB", "IFNULL", "INSTR", "HEX", "LAST_INSERT_ROWID",
            "LENGTH", "LIKE", "LIKELIKE", "LOAD_EXTENSION", "LOWER", "LTRIM", "MAX", "MIN", "NULLIF",
            "PRINT", "QUOTE", "RANDOM", "RANDOMBLOB", "REPLACE", "ROUND", "RTRIM", "SOUNDEX", "SQLITE_COMPILEOPTION_GET",
            "SQLITE_COMPILEOPTION_USED", "SQLITE_SOURCE_ID", "SQLITE_VERSION", "SUBSTR", "TOTAL", "TRIM",
            "TYPEOF", "UNICODE", "UNLIKELY", "UPPER", "ZEROBLOB", "DATE", "DATETIME", "JULIANDAY", "STRFTIME",
            "AVG", "COUNT", "GROUP_CONCAT", "MAX", "MIN", "SUM", "TOTAL", "ROW_NUMBER", "RANK", "DENSE_RANK",
            "PERCENT_RANK", "CUME_DIST", "NTILE", "LAG", "LEAD", "FIRST_VALUE", "LAST_VALUE", "NTH_VALUE"
        ];
        functions.into_iter().map(|s| s.to_string()).collect()
    }

    fn mssql_functions() -> HashSet<String> {
        let functions = vec![
            "ABS", "ACOS", "ASIN", "ATAN", "ATN2", "CEILING", "COS", "COT", "DEGREES", "EXP", "FLOOR",
            "LOG", "LOG10", "PI", "POWER", "RADIANS", "RAND", "ROUND", "SIGN", "SIN", "SQRT", "SQUARE",
            "TAN", "COUNT", "COUNT_BIG", "GROUPING", "GROUPING_ID", "MAX", "MIN", "SUM", "STDEV", "STDEVP",
            "VAR", "VARP", "CHECKSUM_AGG", "ROW_NUMBER", "RANK", "DENSE_RANK", "NTILE", "LAG", "LEAD",
            "FIRST_VALUE", "LAST_VALUE", "PERCENTILE_CONT", "PERCENTILE_DISC", "CUME_DIST", "PERCENT_RANK",
            "COALESCE", "ISNULL", "NULLIF", "CHOOSE", "IIF", "CONCAT", "FORMAT", "LEFT", "LEN", "LOWER",
            "LTRIM", "NCHAR", "PATINDEX", "REPLACE", "REPLICATE", "REVERSE", "RIGHT", "RTRIM", "SPACE",
            "STR", "STUFF", "SUBSTRING", "UNICODE", "UPPER", "ASCII", "CHAR", "CHARINDEX", "DIFFERENCE",
            "SOUNDEX", "GETDATE", "CURRENT_TIMESTAMP", "DATEADD", "DATEDIFF", "DATENAME", "DATEPART",
            "DAY", "MONTH", "YEAR", "CONVERT", "CAST", "PARSE", "TRY_CONVERT", "TRY_PARSE", "DATALENGTH",
            "ISDATE", "ISNUMERIC", "NEWID", "NEWSEQUENTIALID"
        ];
        functions.into_iter().map(|s| s.to_string()).collect()
    }

    fn oracle_functions() -> HashSet<String> {
        let functions = vec![
            "ABS", "ACOS", "ASIN", "ATAN", "ATAN2", "CEIL", "COS", "COSH", "EXP", "FLOOR", "LN", "LOG",
            "MOD", "POWER", "ROUND", "SIGN", "SIN", "SINH", "SQRT", "TAN", "TANH", "TRUNC",
            "COUNT", "SUM", "AVG", "MIN", "MAX", "STDDEV", "STDDEV_POP", "STDDEV_SAMP", "VARIANCE",
            "VAR_POP", "VAR_SAMP", "COVAR_POP", "COVAR_SAMP", "CORR", "REGR_SLOPE", "REGR_INTERCEPT",
            "REGR_COUNT", "REGR_R2", "REGR_AVGX", "REGR_AVGY", "REGR_SXX", "REGR_SYY", "REGR_SXY",
            "ROW_NUMBER", "RANK", "DENSE_RANK", "PERCENT_RANK", "CUME_DIST", "NTILE", "LAG", "LEAD",
            "FIRST_VALUE", "LAST_VALUE", "NTH_VALUE", "RATIO_TO_REPORT", "COALESCE", "NVL", "NVL2",
            "NULLIF", "DECODE", "CASE", "GREATEST", "LEAST", "CONCAT", "SUBSTR", "SUBSTRING", "LENGTH",
            "LEN", "INSTR", "REPLACE", "TRANSLATE", "TRIM", "LTRIM", "RTRIM", "LOWER", "UPPER",
            "INITCAP", "LPAD", "RPAD", "SYSDATE", "CURRENT_DATE", "CURRENT_TIMESTAMP", "LOCALTIMESTAMP",
            "ADD_MONTHS", "LAST_DAY", "MONTHS_BETWEEN", "NEXT_DAY", "ROUND", "TRUNC", "EXTRACT",
            "TO_CHAR", "TO_DATE", "TO_TIMESTAMP", "TO_NUMBER", "NLS_INITCAP", "NLS_LOWER", "NLS_UPPER",
            "ASCII", "CHR", "ROWIDTOCHAR", "USER", "UID", "USERENV"
        ];
        functions.into_iter().map(|s| s.to_string()).collect()
    }

    fn standard_functions() -> HashSet<String> {
        let functions = vec![
            "ABS", "ACOS", "ASIN", "ATAN", "ATAN2", "CEIL", "CEILING", "COS", "EXP", "FLOOR", "LN",
            "LOG", "LOG10", "MOD", "PI", "POWER", "ROUND", "SIGN", "SIN", "SQRT", "TAN", "TRUNC",
            "COUNT", "SUM", "AVG", "MIN", "MAX", "STDDEV", "STDDEV_POP", "STDDEV_SAMP", "VARIANCE",
            "VAR_POP", "VAR_SAMP", "ROW_NUMBER", "RANK", "DENSE_RANK", "PERCENT_RANK", "CUME_DIST",
            "NTILE", "LAG", "LEAD", "FIRST_VALUE", "LAST_VALUE", "NTH_VALUE", "COALESCE", "NULLIF",
            "GREATEST", "LEAST", "CONCAT", "SUBSTRING", "SUBSTR", "LENGTH", "LEN", "POSITION",
            "LOCATE", "REPLACE", "TRIM", "LTRIM", "RTRIM", "LOWER", "UPPER", "NOW", "CURRENT_DATE",
            "CURRENT_TIME", "CURRENT_TIMESTAMP", "EXTRACT", "DATE_PART", "YEAR", "MONTH", "DAY",
            "HOUR", "MINUTE", "SECOND", "CAST", "CONVERT"
        ];
        functions.into_iter().map(|s| s.to_string()).collect()
    }

    fn standard_operators() -> HashMap<String, BinaryOperator> {
        let mut operators = HashMap::new();
        operators.insert("||".to_string(), BinaryOperator::Concat);
        operators.insert("+".to_string(), BinaryOperator::Plus);
        operators.insert("-".to_string(), BinaryOperator::Minus);
        operators.insert("*".to_string(), BinaryOperator::Multiply);
        operators.insert("/".to_string(), BinaryOperator::Divide);
        operators.insert("%".to_string(), BinaryOperator::Modulo);
        operators.insert("&".to_string(), BinaryOperator::BitwiseAnd);
        operators.insert("|".to_string(), BinaryOperator::BitwiseOr);
        operators.insert("^".to_string(), BinaryOperator::BitwiseXor);
        operators.insert("<<".to_string(), BinaryOperator::LeftShift);
        operators.insert(">>".to_string(), BinaryOperator::RightShift);
        operators.insert("=".to_string(), BinaryOperator::Eq);
        operators.insert("<>".to_string(), BinaryOperator::Neq);
        operators.insert("!=".to_string(), BinaryOperator::Neq);
        operators.insert("<".to_string(), BinaryOperator::Lt);
        operators.insert("<=".to_string(), BinaryOperator::Lte);
        operators.insert(">".to_string(), BinaryOperator::Gt);
        operators.insert(">=".to_string(), BinaryOperator::Gte);
        operators.insert("AND".to_string(), BinaryOperator::And);
        operators.insert("OR".to_string(), BinaryOperator::Or);
        operators.insert("LIKE".to_string(), BinaryOperator::Like);
        operators.insert("ILIKE".to_string(), BinaryOperator::ILike);
        operators
    }
}

impl DialectConfig {
    pub fn is_keyword(&self, word: &str) -> bool {
        self.keywords.contains(&word.to_uppercase())
    }

    pub fn is_function(&self, word: &str) -> bool {
        self.functions.contains(&word.to_uppercase())
    }

    pub fn get_data_type(&self, type_name: &str) -> Option<&DataType> {
        self.data_types.get(&type_name.to_uppercase())
    }

    pub fn supports_feature(&self, feature: DialectFeature) -> bool {
        match feature {
            DialectFeature::AutoIncrement => self.supports_auto_increment,
            DialectFeature::Identity => self.supports_identity,
            DialectFeature::Cte => self.supports_cte,
            DialectFeature::RecursiveCte => self.supports_recursive_cte,
            DialectFeature::WindowFunctions => self.supports_window_functions,
        }
    }

    pub fn get_limit_keyword(&self) -> &str {
        &self.limit_keyword
    }

    pub fn get_offset_keyword(&self) -> &str {
        &self.offset_keyword
    }

    pub fn get_auto_increment_syntax(&self) -> &str {
        if self.supports_identity {
            &self.identity_keyword
        } else if self.supports_auto_increment {
            &self.auto_increment_keyword
        } else {
            ""
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DialectFeature {
    AutoIncrement,
    Identity,
    Cte,
    RecursiveCte,
    WindowFunctions,
}

pub struct DialectRegistry {
    dialects: HashMap<String, SqlDialect>,
}

impl DialectRegistry {
    pub fn new() -> Self {
        let mut dialects = HashMap::new();
        dialects.insert("postgresql".to_string(), SqlDialect::PostgreSQL);
        dialects.insert("postgres".to_string(), SqlDialect::PostgreSQL);
        dialects.insert("pg".to_string(), SqlDialect::PostgreSQL);
        dialects.insert("mysql".to_string(), SqlDialect::MySQL);
        dialects.insert("mariadb".to_string(), SqlDialect::MySQL);
        dialects.insert("sqlite".to_string(), SqlDialect::SQLite);
        dialects.insert("sqlite3".to_string(), SqlDialect::SQLite);
        dialects.insert("mssql".to_string(), SqlDialect::MSSQL);
        dialects.insert("sqlserver".to_string(), SqlDialect::MSSQL);
        dialects.insert("oracle".to_string(), SqlDialect::Oracle);
        dialects.insert("standard".to_string(), SqlDialect::Standard);
        dialects.insert("sql".to_string(), SqlDialect::Standard);
        dialects.insert("ansi".to_string(), SqlDialect::Standard);

        Self { dialects }
    }

    pub fn get_dialect(&self, name: &str) -> Option<SqlDialect> {
        self.dialects.get(&name.to_lowercase()).cloned()
    }

    pub fn supported_dialects(&self) -> Vec<String> {
        self.dialects.keys().cloned().collect()
    }
}

impl Default for DialectRegistry {
    fn default() -> Self {
        Self::new()
    }
}
