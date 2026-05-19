<div align="center">
  <img 
    src="https://thumbs.dreamstime.com/b/adorable-rock-hyrax-portrait-happy-animal-closeup-charming-image-features-close-up-showcasing-its-endearing-expression-380168733.jpg"
    width="160" height="120" style="border-radius: 8px;" alt="Rock Hyrax" />
  <h1>HyraxQL</h1>
  <p><strong>“Awawa!”<br>- with Fur, Bullshit</br>A lightning-fast, lightweight database explorer, built in rust for the modern terminal.</strong></p>

  <p>
    <img src="https://img.shields.io/badge/Rust-2026-orange?logo=rust" alt="Rust Edition" />
    <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License" />
    <img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg" alt="PRs Welcome" />
  </p>
</div>

<hr />

## 🧬 What is HyraxQL?

HyraxQL is a minimalist database exploration tool built with **Rust**. It aims to provide a "middle ground" for developers who find full GUI clients (like DBeaver or PGAdmin) too heavy, but find raw `psql` or `mysql` shells too restrictive for quick data exploration.

Named after the Hyrax—a small, tough, and surprisingly agile animal—this tool is designed to be small in footprint but powerful in execution.

### Key Pillars
- **Performance**: Zero-cost abstractions and Rust's safety ensure your explorer never lags.
- **Simplicity**: No need for complex configuration files. Just point to a connection string and go.
- **Universal Interface**: One set of commands, regardless of whether you're on Postgres, MySQL, or SQLite.

---

## 🔌 Compatible With

- 🐘 **PostgreSQL** (including CockroachDB, TimescaleDB, and any other database that speaks the PostgreSQL wire protocol)
- 🐬 **MySQL & MariaDB** (including any database that speaks the MySQL wire protocol)
- 🪶 **SQLite** (file-based or local in-memory)

---

## 🏗 Technical Stack

HyraxQL is built on the shoulders of giants in the Rust ecosystem:
- [**SQLx**](https://github.com/launchbadge/sqlx): For asynchronous, compile-time verified (planned) database interactions.
- [**Clap**](https://github.com/clap-rs/clap): Powering the robust CLI argument parsing.
- [**Rustyline**](https://github.com/kkawakam/rustyline): Providing the Readline-like interactive shell experience.
- [**Tokio**](https://tokio.rs/): The asynchronous runtime driving the entire application.

---

## 🛠 Developer Setup

### Prerequisites
- [Rust](https://rustup.rs/) (v1.75+)
- [Cargo](https://doc.rust-lang.org/cargo/)

### Building from Source
```bash
# Clone the repository
git clone https://github.com/your-username/hyraxql.git
cd hyraxql

# Build for development
cargo build

# Run directly
cargo run -- connect --url "sqlite::memory:"
```

### Running Tests
```bash
cargo make test
```

---

## 🗺 Roadmap

- [x] Basic PostgreSQL/MySQL/SQLite support.
- [x] Interactive TUI loop.
- [ ] Native query execution and result formatting.
- [ ] Table schema visualization.
- [ ] Connection bookmarks/profiles.
- [ ] NoSQL and column-oriented databases adapters.
- [ ] Non-cli application.
- [ ] Automatically generated simple and editable ER.

Keep in mind this is a side project and might have no active development done on it for long intervals at a time.

---

## 📚 Documentation

- [**Usage Guide**](docs/docs.md): Detailed command reference, flags, and keyboard shortcuts.

---

## 🤝 Contributing

Contributions are welcome, but __*please anticipate long wait times for code reviews and feature integrations*__. I simply do not have the free time to commit to a fixed or timely review schedule as of right now.

To contribute:
1.  Check the [Issues](https://github.com/your-username/hyraxql/issues) for open tasks.
2.  Ensure your code passes `cargo fmt` and `cargo clippy`.
3.  Submit a PR with a clear description of your changes.

---

## 📄 License

HyraxQL is open-source software licensed under the **MIT License**.
