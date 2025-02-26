# tembodb

mashariki_db/
├── Cargo.toml                   # Rust project metadata and dependencies
├── src/                         # Rust source code
│   ├── main.rs                  # Application entry point
│   ├── lib.rs                   # Core library exports
│   ├── config.rs                # Configuration settings
│   ├── parser/                 # SQL parsing (lexer, parser, AST)
│   ├── planner/                # Query planning and optimization
│   ├── executor/               # Query execution engine
│   ├── storage/                # Data persistence (tables, indexes, WAL)
│   ├── buffer/                 # Buffer management and caching
│   ├── transaction/            # Transaction and concurrency control
│   ├── access_control/         # Authentication and security
│   ├── network/                # Networking (wire protocol, connection handling)
│   └── cli/                    # Command-line interface (REPL)
│
├── go_modules/                  # Golang modules for specific tasks or integration
│   ├── go.mod                   # Go module file defining dependencies
│   ├── pkg/                     # Reusable Go packages
│   │   ├── protocol/            # Implementation of protocol-related functionality
│   │   │   └── protocol.go      
│   │   ├── connection/          # Client connection management in Go
│   │   │   └── connection.go    
│   │   └── utils/               # Utility functions shared across Go modules
│   │       └── helper.go
│   └── cmd/                     # Go command-line applications or microservices
│       ├── go_server/           # A Go-based server component (if needed)
│       │   └── main.go          
│       └── another_tool/        # Other Go-based tools integrated into the project
│           └── main.go
│
└── tests/                       # Tests for both Rust and Go components
    ├── rust_tests/              # Unit and integration tests for Rust code
    └── go_tests/                # Tests for Go modules (using Go’s testing framework)

Key Considerations
Modular Design:
Each component (parsing, planning, execution, etc.) is isolated in its own module, making the system easier to develop, test, and maintain.

Rust Benefits:
Rust’s strong guarantees around memory safety and concurrency are ideal for building a high-performance, reliable database.

Layered Architecture:
This structure mirrors the internal layers of PostgreSQL, including the query processing layers, storage engine, transaction management, and network interface.

Scalability & Extensibility:
With separate modules for each functionality, you can easily extend or replace parts (like swapping out the storage engine or optimizing the planner) without affecting the entire system.

Testing:
A dedicated tests directory ensures that each module is thoroughly tested, which is critical for database systems where reliability is key.
