# CI/CD Data Science with Rust

[![CI/CD Pipeline](https://github.com/athletedecoded/rusty-ds/actions/workflows/CICD.yml/badge.svg)](https://github.com/athletedecoded/rusty-ds/actions/workflows/CICD.yml)

## Setup

```
# Install Rust
make install

# Install evcxr_jupyter
make evcxr

# Check versions
make toolchain
```

## Rust x Jupyter

1. Launch `./notebook.ipynb` >> Select Kernel >> Jupyter Kernel >> Rust
2. Run All Cells


## Rust CLI EDA Tool (.csv, .json files supported)

**Summary**
```
# If file includes headers
cargo run summary --path </path/to/data> --headers

# If file doesn't have headers
cargo run summary --path </path/to/data>
```

**Plot** -- WIP
```
cargo run plot --path </path/to/data> --headers --x <col_name> --y <col_name>
```

## Files

* `.devcontainer/` -- configures local development container environment
* `.github/workflows/CICD.yml` -- triggers CI/CD on git push and pull request
* `data/` -- sample data files for unit testing
* `src/lib.rs` -- shared library for `main.rs` and `notebook.ipynb`
* `src/main.rs` -- rusty-ds CLI script
* `cargo.toml` -- cargo dependencies
* `notebook.ipynb` -- Rust x Jupyter using EvCxR kernel
* `Makefile` -- build commands and utilities

## CI/CD

On git push/pull request the CI/CD flow is triggered using Github Actions:

1. Install and validate Rust toolchain
2. Format and lint code
3. Run unit tests
4. Build binary release
5. Lint Dockerfile
6. Build distroless rusty-ds image
7. Push image to [Github Container Registry](https://github.com/athletedecoded?tab=packages)

## Resources
* [EvCxR docs](https://github.com/evcxr/evcxr/tree/main/evcxr_jupyter)
* [EvCxR Jupyter Demo](https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/samples/evcxr_jupyter_tour.ipynb) 
* [Plotters x evcxr](https://github.com/plotters-rs/plotters#trying-with-jupyter-evcxr-kernel-interactively)
