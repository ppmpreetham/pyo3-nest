# pyo3-nest

[![Crates.io](https://img.shields.io/crates/v/pyo3-nest.svg)](https://crates.io/crates/pyo3-nest)
[![Docs.rs](https://docs.rs/pyo3-nest/badge.svg)](https://docs.rs/pyo3-nest)

A clean and ergonomic macro DSL for creating **deeply nested submodules** in PyO3 with excellent developer experience.

Say goodbye to repetitive `PyModule::new` + `add_submodule` boilerplate with zero runtime overhead!

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pyo3 = { version = "0.28", features = ["extension-module"] }
pyo3-nest = "0.1"
```

Or use cargo:

```bash
cargo add pyo3-nest
```

## Usage

### Basic Example

```rust
use pyo3::prelude::*;
use pyo3_nest::{submodule, add_classes, add_functions};

#[pyclass]
struct Router;

#[pyclass]
struct Route;

#[pyfunction]
fn hello() -> &'static str {
    "Hello from Rust!"
}

#[pymodule]
fn my_extension(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // top level classes
    add_classes!(m, Router);

    // nested submodules
    submodule!(m, "routing", add_classes!(Router, Route));
    submodule!(m, "utils", add_functions!(hello));

    // classes + functions
    submodule!(m, "api.v1.responses", 
        add_classes!(JSONResponse, ErrorResponse) && 
        add_functions!(success_response, error_response)
    );

    Ok(())
}
```

### Python Usage

After building with `maturin`, users can import cleanly:

```python
from my_extension import Router
from my_extension.routing import Route
from my_extension.api.v1.responses import JSONResponse, success_response
```

## Why?

Before **pyo3-nest**:

```rust
let routing = PyModule::new(py, "routing")?;
routing.add_class::<Router>()?;
routing.add_class::<Route>()?;
m.add_submodule(&routing)?;
// ... and every now and then for every level + sys.modules management
```

After **pyo3-nest**:

```rust
submodule!(m, "routing", add_classes!(Router, Route));
```

Much cleaner, safer, and maintainable, especially as your project grows with many nested modules.

## Supported Patterns

- `submodule!(m, "foo", add_classes!(A, B))`
- `submodule!(m, "foo", add_functions!(fn1, fn2))`
- `submodule!(m, "foo", add_classes!(A, B) && add_functions!(fn1))`
- `submodule!(m, "foo", add_functions!(fn1) && add_classes!(A, B))`
- Multiple calls to the same path are safe

## Requirements

- Rust 1.70+
- PyO3 0.22+

## License

Licensed under MIT License


## Contributing

Contributions and feedback are welcome! Feel free to open issues or PRs.