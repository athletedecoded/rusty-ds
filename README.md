# CI/CD Data Science with Rust

CLI and Notebook EDA using polars/plotters/evcxr + CI/CD distroless deployment

"Futureproofs" by testing build across rust releases: stable, beta, nightly

[![CI/CD Pipeline](https://github.com/athletedecoded/rusty-ds/actions/workflows/CICD.yml/badge.svg)](https://github.com/athletedecoded/rusty-ds/actions/workflows/CICD.yml)

**[DEMO](https://youtu.be/ZJXYvAEZFbM)**


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


## CLI EDA Tool

*Supported data formats: .csv, .json files*

**Summary**
```
# If file includes headers
cargo run summary --path </path/to/data> --headers
# ex. cargo run summary --path ./data/sample.csv --headers

# If file doesn't have headers
cargo run summary --path </path/to/data>
# ex. cargo run summary --path ./data/sample.json
```

**Plot**
```
cargo run plot --path </path/to/data> <--headers> --x <col_name> --y <col_name>
# ex. cargo run plot --path ./data/sample.csv --headers --x fats_g --y calories
# ex. cargo run plot --path ./data/sample.json --x fats_g --y calories
```

## Unit Tests

```
make test
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

1. Install and validate Rust toolchain for each of stable/beta/nightly release
2. Format and lint code
3. Run unit tests
4. Build binary release
5. Lint Dockerfile
6. Build distroless rusty-ds image
7. Push image to [Github Container Registry](https://github.com/athletedecoded?tab=packages)

NB: To build and push to GHCR, uncomment section in `.github/workflows/CICD.yml`


## ToDos
- [ ] Add error handling for column name DNE
- [x] Add dynamic plot bounds

## Resources
* [EvCxR docs](https://github.com/evcxr/evcxr/tree/main/evcxr_jupyter)
* [EvCxR Jupyter Demo](https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/samples/evcxr_jupyter_tour.ipynb) 
* [Plotters x evcxr](https://github.com/plotters-rs/plotters#trying-with-jupyter-evcxr-kernel-interactively)
* [Plotters Dev Guide](https://plotters-rs.github.io/book/intro/introduction.html)
