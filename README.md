# Algvis Project

Algvis is an application focused on visualizations of different algorithms. The project aims to implement visualizations for algorithms such as sorting and maze running. Additionally, the application includes simple pseudocodes and explanations for these algorithms.

- **Tech Specs:**
  - [Rust programming language](https://www.rust-lang.org/)
  - [Yew frontend framework](https://yew.rs/)
  - [Tailwind CSS](https://tailwindcss.com/)

## Why

The Algvis project was created primarily for practicing Rust and testing its performance. It serves as a platform for practicing common algorithms and data structures. While not claiming to be the best algorithm visualization tool, Algvis is a version highly inspired by [Algorust](https://algorust.dev/).

---

## Getting Started

To get started with Algvis, follow these steps:

0. Install trunk https://trunkrs.dev/ and tailwindcss cli https://tailwindcss.com/docs/installation.
1. **install deps:**
   ```bash
   rustup target add wasm32-unknown-unknown
   cargo install wasm-bindgen-cli
2. **Clone repo and serve app using trunk:**
   ```bash
    git clone git@github.com:Vterebenin/algvis.git
    cd algvis
    trunk serve
