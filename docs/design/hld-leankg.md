# LeanKG High Level Design

**Phiên bản:** 1.0  
**Ngày:** 2026-03-23  
**Dựa trên:** PRD v1.0  
**Trạng thái:** Bản nháp

---

## 1. Tổng quan kiến trúc

### 1.1 Design Principles

| Principle | Mô tả |
|-----------|-------|
| **Local-first** | Tất cả data và xử lý đều chạy local, không phụ thuộc cloud |
| **Single binary** | Ứng dụng được pack thành một file binary duy nhất |
| **Minimal dependencies** | Không yêu cầu external services như database processes |
| **Incremental** | Chỉ xử lý thay đổi, không scan lại toàn bộ |
| **MCP-native** | Thiết kế từ đầu cho MCP protocol |

### 1.2 System Overview

LeanKG là một local-first knowledge graph system cung cấp codebase intelligence cho AI coding tools. Hệ thống parse code, build dependency graph, và expose interface qua CLI và MCP server.

---

## 2. C4 Models

### 2.1 Context Diagram (C4-1)

```mermaid
graph TB
    subgraph "User's Machine"
        subgraph "LeanKG System"
            KG[LeanKG Application]
            DB[(libSQL Database)]
            KG --- DB
        end
        
        AI1[Cursor]
        AI2[OpenCode]
        AI3[Claude Code]
        AI4[Other AI Tools]
        
        subgraph "CLI Users"
            Dev[Developer]
            CI[CI/CD Pipeline]
        end
        
        subgraph "Web UI Users"
            Browser[Browser]
        end
    end
    
    AI1 -->|MCP Protocol| KG
    AI2 -->|MCP Protocol| KG
    AI3 -->|MCP Protocol| KG
    AI4 -->|MCP Protocol| KG
    
    Dev -->|CLI Commands| KG
    CI -->|CLI Commands| KG
    
    Browser -->|HTTP| KG
    
    subgraph "External Services (Optional)"
        LLM[Cloud LLM API]
    end
    
    KG -.->|Optional API Calls| LLM
```

**Mô tả:**
- **LeanKG System:** Hệ thống chính chạy trên máy người dùng
- **AI Tools:** Cursor, OpenCode, Claude Code và các AI coding tools khác tương tác qua MCP protocol
- **Developer:** Sử dụng CLI để index, query, và generate documentation
- **CI/CD Pipeline:** Tự động hóa indexing trong quá trình build
- **Browser:** Truy cập lightweight web UI
- **Cloud LLM API:** Optional - cho future semantic search features

### 2.2 Container Diagram (C4-2)

```mermaid
graph TB
    subgraph "LeanKG Application"
        CLI[CLI Interface<br/>Cobra]
        MCP[MCP Server<br/>Protocol Handler]
        Web[Web UI Server<br/>HTMX]
        
        Indexer[Code Indexer<br/>tree-sitter]
        Graph[Graph Engine<br/>Query Processor]
        DocGen[Doc Generator]
        Watcher[File Watcher<br/>FSWatcher]
        
        DB[(libSQL<br/>Database)]
        
        CLI --> Indexer
        CLI --> Graph
        CLI --> DocGen
        
        MCP --> Graph
        MCP --> DocGen
        
        Web --> Graph
        Web --> DocGen
        
        Indexer --> DB
        Graph --> DB
        DocGen --> DB
        Watcher --> Indexer
    end
    
    Codebase[Codebase<br/>Source Files]
    Indexer -.->|Parses| Codebase
    
    Output[Generated<br/>Documentation]
    DocGen --> Output
```

**Containers:**

| Container | Responsibility | Technology |
|-----------|---------------|------------|
| CLI Interface | Command-line interaction | Cobra (Go) |
| MCP Server | MCP protocol communication | Custom Go implementation |
| Web UI Server | HTTP server for UI | Go + HTMX |
| Code Indexer | Parse source code with tree-sitter | tree-sitter + Go |
| Graph Engine | Query and traverse knowledge graph | Go |
| Doc Generator | Generate markdown documentation | Go templates |
| File Watcher | Monitor file changes | fsnotify |
| libSQL Database | Persistent storage | libSQL (Turso) |

**Interactions:**

