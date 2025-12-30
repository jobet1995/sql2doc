# SQL2Doc – Rust SQL to Multi-Format Documentation Generator

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

SQL2Doc is a **Rust-based system tool** that **parses SQL schemas and queries** and automatically generates **machine-readable API specifications** and **human-readable documentation**. It supports multiple formats including **OpenAPI (YAML/JSON), JSON Schema, XML Schema (XSD), Markdown, and HTML**.  

SQL2Doc ensures your **API and data documentation is always accurate, up-to-date, and fully derived from your SQL schema**, reducing documentation drift and improving developer productivity.

---

## Features

- **SQL Parsing & Domain Modeling**
  - Supports `CREATE TABLE`, `PRIMARY KEY`, `FOREIGN KEY`, `CHECK` constraints, indexes
  - Handles multiple SQL dialects: Postgres, MySQL, SQLite
  - Generates AST → domain model → API inference

- **Automatic API Inference**
  - CRUD endpoints derived from tables
  - Infers path parameters and relationships
  - Pagination and filtering support

- **Multi-Format Output**
  - OpenAPI 3.0 (YAML/JSON)
  - JSON Schema for validation
  - XML Schema (XSD)
  - Markdown and HTML human-readable documentation
  - Optional PDF generation via HTML

- **Human-Readable Documentation**
  - Table and field descriptions
  - Relationships and junction tables
  - Sample JSON/XML payloads
  - Validation rules derived from SQL constraints

- **CLI Tool**
  - Simple command-line interface for schema conversion
  - Configurable output formats and directories
  - Supports schema diffing and versioning

---

## Installation

You need **Rust 1.75+** installed. Then clone the repository:

```bash
git clone https://github.com/jobet1995/sql2doc.git
cd sql2doc
cargo build --release

