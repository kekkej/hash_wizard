# 🔐 KeyGen CLI Utility

A flexible CLI tool written in **Rust** for generating cryptographic keys, hashing them using various tools (Rust, Java, OpenSSL, Swift, etc.), and exporting results for use in databases, vaults, or manual operations.


## 🚀 Features

* Key generation with secure random hex strings
* Configurable hashers via `hashers.toml`
* Multiple hashing backends (Rust, Java, OpenSSL, Swift, etc.)
* Multi-mode input support (`stdin`, `arg`, `file`)
* Output modes: console, SQL insert, or direct Vault write
* Optional hash verification across all hashers


## 📦 Adding a New Hasher

Edit `hashers.toml`:

```toml
[[hasher]]
name = "MyHasher"
type = "external"
command = "my_tool"
args = ["--hash"]
input_mode = "stdin"
algorithm = "sha256"
```

Available input modes:

* `stdin` — writes the key to stdin of your tool
* `arg` — passes the key as a command-line argument
* `file` — writes the key to a temp file and substitutes `{{INPUT_FILE}}` in args


## 🛠 Build Instructions

```bash
cargo build --release
```

Binary will be in:

```
target/release/keygen
```

Optional: build a statically linked binary for Linux:

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

## 📂 Bundle Package Example

After building:

```bash
tar -czf keygen_bundle.tar.gz target/release/keygen hashers.toml
```

To unpack and run:

```bash
tar -xzf keygen_bundle.tar.gz
./keygen --user alice --output sql
```


## ⚙️ CLI Arguments

```bash
--user <username>          # Required: username to assign the key
--output <console|sql|vault>  # Output mode (default: console)
--table <tablename>        # SQL table name (default: users)
--check                    # Compare hash results across all hashers
--config <file>            # Path to hashers.toml (default: hashers.toml)
```

Example:

```bash
./keygen --user alice --output sql --check
```


## 📊 Output Modes



---