1. **CLI → Indexer:** Developer chạy lệnh index
2. **CLI → Graph:** Developer query knowledge graph
3. **CLI → DocGen:** Developer generate documentation
4. **MCP → Graph:** AI tools query code relationships
5. **MCP → DocGen:** AI tools retrieve context
6. **Web → Graph:** User browse graph trong browser
7. **Indexer → DB:** Store parsed code elements
8. **Watcher → Indexer:** Trigger re-index khi files thay đổi

### 2.3 Component Diagram (C4-3)

```mermaid
graph TB
    subgraph "Code Indexer"
        Parser[Parser Manager]
        GoParser[Go Parser]
        TSParser[TS/JS Parser]
        PyParser[Python Parser]
        Extractor[Entity Extractor]
        Builder[Graph Builder]
        
        Parser --> GoParser
        Parser --> TSParser
        Parser --> PyParser
        
        GoParser --> Extractor
        TSParser --> Extractor
        PyParser --> Extractor
        
        Extractor --> Builder
    end
    
    subgraph "Graph Engine"
        Query[Query Processor]
        Search[Search Engine]
        Relate[Relationship Engine]
        Cache[Query Cache]
        
        Query --> Search
        Query --> Relate
        Search --> Cache
        Relate --> Cache
    end
    
    subgraph "Doc Generator"
        Template[Template Engine]
        Renderer[Markdown Renderer]
        Format[Formatter]
        Sync[Doc Sync Manager]
        
        Template --> Renderer
        Renderer --> Format
        Format --> Sync
    end
    
    subgraph "MCP Server"
        Protocol[MCP Protocol]
        Handler[Request Handler]
        Tools[Tool Registry]
        Auth[Auth Manager]
        
        Protocol --> Handler
        Handler --> Tools
        Handler --> Auth
    end
    
    Builder -->|Writes| DB[(libSQL)]
    Query -.->|Reads| DB
    Sync -.->|Reads| DB
    Tools -.->|Queries| Graph
```

**Components:**

| Component | Responsibility |
|-----------|----------------|
| Parser Manager | Language detection và parser delegation |
| Go Parser | Parse Go source files |
| TS/JS Parser | Parse TypeScript/JavaScript files |
| Python Parser | Parse Python files |
| Entity Extractor | Extract functions, classes, imports |
| Graph Builder | Build relationships và store to DB |
| Query Processor | Process user queries |
| Search Engine | Search code elements |
| Relationship Engine | Traverse graph relationships |
| Query Cache | Cache frequent queries |
| Template Engine | Load documentation templates |
| Markdown Renderer | Render markdown output |
| Formatter | Format documentation |
| Doc Sync Manager | Sync docs với code changes |
| MCP Protocol | Handle MCP protocol messages |
| Request Handler | Route requests to appropriate tools |
| Tool Registry | Register available MCP tools |
| Auth Manager | Authenticate MCP connections |

### 2.4 Deployment Diagram (C4-4)

```mermaid
graph TB
    subgraph "Developer Machine"
        subgraph "Operating System"
            subgraph "LeanKG Deployment"
                Binary[LeanKG Binary]
                
                subgraph "Runtime Components"
                    HTTP[HTTP Server<br/>:8080]
                    MCP[MCP Server<br/>:3000]
                    Watch[File Watcher]
                end
                
                subgraph "Data Layer"
                    DB[(libSQL<br/>~/.leankg/data.db)]
                    Cache[Query Cache<br/>In-memory]
                end
                
                subgraph "Processing"
                    Parser[Parser Pool<br/>tree-sitter]
                    Graph[Graph Engine]
                end
            end
            
            subgraph "File System"
                Code[Project Code<br/>/path/to/project]
                Config[Config<br/>/.leankg]
                Docs[Generated Docs<br/>/path/to/docs]
            end
        end
    end
    
    HTTP --> Binary
    MCP --> Binary
    Watch --> Binary
    
    Binary --> Parser
    Binary --> Graph
    
    Parser --> Code
    Graph --> DB
    Cache --> DB
    
    Docs -.->|Writes| Binary
    Config -.->|Reads| Binary
```

**Deployment Scenarios:**

| Scenario | Environment | Resources |
|----------|--------------|------------|
| macOS Intel | macOS x64 | < 100MB RAM, < 200MB disk |
| macOS Apple Silicon | macOS ARM64 | < 100MB RAM, < 200MB disk |
| Linux x64 | Linux x64 | < 100MB RAM, < 200MB disk |
| Linux ARM64 | Linux ARM64 | < 100MB RAM, < 200MB disk |

