use serde::{Deserialize, Serialize};

use crate::core::ast::ddl::DdlStatement;
use crate::core::ast::dml::DmlStatement;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Ddl(DdlStatement),
    Dml(Box<DmlStatement>),
}
