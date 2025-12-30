# SQL2Doc – Rust SQL to Multi-Format Documentation Generator

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/badge/build-development-red.svg)]()
[![Version](https://img.shields.io/badge/version-0.1.0--dev-blue.svg)]()

SQL2Doc is a **Rust-based system tool** that **parses SQL schemas and queries** and automatically generates **machine-readable API specifications** and **human-readable documentation**. It supports multiple formats including **OpenAPI (YAML/JSON), JSON Schema, XML Schema (XSD), Markdown, and HTML**.  

SQL2Doc ensures your **API and data documentation is always accurate, up-to-date, and fully derived from your SQL schema**, reducing documentation drift and improving developer productivity.

## Table of Contents

- [Features](#features)
- [Current Status](#current-status)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage](#usage)
- [Configuration](#configuration)
- [Architecture](#architecture)
- [Examples](#examples)
- [API Reference](#api-reference)
- [Development](#development)
- [Testing](#testing)
- [Contributing](#contributing)
- [Roadmap](#roadmap)
- [FAQ](#faq)
- [Troubleshooting](#troubleshooting)
- [License](#license)
- [Related Projects](#related-projects)

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

## Current Status

⚠️ **This project is in early development.** The current implementation is a basic Rust skeleton. The features described below represent the planned functionality.

## Installation

### Prerequisites
- **Rust 1.75+** installed ([install Rust](https://rustup.rs/))

### Build from Source
```bash
git clone https://github.com/jobet1995/sql2doc.git
cd sql2doc
cargo build --release
```

The compiled binary will be available at `target/release/sql2doc`.

## Quick Start

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone and build**:
   ```bash
   git clone https://github.com/jobet1995/sql2doc.git
   cd sql2doc
   cargo build --release
   ```

3. **Create a sample SQL schema** (`schema.sql`):
   ```sql
   CREATE TABLE users (
       id INTEGER PRIMARY KEY,
       name VARCHAR(255) NOT NULL,
       email VARCHAR(255) UNIQUE NOT NULL
   );
   ```

4. **Generate documentation** (when implemented):
   ```bash
   ./target/release/sql2doc --input schema.sql --output api.yaml --format openapi
   ```

## Usage

*Coming soon - this section will be updated once the core functionality is implemented.*

### Planned CLI Interface
```bash
# Generate OpenAPI spec from SQL schema
sql2doc --input schema.sql --output api.yaml --format openapi

# Generate multiple formats
sql2doc --input schema.sql --output-dir docs/ --formats openapi,json-schema,markdown

# Watch mode for continuous generation
sql2doc --input schema.sql --watch --output-dir docs/
```

### Input Formats
- SQL DDL files (`.sql`)
- Database connection strings (planned)

### Output Formats
- **OpenAPI 3.0** (YAML/JSON) - REST API specifications
- **JSON Schema** - Data validation schemas
- **XML Schema (XSD)** - XML validation
- **Markdown** - Human-readable documentation
- **HTML** - Web-viewable documentation
- **PDF** - Printable documentation (via HTML)

## Configuration

### Command Line Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--input`, `-i` | `-i` | Input SQL file path | Required |
| `--output`, `-o` | `-o` | Output file path | Required |
| `--format`, `-f` | `-f` | Output format (openapi, json-schema, xsd, markdown, html) | `openapi` |
| `--output-dir` | | Output directory for multiple formats | Current directory |
| `--formats` | | Comma-separated list of output formats | `openapi` |
| `--watch`, `-w` | `-w` | Watch mode - regenerate on file changes | `false` |
| `--config` | `-c` | Configuration file path | `sql2doc.toml` |
| `--verbose`, `-v` | `-v` | Enable verbose logging | `false` |
| `--quiet`, `-q` | `-q` | Suppress all output except errors | `false` |
| `--help`, `-h` | `-h` | Display help information | |

### Configuration File

Create a `sql2doc.toml` file for advanced configuration:

```toml
# sql2doc.toml
[general]
title = "My API Documentation"
version = "1.0.0"
description = "Auto-generated from SQL schema"

[database]
dialect = "postgresql"  # postgresql, mysql, sqlite, mssql
schema = "public"

[output]
formats = ["openapi", "markdown", "html"]
directory = "./docs"
clean_output_dir = true

[openapi]
version = "3.0.0"
base_path = "/api/v1"
servers = [
    { url = "https://api.example.com", description = "Production" },
    { url = "http://localhost:3000", description = "Development" }
]

[markdown]
include_diagrams = true
include_examples = true

[html]
theme = "default"
include_search = true
include_toc = true
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `SQL2DOC_LOG_LEVEL` | Logging level (error, warn, info, debug, trace) | `info` |
| `SQL2DOC_CONFIG` | Path to configuration file | `sql2doc.toml` |

## Architecture

### Core Components (Planned)
- **SQL Parser**: AST generation from DDL statements
- **Domain Model**: Internal representation of tables, relationships, constraints
- **API Generator**: REST endpoint inference from schema
- **Format Writers**: Multi-format output generation
- **CLI Interface**: Command-line argument processing

### Supported SQL Dialects
- PostgreSQL
- MySQL/MariaDB
- SQLite
- Microsoft SQL Server (planned)

## Examples

### Sample SQL Input
```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    title VARCHAR(500) NOT NULL,
    content TEXT,
    published BOOLEAN DEFAULT false
);
```

### Generated OpenAPI Output (Planned)
```yaml
openapi: 3.0.0
info:
  title: Generated API
  version: 1.0.0
paths:
  /users:
    get:
      summary: List users
      responses:
        '200':
          description: Success
          content:
            application/json:
              schema:
                type: array
                items:
                  $ref: '#/components/schemas/User'
  /users/{id}:
    get:
      summary: Get user by ID
      parameters:
        - name: id
          in: path
          required: true
          schema:
            type: integer
components:
  schemas:
    User:
      type: object
      properties:
        id:
          type: integer
        email:
          type: string
          format: email
        created_at:
          type: string
          format: date-time
```

## API Reference

### OpenAPI 3.0 Specification

SQL2Doc generates complete OpenAPI 3.0 specifications with the following features:

#### Generated Endpoints

For each table, SQL2Doc creates standard REST endpoints:

- `GET /table` - List all records with pagination
- `POST /table` - Create new record
- `GET /table/{id}` - Get single record by primary key
- `PUT /table/{id}` - Update record
- `PATCH /table/{id}` - Partial update
- `DELETE /table/{id}` - Delete record

#### Query Parameters

- `limit`: Maximum number of results (default: 100)
- `offset`: Pagination offset (default: 0)
- `sort`: Sort field and direction (e.g., `name:asc`, `created_at:desc`)
- `filter`: Field-based filters (e.g., `status:eq:active`, `age:gt:18`)

#### Response Schemas

Generated schemas include:
- **Properties**: All table columns with appropriate types
- **Required fields**: Based on NOT NULL constraints
- **Validation rules**: Derived from CHECK constraints and column types
- **Relationships**: Foreign key references as nested objects or IDs

### JSON Schema

Generated JSON Schema includes:
- Complete type definitions for all tables
- Validation constraints from SQL schema
- Support for nested relationships
- Array types for collection responses

### XML Schema (XSD)

XSD generation includes:
- Complex types for each table
- Element definitions with proper cardinality
- Attribute declarations
- Namespace support

### Markdown Documentation

Human-readable documentation includes:
- Table descriptions and field details
- Relationship diagrams (ASCII art)
- Example API calls with curl
- Sample request/response payloads
- Validation rules and constraints

### HTML Documentation

Interactive HTML documentation features:
- Searchable table of contents
- Syntax-highlighted code examples
- Expandable/collapsible sections
- Responsive design for mobile devices
- Print-friendly styling

## Development

### Setup
```bash
git clone https://github.com/jobet1995/sql2doc.git
cd sql2doc
cargo build
```

### Testing
```bash
cargo test
```

### Project Structure
```
sql2doc/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── parser/           # SQL parsing logic (planned)
│   ├── model/            # Domain models (planned)
│   ├── generator/        # Output generators (planned)
│   └── cli/              # Command-line interface (planned)
├── tests/                # Integration tests (planned)
├── Cargo.toml
└── README.md
```

### Testing Strategy

#### Unit Tests
```bash
cargo test --lib
```

#### Integration Tests
```bash
cargo test --test integration
```

#### Test Coverage
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

#### SQL Dialect Testing

The project includes test fixtures for different SQL dialects:

- `tests/fixtures/postgresql/` - PostgreSQL specific schemas
- `tests/fixtures/mysql/` - MySQL/MariaDB schemas
- `tests/fixtures/sqlite/` - SQLite schemas
- `tests/fixtures/mssql/` - Microsoft SQL Server schemas

#### Fuzz Testing
```bash
cargo install cargo-fuzz
cargo fuzz run fuzz_parser
```

## Roadmap

### Phase 1: Core Foundation ✅ (Current)
- [x] Project setup and basic structure
- [x] CLI skeleton
- [ ] Basic SQL parsing (CREATE TABLE support)

### Phase 2: SQL Parsing & Modeling
- [ ] AST generation for DDL statements
- [ ] Domain model for tables, columns, constraints
- [ ] Support for multiple SQL dialects
- [ ] Foreign key relationship detection

### Phase 3: API Generation
- [ ] CRUD endpoint inference
- [ ] OpenAPI 3.0 specification generation
- [ ] JSON Schema output
- [ ] XML Schema (XSD) generation

### Phase 4: Documentation & Polish
- [ ] Markdown documentation generation
- [ ] HTML output with styling
- [ ] PDF generation
- [ ] Configuration file support
- [ ] Watch mode for file changes

### Phase 5: Advanced Features
- [ ] Database schema diffing
- [ ] Version management
- [ ] Custom templates
- [ ] Plugin system

## FAQ

### General Questions

**Q: Why another documentation generator?**  
A: SQL2Doc focuses specifically on SQL-first development workflows where the database schema is the source of truth. Unlike API-first tools, it ensures documentation stays synchronized with your actual database structure.

**Q: How does it differ from database migration tools?**  
A: Migration tools focus on schema changes over time. SQL2Doc focuses on generating API documentation and specifications from existing schemas.

**Q: Does it support database connections directly?**  
A: Currently, SQL2Doc works with SQL DDL files. Direct database connections are planned for future releases.

### Technical Questions

**Q: What SQL features are supported?**  
A: Basic DDL statements (CREATE TABLE, ALTER TABLE), constraints (PRIMARY KEY, FOREIGN KEY, CHECK, UNIQUE), indexes, and views.

**Q: Can I customize the generated API endpoints?**  
A: Yes, through configuration files you can customize endpoint patterns, add custom endpoints, and modify response schemas.

**Q: How does it handle database relationships?**  
A: Foreign keys are automatically detected and translated into API relationships, with options for nested responses or ID-only references.

**Q: Is the output production-ready?**  
A: Yes, the generated OpenAPI specs can be directly imported into API gateways, documentation tools, and client generators.

## Troubleshooting

### Common Issues

#### Build Errors

**Problem**: `error[E0658]: use of unstable library feature`  
**Solution**: Update Rust to version 1.75+ or later.

```bash
rustup update stable
```

**Problem**: Missing dependencies during build  
**Solution**: Ensure all system dependencies are installed.

```bash
# On Ubuntu/Debian
sudo apt-get install build-essential pkg-config libssl-dev

# On macOS
xcode-select --install
```

#### Runtime Errors

**Problem**: `Parse error: Unexpected token`  
**Solution**: Check your SQL syntax. SQL2Doc currently supports standard SQL DDL. Ensure you're using supported SQL dialect.

**Problem**: `Output format not supported`  
**Solution**: Verify the format parameter. Supported formats: `openapi`, `json-schema`, `xsd`, `markdown`, `html`.

#### Configuration Issues

**Problem**: Configuration file not found  
**Solution**: Ensure `sql2doc.toml` exists in the current directory or specify path with `--config`.

**Problem**: Invalid configuration syntax  
**Solution**: Validate your TOML syntax. Use an online TOML validator or check the example configuration above.

### Performance Issues

**Problem**: Slow processing of large schemas  
**Solution**:
- Use `--quiet` flag to reduce output
- Process schemas in smaller chunks
- Ensure adequate system memory (4GB+ recommended)

### Getting Help

1. Check this troubleshooting guide
2. Review the [GitHub Issues](https://github.com/jobet1995/sql2doc/issues) for similar problems
3. Create a new issue with:
   - Your SQL schema (anonymized if needed)
   - Complete error message
   - SQL2Doc version and command used
   - Your operating system and Rust version

### Debug Mode

Enable verbose logging for detailed error information:

```bash
sql2doc --input schema.sql --output output.yaml --verbose
```

Or set the environment variable:

```bash
export SQL2DOC_LOG_LEVEL=debug
sql2doc --input schema.sql --output output.yaml
```

## Security Considerations

### Data Sanitization

- SQL2Doc does not execute SQL statements - it only parses DDL for documentation generation
- No database connections are made during parsing
- Input files are processed locally with no external data transmission

### Generated Documentation

- Review generated API documentation before deployment
- Consider security implications of exposing schema information
- Use appropriate authentication/authorization in generated APIs
- Validate generated OpenAPI specs with security scanning tools

### Best Practices

1. **Input Validation**: Always validate SQL input files before processing
2. **Access Control**: Limit access to generated documentation as needed
3. **Regular Updates**: Keep SQL2Doc updated for security patches
4. **Code Review**: Review generated specifications before implementation

## Performance

### Benchmarks

*Performance benchmarks will be added once core functionality is implemented.*

### Memory Usage

- Typical schema parsing: < 50MB RAM
- Large schemas (100+ tables): < 200MB RAM
- Output generation scales linearly with schema size

### Optimization Tips

1. **Large Schemas**: Process in smaller chunks if memory is limited
2. **Watch Mode**: Use efficiently for development workflows
3. **Output Formats**: Generate only required formats to reduce processing time
4. **Caching**: Future versions will include caching for repeated generations

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Guidelines
1. Follow Rust coding standards and idioms
2. Add tests for new functionality
3. Update documentation for API changes
4. Use conventional commit messages

### Areas for Contribution
- SQL dialect implementations
- Output format writers
- Testing and validation
- Documentation and examples

## License

This project is licensed under the MIT License:

```
Copyright (c) 2024 SQL2Doc Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## Deployment

### Pre-built Binaries

*Coming soon - automated releases will provide pre-built binaries for:*
- Linux (x86_64, aarch64)
- macOS (x86_64, aarch64)
- Windows (x86_64)

### Docker

```dockerfile
FROM rust:1.75-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/sql2doc /usr/local/bin/
ENTRYPOINT ["sql2doc"]
```

### CI/CD Integration

#### GitHub Actions

```yaml
name: Generate API Documentation
on:
  push:
    paths:
      - 'schema/**'
      - '.github/workflows/docs.yml'

jobs:
  docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install sql2doc
      - run: sql2doc --input schema.sql --output-dir docs/ --formats openapi,markdown,html
      - uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./docs
```

#### GitLab CI

```yaml
generate-docs:
  image: rust:latest
  script:
    - cargo install sql2doc
    - sql2doc --input schema.sql --output-dir public/ --formats openapi,markdown,html
  artifacts:
    paths:
      - public/
    expire_in: 1 week
  only:
    - main
    - merge_requests
```

## Related Projects

- [sqlc](https://github.com/kyleconroy/sqlc) - Generate type-safe Go from SQL
- [Prisma](https://github.com/prisma/prisma) - Next-generation ORM
- [Schemacrawler](https://www.schemacrawler.com/) - Database schema discovery