**Processes:**

| Process | Port | Description |
|---------|------|-------------|
| LeanKG Binary | - | Main application process |
| HTTP Server | 8080 | Web UI server (optional) |
| MCP Server | 3000 | MCP protocol endpoint |
| File Watcher | - | Background fsnotify process |
| libSQL | - | Embedded in-process database |

---

## 3. Data Flow

### 3.1 Indexing Flow

```mermaid
sequenceDiagram
    participant Dev as Developer
    participant CLI as CLI
    participant Watch as File Watcher
    participant Parse as Parser
    participant Extract as Entity Extractor
    participant Build as Graph Builder
    participant DB as libSQL

    Dev->>CLI: leankg index ./src
    CLI->>Parse: Parse directory
    Parse->>Extract: Extract entities
    Extract->>Build: Build relationships
    Build->>DB: Store elements
    
    Note over Watch: File change detected
    Watch->>Parse: Re-parse changed file
    Parse->>Extract: Extract entities
    Extract->>Build: Update graph
    Build->>DB: Upsert elements
```

### 3.2 Query Flow

```mermaid
sequenceDiagram
    participant AI as AI Tool
    participant MCP as MCP Server
    participant Query as Query Processor
    participant Search as Search Engine
    participant Relate as Relationship Engine
    participant DB as libSQL

    AI->>MCP: query_dependencies(file.ts)
    MCP->>Query: Process query
    Query->>Search: Find file element
    Search->>DB: SQL query
    DB-->>Search: Results
    Search->>Relate: Find related elements
    Relate->>DB: Traverse relationships
    DB-->>Relate: Graph results
    Relate-->>Query: Formatted results
    Query-->>MCP: MCP response
    MCP-->>AI: Context payload
```

### 3.3 Documentation Generation Flow

```mermaid
sequenceDiagram
    participant Dev as Developer
    participant CLI as CLI
    participant Doc as Doc Generator
    participant Template as Template Engine
    participant Render as Renderer
    participant DB as libSQL

    Dev->>CLI: leankg generate docs
    CLI->>Doc: Generate all
    Doc->>Template: Load templates
    Template->>DB: Query structure
    DB-->>Template: Code elements
    Template->>Render: Render markdown
    Render->>Doc: Generated docs
    Doc->>Doc: Write to filesystem
```

---

## 4. Data Model

### 4.1 Entity Relationship Diagram

```mermaid
erDiagram
    CODE_ELEMENTS ||--o{ RELATIONSHIPS : has
    CODE_ELEMENTS ||--o| BUSINESS_LOGIC : describes
    CODE_ELEMENTS ||--o| DOCUMENTS : referenced_by
    USER_STORIES ||--o{ BUSINESS_LOGIC : mapped_to
    FEATURES ||--o{ BUSINESS_LOGIC : implements

    CODE_ELEMENTS {
        uuid id PK
        string type
        string name
        string file_path
        int line_start
        int line_end
        string language
        uuid parent_id FK
        json metadata
        timestamp created_at
        timestamp updated_at
    }

    RELATIONSHIPS {
        uuid id PK
        uuid source_id FK
        uuid target_id FK
        string type
        json metadata
        timestamp created_at
    }

    BUSINESS_LOGIC {
        uuid id PK
        uuid element_id FK
        string description
        string user_story_id FK
        string feature_id FK
        timestamp created_at
        timestamp updated_at
    }

    DOCUMENTS {
        uuid id PK
        string title
        string content
        string file_path
        uuid generated_from FK
        timestamp last_updated
    }

    USER_STORIES {
        string id PK
        string title
        string description
        string status
    }

    FEATURES {
        string id PK
        string name
        string description
    }
```

### 4.2 Schema Description

| Table | Mô tả |
|-------|-------|
| CODE_ELEMENTS | Lưu trữ tất cả code elements (files, functions, classes, imports, exports) |
| RELATIONSHIPS | Quan hệ giữa các elements (imports, calls, implements, contains) |
| BUSINESS_LOGIC | Annotations mô tả business logic của từng element |
| DOCUMENTS | Generated documentation files |
| USER_STORIES | User stories được map với code |
| FEATURES | Features được map với code |

---

## 5. Interface Specifications

### 5.1 CLI Commands

