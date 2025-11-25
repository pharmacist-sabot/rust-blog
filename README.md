# Rust Variables: Interactive Blog 🦀

![Build Status](https://github.com/pharmacist-sabot/rust-blog/actions/workflows/deploy.yml/badge.svg)
![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)
![Leptos](https://img.shields.io/badge/framework-Leptos-red)
![Tailwind](https://img.shields.io/badge/styling-Tailwind_CSS-38bdf8)

A modern, interactive blog post demonstrating **Rust Variables and Mutability**.
This project is not just *about* Rust—it is **built with Rust**, running entirely in the browser using WebAssembly (Wasm) via the **Leptos** framework.

[**🔴 View Live Demo**](https://pharmacist-sabot.github.io/rust-blog/)

## 📖 About The Project

This application serves as an interactive educational material (in Thai) to explain the core concepts of Rust's memory safety model:
*   **Immutability by default:** Why Rust variables cannot be changed without permission.
*   **The `mut` keyword:** How to explicitly opt-in to mutability.
*   **Interactive State:** Demonstrates a real Rust Signal counter running in the browser to prove Wasm execution.

## 🛠️ Tech Stack

*   **Language:** [Rust](https://www.rust-lang.org/) (Edition 2024)
*   **Framework:** [Leptos](https://leptos.dev/) (Client-Side Rendering / CSR)
*   **Build Tool:** [Trunk](https://trunkrs.dev/)
*   **Styling:** [Tailwind CSS](https://tailwindcss.com/) (via CDN for simplicity)
*   **Deployment:** GitHub Pages (Automated via GitHub Actions)

## 🚀 Getting Started

To run this project locally, you need to have Rust installed on your machine.

### Prerequisites

1.  **Install Rust**:
    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2.  **Add Wasm Target**:
    ```sh
    rustup target add wasm32-unknown-unknown
    ```

3.  **Install Trunk** (Wasm web bundler):
    ```sh
    cargo install trunk
    ```

### Running Locally

1.  **Clone the repository**
    ```sh
    git clone https://github.com/pharmacist-sabot/rust-blog.git
    cd rust-blog
    ```

2.  **Start the development server**
    ```sh
    trunk serve
    ```

3.  **Open in Browser**
    Go to `http://localhost:8080`. The app will automatically reload when you modify the code.

## 📂 Project Structure

```text
rust-blog/
├── Cargo.toml      # Dependencies (Leptos, etc.)
├── index.html      # Entry point & Tailwind configuration
├── src/
│   └── main.rs     # Application logic & UI Components
└── .github/
    └── workflows/  # CI/CD pipeline for GitHub Pages
```

## 📦 Deployment

This repository uses **GitHub Actions** to automatically build and deploy to GitHub Pages.
The workflow is defined in `.github/workflows/deploy.yml`.

1.  Triggers on push to `main`.
2.  Sets up Rust and Trunk.
3.  Builds the project in release mode (`trunk build --release`).
4.  Deploys the `dist` artifact to GitHub Pages.

## 📝 License

Distributed under the MIT License.

---
