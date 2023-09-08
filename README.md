# Rust Distroless CI/CD Template

[![CI/CD Pipeline](https://github.com/athletedecoded/rust-distro-cicd/actions/workflows/CICD.yml/badge.svg)](https://github.com/athletedecoded/rust-distro-cicd/actions/workflows/CICD.yml)

Template for containerized Rust projects with automated CI/CD pipeline using Github Actions

On Push/PR:
* Format/lint/test code
* Lint Dockerfile # currently set to log errors without failing to allow for cc/distroless:latest
* Build and deploy distroless container to GHCR