| Command | Description |
|---------|-------------|
| `leankg init` | Initialize new LeanKG project |
| `leankg index [path]` | Index codebase |
| `leankg query <query>` | Query knowledge graph |
| `leankg generate docs` | Generate documentation |
| `leankg annotate` | Add business logic annotations |
| `leankg serve` | Start MCP server và/hoặc web UI |
| `leankg status` | Show index status |
| `leankg watch` | Start file watcher |

### 5.2 MCP Tools

| Tool | Description |
|------|-------------|
| `query_file` | Find file by name or pattern |
| `get_dependencies` | Get file dependencies |
| `get_dependents` | Get files depending on target |
| `find_function` | Locate function definition |
| `get_call_graph` | Get function call chain |
| `search_code` | Search code elements |
| `get_context` | Get AI context for file |
| `generate_doc` | Generate documentation for file |

### 5.3 Web UI Routes

| Route | Description |
|-------|-------------|
| `/` | Main dashboard |
| `/graph` | Interactive graph visualization |
| `/browse` | Code browser |
| `/docs` | Documentation viewer |
| `/annotate` | Business logic annotation |
| `/settings` | Configuration |

---

## 6. Security Considerations

### 6.1 Local Security

| Concern | Mitigation |
|---------|------------|
| Data at rest | Database file stored locally with optional encryption |
| MCP authentication | Local token-based authentication |
| File access | Sandboxed to project directory |

### 6.2 Network Security

| Concern | Mitigation |
|---------|------------|
| HTTP exposure | Bind to localhost only by default |
| MCP exposure | Local socket or localhost binding |
| External APIs | Optional, user-controlled |

---

## 7. Performance Targets

| Operation | Target | Notes |
|-----------|--------|-------|
| Cold start | < 2s | Binary initialization |
| Index speed | > 10K LOC/s | Per language parser |
| Query latency | < 100ms | Graph queries |
| Memory idle | < 100MB | No active operations |
| Memory peak | < 500MB | During indexing |
| Disk footprint | < 50MB/100K LOC | Database size |

---

## 8. Configuration

### 8.1 Project Configuration (leankg.yaml)

```yaml
project:
  name: my-project
  root: ./src
  languages:
    - go
    - typescript
    - python

indexer:
  exclude:
    - "**/node_modules/**"
    - "**/vendor/**"
    - "**/*.test.go"
  include:
    - "*.go"
    - "*.ts"
    - "*.py"

mcp:
  enabled: true
  port: 3000
  auth_token: generated

web:
  enabled: true
  port: 8080

documentation:
  output: ./docs
  templates:
    - agents
    - claude
```

---

## 9. Error Handling

### 9.1 Error Categories

| Category | Handling | User Feedback |
|----------|----------|---------------|
| Parse errors | Skip file, log warning | Warning in CLI output |
| Database errors | Retry with backoff | Error message |
| MCP errors | Return error response | MCP error payload |
| File system errors | Graceful degradation | Warning |

### 9.2 Logging

- **Level:** Configurable (debug, info, warn, error)
- **Output:** STDERR by default, file option available
- **Format:** Structured JSON for machine parsing, text for human

---

## 10. Future Considerations

### 10.1 Phase 2 Features

- Web UI improvements
- Business logic annotations
- Additional language support (Rust, Java, C#)
- Incremental indexing optimization

### 10.2 Phase 3 Features

- Vector embeddings cho semantic search
- Cloud sync option
- Team features (shared knowledge graphs)
- Plugin system

---

## 11. Dependencies

### 11.1 Direct Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| libSQL | latest | Embedded database |
| tree-sitter | latest | Code parsing |
| Cobra | latest | CLI framework |
| fsnotify | latest | File watching |

### 11.2 Build Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| Go | 1.21+ | Build toolchain |
| tree-sitter parsers | bundled | Language support |

---

## 12. Appendix

### 12.1 Glossary

| Term | Definition |
|------|------------|
| Container | Executable process in C4 model |
| Component | Internal module của container |
| Code element | File, function, class, import trong codebase |
| Context | Information provided to AI tool |
| Blast radius | Files affected by a change |

### 12.2 References

- C4 Model: https://c4model.com/
- libSQL: https://github.com/tursodatabase/libsql
- tree-sitter: https://tree-sitter.github.io/tree-sitter/
- MCP Protocol: https://modelcontextprotocol.io/
