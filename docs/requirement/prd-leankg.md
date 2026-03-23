# LeanKG PRD - Product Requirements Document

**Version:** 1.0  
**Date:** 2026-03-23  
**Status:** Draft  
**Author:** Product Owner  
**Target Users:** Software developers using AI coding tools (Cursor, OpenCode, Claude Code, etc.)

---

## 1. Executive Summary

LeanKG is a lightweight, local-first knowledge graph solution designed for developers who use AI-assisted coding tools. The primary purpose is to provide AI models with accurate, concise codebase context without scanning unnecessary code, avoiding context window dilution, and ensuring documentation stays up-to-date with business logic mapping.

Unlike heavy frameworks like Graphiti that require external databases (Neo4j) and cloud infrastructure, LeanKG runs entirely locally on macOS and Linux with minimal resource consumption. It automatically generates and maintains documentation while mapping business logic to the existing codebase.

## 2. Problem Statement

### 2.1 Current Pain Points

| Pain Point | Description |
|------------|-------------|
| **Context Window Dilution** | AI tools scan entire codebases, including irrelevant files, wasting context window tokens |
| **Outdated Documentation** | Manual docs quickly become stale; AI receives wrong context |
| **Business Logic Disconnect** | No clear mapping between business requirements and code implementation |
| **Token Waste** | Redundant code scanning generates unnecessary token costs |
| **Poor Code Generation** | AI lacks accurate context, producing incorrect or suboptimal code |
| **Feature Transfer Difficulty** | Onboarding new developers requires extensive code exploration |

### 2.2 Why Graphiti Is Not Suitable

- Requires Neo4j or similar external database (operational complexity)
- LLM API calls required for every episode ingestion (ongoing costs)
- Memory-intensive entity resolution
- No embedded deployment option
- Cannot run offline (network required)

## 3. Product Overview

### 3.1 Product Name

**LeanKG** - Lightweight Knowledge Graph for AI-Assisted Development

### 3.2 Product Type

Local-first knowledge graph with CLI and MCP server interface

### 3.3 Core Value Proposition

LeanKG enables AI coding tools to understand exactly what they need—nothing more, nothing less. It provides precise codebase context, automatic documentation, and business logic mapping while running entirely locally with minimal resource usage.

### 3.4 Target Users

1. **Primary:** Developers using AI coding assistants (Cursor, OpenCode, Claude Code, Codex, Windsurf)
2. **Secondary:** Development teams wanting self-hosted codebase intelligence
3. **Tertiary:** Individual developers needing better AI code generation

---

## 4. User Stories

| ID | User Story | Priority |
|----|------------|----------|
| US-01 | As a developer, I want LeanKG to index my codebase automatically so that AI tools have accurate context | Must Have |
| US-02 | As a developer, I want LeanKG to generate and update documentation automatically so that I don't have to write docs manually | Must Have |
| US-03 | As a developer, I want LeanKG to map business logic to code so that AI understands the "why" behind implementation | Must Have |
| US-04 | As a developer, I want LeanKG to expose an MCP server so that my AI tools can query the knowledge graph | Must Have |
| US-05 | As a developer, I want LeanKG to run as a CLI so that I can integrate it into my workflow | Must Have |
| US-06 | As a developer, I want LeanKG to use minimal resources so that it doesn't slow down my machine | Must Have |
| US-07 | As a developer, I want LeanKG to provide a lightweight UI so that I can explore the knowledge graph visually | Should Have |
| US-08 | As a developer, I want LeanKG to support multiple languages so that it works with my tech stack | Must Have |

---

## 5. Functional Requirements

### 5.1 Core Features

#### 5.1.1 Code Indexing and Dependency Graph

**FR-01:** Parse source code files and extract structural information (files, functions, classes, imports, exports)

**FR-02:** Build a dependency graph showing relationships between code elements

**FR-03:** Support multiple programming languages (initially: Go, TypeScript/JavaScript, Python, Rust)

**FR-04:** Incremental indexing - only re-index changed files

**FR-05:** Watch for file changes and auto-update the graph

#### 5.1.2 Auto Documentation Generation

**FR-06:** Generate markdown documentation from code structure

**FR-07:** Maintain documentation freshness - update on code changes

**FR-08:** Generate AGENTS.md, CLAUDE.md, and other AI context files

**FR-09:** Support custom documentation templates

**FR-10:** Include business logic descriptions in generated docs

#### 5.1.3 Business Logic to Code Mapping

**FR-11:** Allow annotating code with business logic descriptions

**FR-12:** Map user stories/features to specific code files and functions

**FR-13:** Generate feature-to-code traceability

**FR-14:** Support business logic queries ("which code handles user authentication?")

#### 5.1.4 Context Provisioning

**FR-15:** Provide targeted context to AI tools (not full codebase)

**FR-16:** Calculate and minimize token usage for context queries

**FR-17:** Support context templates (file summary, function summary, etc.)

**FR-18:** Query by relevance, not just file structure

#### 5.1.5 MCP Server Interface

**FR-19:** Expose knowledge graph via MCP protocol

**FR-20:** Provide tools for querying code relationships

**FR-21:** Support context retrieval for specific AI operations

**FR-22:** Authenticate MCP connections

#### 5.1.6 CLI Interface

**FR-23:** Initialize a new LeanKG project

**FR-24:** Index codebase with configurable options

**FR-25:** Query the knowledge graph from command line

**FR-26:** Generate documentation

**FR-27:** Manage business logic annotations

**FR-28:** Start/stop MCP server

#### 5.1.7 Lightweight Web UI

**FR-29:** Visualize code dependency graph

**FR-30:** Browse and search code elements

**FR-31:** View and edit business logic annotations

**FR-32:** Simple documentation viewer

### 5.2 Non-Functional Requirements

#### 5.2.1 Performance

| Metric | Target |
|--------|--------|
| Cold start time | < 2 seconds |
| Indexing speed | > 10,000 lines/second |
| Query response time | < 100ms |
| Memory usage (idle) | < 100MB |
| Memory usage (indexing) | < 500MB |
| Disk space (per 100K lines) | < 50MB |

#### 5.2.2 Compatibility

- **Operating Systems:** macOS (Apple Silicon + Intel), Linux (x64, ARM64)
- **Languages Supported (MVP):** Go, TypeScript/JavaScript, Python
- **AI Tools:** Cursor, OpenCode, Claude Code (compatible MCP)

#### 5.2.3 Security

- All data stored locally (no cloud sync for MVP)
- No external API calls except for optional LLM
- MCP authentication via local tokens

---

## 6. Technical Architecture

### 6.1 Technology Stack

| Component | Technology | Rationale |
|-----------|------------|-----------|
| Core Language | Go | Cross-platform, single binary, good performance |
| Database | libSQL (Turso) | Embedded, SQLite-compatible, no external process |
| Code Parsing | tree-sitter | Efficient, multi-language support |
| MCP Server | FastMCP or custom | Standard MCP protocol |
| CLI | Cobra | Standard Go CLI patterns |
| Web UI | HTMX + Go templates | Lightweight, no complex frontend |
| Embeddings | Optional (cloud API) | For semantic search (future) |

### 6.2 Data Model

```
CodeElement:
  - id: UUID
  - type: file | function | class | import | export
  - name: string
  - file_path: string
  - line_start: int
  - line_end: int
  - language: string
  - parent_id: UUID (optional)
  - metadata: JSON

Relationship:
  - id: UUID
  - source_id: UUID
  - target_id: UUID
  - type: imports | implements | calls | contains | exports
  - metadata: JSON

BusinessLogic:
  - id: UUID
  - element_id: UUID
  - description: string
  - user_story_id: string (optional)
  - feature_id: string (optional)

Document:
  - id: UUID
  - title: string
  - content: string
  - file_path: string
  - generated_from: UUID[]
  - last_updated: timestamp
```

---

## 7. Out of Scope (MVP)

The following features are explicitly out of scope for MVP:

1. **Vector embeddings / semantic search** - Rule-based only
2. **Cloud sync** - Fully local
3. **Multi-user / team features** - Single user only
4. **Advanced authentication** - Local token only
5. **Plugin system** - Future consideration
6. **Enterprise integrations** - Future consideration
7. **All programming languages** - MVP: Go, TS/JS, Python only
8. **AI-powered entity extraction** - Cloud LLM integration optional, rule-based default

---

## 8. Success Metrics

| Metric | Target |
|--------|--------|
| Token reduction vs full scan | > 80% |
| Documentation accuracy | > 95% |
| Indexing time (10K LOC) | < 30 seconds |
| MCP query latency | < 100ms |
| User onboarding time | < 5 minutes |
| Crash-free usage | > 99.9% |

---

## 9. Release Criteria

### 9.1 MVP Release Criteria

- [ ] Code indexing works for Go, TypeScript, Python
- [ ] Dependency graph builds correctly
- [ ] CLI commands functional (init, index, query, generate)
- [ ] MCP server exposes basic query tools
- [ ] Documentation generation produces valid markdown
- [ ] Business logic annotations can be created and queried
- [ ] Web UI shows basic graph visualization
- [ ] Resource usage within targets
- [ ] Documentation complete

### 9.2 Acceptance Criteria

1. Developer can install LeanKG via single command
2. Developer can initialize project with one command
3. Developer can index codebase with one command
4. AI tools can query LeanKG via MCP protocol
5. Generated documentation is accurate and usable
6. Business logic annotations are persisted and queryable
7. Resource usage stays within non-functional targets

---

## 10. Roadmap

### Phase 1: MVP (v0.1.0)
- Core indexing (Go, TS/JS, Python)
- Basic dependency graph
- CLI interface
- MCP server (basic queries)
- Documentation generation

### Phase 2: Enhanced Features (v0.2.0)
- Web UI improvements
- Business logic annotations
- More language support
- Incremental indexing optimization

### Phase 3: Advanced (v0.3.0)
- Vector embeddings
- Semantic search
- Cloud sync (optional)
- Team features

---

## 11. Appendix

### 11.1 Glossary

| Term | Definition |
|------|------------|
| Knowledge Graph | Graph structure storing entities and relationships from codebase |
| Code Indexing | Process of parsing code and extracting structural information |
| MCP Server | Model Context Protocol server for AI tool integration |
| Context Window | AI model's input capacity; LeanKG minimizes tokens needed |
| Business Logic Mapping | Linking code to business requirements |

### 11.2 References

- Graphiti: https://github.com/getzep/graphiti
- Turso/libSQL: https://github.com/tursodatabase/libsql
- tree-sitter: https://tree-sitter.github.io/tree-sitter/
- MCP Protocol: https://modelcontextprotocol.io/
