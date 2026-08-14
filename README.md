<div align="center">

# envdoctor

**[ English ](#envdoctor-english-version) | [ 日本語 (Japanese) ](#envdoctor-japanese-version--日本語版)**

[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey.svg)]()

</div>

---

# envdoctor (English Version)

Automated developer environment diagnostic and health check tool that pinpoints root causes and provides actionable fix commands.

## Table of Contents
- [What is envdoctor?](#what-is-envdoctor)
- [Key Features](#key-features)
- [Supported Environments & Tools](#supported-environments--tools)
- [Supported Platforms & Architectures](#supported-platforms--architectures)
- [Installation & Quickstart](#installation--quickstart)
- [Usage & Commands](#usage--commands)
- [Output Formats](#output-formats)
- [Project Structure](#project-structure)
- [Running Tests](#running-tests)
- [Feedback & Contact](#feedback--contact)
- [License](#license)

---

## What is envdoctor?

`envdoctor` (`envdr`) scans your programming language toolchains, development tools, and operating system configuration to diagnose common environment issues, identify root causes, and propose exact commands to resolve them.

```text
Scan Environment ──▶ Detect Conflicts & Errors ──▶ Identify Root Cause ──▶ Actionable Fixes (Dry-Run / Apply)
```

Common issues detected:
- Python and Pip version mismatch (e.g., `pip` points to a different Python installation).
- Non-existent directories, duplicate entries, or unsafe overrides in `PATH`.
- Outdated system package manager caches (`apt`, `brew`, `pacman`, `dnf`).
- Docker socket permission errors (user not in `docker` group) or stopped daemon.
- Missing Git author identity (`user.name` / `user.email`) or commit signing keys.
- Machine learning framework GPU detection issues (PyTorch CPU fallback, ROCm `/dev/kfd` permissions).

---

## Key Features

- **Fast & Zero Dependencies**: Compiled as a standalone native Rust binary (~1MB). Operates even when Python or Node.js runtimes on the host are broken.
- **Safe by Default**: Diagnostic scans are strictly read-only. Fix features (`fix`) run in preview (dry-run) mode by default unless `--apply` is specified.
- **Dynamic OS & Package Manager Integration**: Automatically identifies `apt`, `brew`, `dnf`, `pacman`, `apk`, etc., and suggests tailored installation/update commands.
- **Multi-Architecture Support**: Native support for Apple Silicon (M1/M2/M3/M4), Intel macOS, Linux x86_64, and Linux ARM64 (AWS Graviton, Raspberry Pi).
- **CI/CD & Report Ready**: Standard terminal output, structured JSON (`--format json`), and Markdown (`--format markdown`) for GitHub Issues and PRs.
- **Automatic Privacy Masking**: Sensitive keys, authentication tokens, and passwords in environment variables are automatically masked.

---

## Supported Environments & Tools

| Category | Flag / Command | Diagnostic Scope |
| :--- | :--- | :--- |
| **System** | `system`, `--system` | OS, kernel, CPU, memory, PATH validity, package manager cache freshness |
| **Shell** | `shell`, `--shell` | Login shell, `~/.bashrc` / `~/.zshrc` syntax, duplicated PATH exports |
| **Python** | `python`, `--python` | Python 3, pip alignment, venv/conda, uv/poetry, PyTorch GPU acceleration |
| **Rust** | `rust`, `--rust` | rustc, cargo, rustup, toolchains, `~/.cargo/bin` in PATH |
| **Go** | `go`, `--go` | go CLI, gofmt, GOPATH/GOROOT, `$GOPATH/bin` in PATH, golangci-lint |
| **Node.js** | `node`, `--node` | node, npm, yarn, pnpm, bun, deno, fnm/nvm version managers |
| **Java / JVM** | `java`, `--java` | java, javac, JAVA_HOME, Maven, Gradle, kotlinc, SDKMAN |
| **Ruby** | `ruby`, `--ruby` | ruby, gem, bundler, rbenv, rvm, asdf |
| **PHP** | `php`, `--php` | php CLI, composer |
| **C# / .NET** | `dotnet`, `--dotnet` | dotnet CLI, SDK list, runtime list, `DOTNET_ROOT` |
| **C / C++** | `cpp`, `--cpp` | gcc, g++, clang, linkers (`ld`, `lld`, `mold`), make, cmake |
| **Swift** | `swift`, `--swift` | swift, swiftc |
| **Dart & Flutter** | `dart`, `--dart` | dart SDK, flutter SDK |
| **Zig** | `zig`, `--zig` | zig compiler |
| **Lua** | `lua`, `--lua` | lua, luajit, luarocks |
| **Git** | `git`, `--git` | git CLI, `user.name` / `user.email`, SSH keys, commit signing |
| **Docker** | `docker`, `--docker` | Docker CLI, daemon socket access, Docker Compose, GPU passthrough |
| **GPU** | `gpu`, `--gpu` | NVIDIA GPU (`nvidia-smi`, CUDA), AMD GPU (`rocm-smi`, `/dev/kfd`) |

---

## Supported Platforms & Architectures

| OS | Architecture | Recommended Installation Method |
| :--- | :--- | :--- |
| **macOS (Apple Silicon)** | Apple M1 / M2 / M3 / M4 (`arm64`) | `brew install sha256san/tap/envdoctor` or Automated Installer |
| **macOS (Intel)** | Intel 64-bit (`x86_64`) | `brew install sha256san/tap/envdoctor` or Automated Installer |
| **Linux (Ubuntu / Debian)** | Intel/AMD (`amd64`), ARM64 (`arm64`) | `sudo apt install envdoctor` or Automated Installer |
| **Linux (Generic / RHEL / Arch)** | `x86_64`, `aarch64` (Graviton/Raspberry Pi) | Automated Installer or GitHub Releases binary |
| **Windows** | 64-bit (`x86_64`) | `cargo install` or Releases binary |

---

## Installation & Quickstart

### 1. Automated One-Line Installer (Linux & macOS)
Automatically detects your OS and architecture:

```bash
curl -fsSL https://raw.githubusercontent.com/sha256san/envdr/main/install.sh | sudo bash
```

### 2. macOS (Homebrew Tap)

```bash
brew install sha256san/tap/envdoctor
# or the short alias
brew install sha256san/tap/envdr
```

### 3. Ubuntu / Debian (APT Package)

```bash
# Register APT repository
echo "deb [trusted=yes] https://raw.githubusercontent.com/sha256san/envdr/main/docs/apt stable main" | sudo tee /etc/apt/sources.list.d/envdoctor.list
sudo apt update

# Install envdoctor
sudo apt install -y envdoctor
```

### 4. Build from Source

```bash
git clone https://github.com/sha256san/envdr.git
cd envdr
cargo build --release
sudo cp target/release/envdr /usr/local/bin/
```

---

## Usage & Commands

### 1. Complete Environment Health Check
Scans the system. Uninstalled programming languages are automatically hidden to keep output concise:

```bash
envdr
# or
envdoctor
```

### 2. Targeted Diagnostics via Direct Flags
Quickly diagnose specific languages or tools:

```bash
# Diagnose Python only
envdr --python

# Diagnose Go and Rust simultaneously
envdr --go --rust

# Diagnose Docker and GPU environments
envdr --docker --gpu

# Diagnose all installed languages / specific language list
envdr -l                     # all installed languages
envdr --language python,go   # specific languages

# Diagnose all developer tools / specific tool list
envdr -t                     # all tools
envdr --tool docker,gpu      # specific tools
```

### 3. Subcommand Checks (`check`)
```bash
envdr check python
envdr check rust
envdr check docker
envdr check git
```

### 4. Safe Auto-Fix (`fix`)
View and apply recommended fixes:

```bash
# Preview proposed fix plan (Dry-run mode: no changes are made)
envdr fix

# Preview fix plan for a specific target
envdr fix git

# Apply proposed fixes
envdr fix --apply
```

---

## Output Formats

### Terminal Output (Default)
```text
  OK   System & Environment
   ✔ OS & Architecture (Ubuntu 24.04 (x86_64))
   ✔ Hardware Resources
   ▲ PATH Environment Variable
     • Total entries in PATH: 39
     • Non-existent path: /snap/bin
     Warning: Found 1 non-existent path(s) in PATH
       Cause: Directories specified in PATH do not exist on the filesystem
       Impact: Commands installed in these directories may fail to execute
     💡 Recommendation: Remove non-existent directories from your PATH in ~/.bashrc or ~/.zshrc
        Verify: $ echo $PATH
   ✔ Package Manager & Cache (APT package cache is up-to-date)

  OK   Shell & Configuration
   ✔ Login Shell (GNU bash, version 5.2.21)
   ✔ Configuration: ~/.bashrc

 FAIL  Docker & Containerization
   ✔ Docker CLI (Docker version 27.1.1)
   ✖ Docker Daemon & Permissions
     Error: Current user does not have permission to communicate with Docker daemon socket (/var/run/docker.sock)
       Cause: The active user does not belong to the 'docker' group
       Impact: Docker CLI commands cannot manage containers without root / sudo privileges
     💡 Recommendation: Add current user to 'docker' group and activate group membership
        $ sudo usermod -aG docker $USER && newgrp docker
        Verify: $ docker ps

 ────────────────────────────────────────────────────────────
 Summary
   ✔ 18 checks passed
   ▲ 1 warnings
   ✖ 1 errors
   ℹ 2 informational

 Issues:
   1. [WARN] System & Environment: Found 1 non-existent path(s) in PATH
      Impact: Commands installed in these directories may fail to execute
   2. [ERROR] Docker & Containerization: Current user does not have permission to communicate with Docker daemon socket (/var/run/docker.sock)
      Impact: Docker CLI commands cannot manage containers without root / sudo privileges
      Fix: $ sudo usermod -aG docker $USER && newgrp docker

 Run `envdr fix` to preview or apply available fixes.
 Run `envdr --help` for more options.
```

### JSON Output (`--format json`)
Ideal for CI/CD pipelines and automated auditing:

```bash
envdoctor doctor --format json
```

### Markdown Output (`--format markdown`)
Generate reports ready to paste into GitHub Issues and Pull Requests:

```bash
envdoctor doctor --format markdown
# Save report directly to file
envdoctor doctor --format markdown -o report.md
```

---

## Project Structure

```text
envdoctor/
├── Cargo.toml                  # Package configuration & dependencies
├── Cargo.lock                  # Dependency lockfile
├── CHANGELOG.md                # Release notes & version history
├── LICENSE                     # MIT License
├── README.md                   # Project documentation (English & Japanese)
├── install.sh                  # Multi-platform automated installer (Linux & macOS)
├── Dockerfile                  # Container environment build & testing
├── .dockerignore               # Docker build context optimization
├── docs/                       # Documentation and APT repository hosting
│   └── apt/                    # Static APT repository (dists, pool)
├── scripts/                    # Automation and packaging scripts
│   ├── package.sh              # Multi-architecture release packager (.tar.gz & .deb)
│   ├── test-docker.sh          # Container integration test suite
│   └── create-apt-repo.sh      # APT repository generation script
└── src/
    ├── main.rs                 # envdoctor CLI primary binary entrypoint
    ├── lib.rs                  # Core diagnostic orchestration library
    ├── bin/
    │   └── envdr.rs            # envdr short-alias binary entrypoint
    ├── cli/
    │   └── mod.rs              # Command line argument & flag parsing (clap)
    ├── core/
    │   ├── mod.rs              # Diagnostic types, severity, and status models
    │   └── fixer.rs            # AutoFixer action planning and safe execution
    ├── system/
    │   ├── mod.rs              # Hardware and operating system inspection
    │   └── package_manager.rs  # Package manager detection & cache freshness
    ├── checkers/               # Diagnostic checker implementations
    │   ├── mod.rs              # CheckerRegistry and target routing
    │   ├── system.rs           # System and hardware diagnostic checker
    │   ├── shell.rs            # Shell configuration file checker (.bashrc, .zshrc)
    │   ├── python.rs           # Python & virtual environment checker
    │   ├── rust.rs             # Rust toolchain checker
    │   ├── go.rs               # Go toolchain & GOPATH checker
    │   ├── node.rs             # Node.js, npm, yarn, pnpm checker
    │   ├── cpp.rs              # C/C++ compiler and build system checker
    │   ├── java.rs             # Java, JDK, Maven, Gradle checker
    │   ├── ruby.rs             # Ruby, gem, bundler checker
    │   ├── php.rs              # PHP & Composer checker
    │   ├── dotnet.rs           # .NET SDK & runtime checker
    │   ├── swift.rs            # Swift toolchain checker
    │   ├── dart.rs             # Dart & Flutter SDK checker
    │   ├── zig.rs              # Zig compiler checker
    │   ├── lua.rs              # Lua & Luarocks checker
    │   ├── generic.rs          # Declarative language checker builder
    │   ├── git.rs              # Git configuration & SSH key checker
    │   ├── docker.rs           # Docker CLI, daemon, and compose checker
    │   └── gpu.rs              # GPU (NVIDIA CUDA / AMD ROCm) checker
    ├── output/                 # Report formatters
    │   ├── mod.rs
    │   ├── terminal.rs         # Rich terminal output formatter
    │   ├── json.rs             # Structured JSON formatter
    │   └── markdown.rs         # Markdown report formatter
    └── utils/                  # Shared helper modules
        ├── mod.rs
        ├── command.rs          # Safe command execution helper
        ├── path.rs             # PATH analysis, symlink & shim verification
        └── mask.rs             # Sensitive credential masking filter
```

---

## Running Tests

```bash
cargo test
```

To run integration tests inside Docker:
```bash
./scripts/test-docker.sh
```

---

## Feedback & Contact

For bug reports, feature requests, or suggestions for new language/tool checkers, please reach out via:

- **Issue Tracker / Feature Requests**: [GitHub Issues](https://github.com/sha256san/envdr/issues)
- **Discussions & Ideas**: [GitHub Discussions](https://github.com/sha256san/envdr/discussions)
- **Email Contact**: [contact@example.com](mailto:sha256san@gmail.com)
- **Repository**: [https://github.com/sha256san/envdr](https://github.com/sha256san/envdr)

---

## License

This project is licensed under the [MIT License](LICENSE).

---

<br>

---

# envdoctor (Japanese Version / 日本語版)

**[ English Version に戻る ](#envdoctor-english-version)**

開発環境の「動かない」「バージョンが合わない」「PATHが通っていない」などの問題を自動診断し、根本原因と解決コマンドを提示するクロスプラットフォームCLIツールです。

## 目次
- [What is envdoctor? (概要)](#what-is-envdoctor-概要)
- [主な特徴](#主な特徴)
- [診断対象ツール & 環境](#診断対象ツール--環境-1)
- [対応プラットフォーム & アーキテクチャ](#対応プラットフォーム--アーキテクチャ)
- [インストール & クイックスタート](#インストール--クイックスタート-1)
- [使い方 & コマンド](#使い方--コマンド-1)
- [出力フォーマット](#出力フォーマット)
- [プロジェクト構成](#プロジェクト構成-1)
- [テストの実行](#テストの実行-1)
- [ご意見・ご要望・連絡先](#ご意見ご要望連絡先)
- [ライセンス](#ライセンス-1)

---

## What is envdoctor? (概要)

`envdoctor` (`envdr`) は、Python、Rust、C/C++、Node.js、Docker、Git、GPU（CUDA/ROCm）などの開発環境を検査し、**問題の発見・原因の推定・具体的な解決コマンドの提示** までをワンストップで行う診断ツールです。

```text
環境スキャン ──▶ 異常・競合検出 ──▶ 原因推定 ──▶ 解決コマンド提示 (Dry-Run / Apply)
```

検出する主な環境問題：
- `python3` と `pip` のバージョン不一致（別バージョンの Python パスを参照している）。
- `PATH` 内の存在しないディレクトリ、重複パス、危険な上書き構文。
- パッケージマネージャーのキャッシュ期限切れ（`apt`, `brew`, `pacman`, `dnf`）。
- Docker デーモンのソケット権限不足（ユーザーが `docker` グループ未所属）やデーモン停止。
- Git コミットの署名設定・ユーザー情報未設定。
- 機械学習フレームワークの GPU 認識不全（PyTorch の CPU フォールバック、ROCm `/dev/kfd` 権限）。

---

## 主な特徴

- **超高速・依存関係ゼロ**: Rust 製シングルバイナリ（~1MB）。Python や Node.js の壊れた環境でも単体動作。
- **安全第一（Safe by Default）**: 診断処理は完全リードオンリー。修復機能（`fix`）はデフォルトで dry-run（実行予定コマンドのプレビュー表示）。
- **OS・パッケージマネージャー動的対応**: `apt`, `dnf`, `pacman`, `brew`, `winget` などを自動判定し、環境に最適なコマンドを提示。
- **マルチアーキテクチャ対応**: macOS Apple Silicon (M1/M2/M3/M4)、Intel Mac、Linux x86_64、Linux ARM64 (AWS Graviton, Raspberry Pi) に対応。
- **CI/CD & レポート連携**: 構造化 `JSON` 出力および GitHub Issue/PR にそのまま貼れる `Markdown` 出力を標準サポート。
- **プライバシー保護**: トークン、パスワード、API キーなどの機密情報を自動マスキング。

---

## 診断対象ツール & 環境

| カテゴリ | コマンド引数 / フラグ | 検査項目 |
| :--- | :--- | :--- |
| **System** | `system`, `--system` | OS、カーネル、CPU/メモリ、`PATH` の重複・壊れたパス、パッケージマネージャー鮮度 |
| **Shell** | `shell`, `--shell` | ログインシェル、`~/.bashrc` / `~/.zshrc` の構文・PATH 破損検知 |
| **Python** | `python`, `--python` | Python 3, pip 整合性, venv/conda, uv/poetry, PyTorch GPU 認識 |
| **Rust** | `rust`, `--rust` | rustc, cargo, rustup, ツールチェーン, `~/.cargo/bin` PATH |
| **Go** | `go`, `--go` | go CLI, gofmt, GOPATH/GOROOT, `$GOPATH/bin` PATH, golangci-lint |
| **Node.js** | `node`, `--node` | node, npm, yarn, pnpm, bun, deno, fnm/nvm バージョンマネージャー |
| **Java / JVM** | `java`, `--java` | java, javac, JAVA_HOME, Maven, Gradle, kotlinc, SDKMAN |
| **Ruby** | `ruby`, `--ruby` | ruby, gem, bundler, rbenv, rvm, asdf |
| **PHP** | `php`, `--php` | php CLI, composer |
| **C# / .NET** | `dotnet`, `--dotnet` | dotnet CLI, SDK一覧, Runtime一覧, `DOTNET_ROOT` |
| **C/C++** | `cpp`, `--cpp` | gcc, g++, clang, リンカー (`ld`/`lld`/`mold`), make, cmake |
| **Swift** | `swift`, `--swift` | swift, swiftc |
| **Dart & Flutter** | `dart`, `--dart` | dart SDK, flutter SDK |
| **Zig** | `zig`, `--zig` | zig コンパイラ |
| **Lua** | `lua`, `--lua` | lua, luajit, luarocks |
| **Git** | `git`, `--git` | git CLI, `user.name`/`user.email` 設定, SSH 鍵, コミット署名 |
| **Docker** | `docker`, `--docker` | Docker CLI, デーモン接続, ソケット権限, Docker Compose, GPU パススルー |
| **GPU** | `gpu`, `--gpu` | NVIDIA GPU (`nvidia-smi`, CUDA), AMD GPU (`rocm-smi`, `/dev/kfd`) |

---

## 対応プラットフォーム & アーキテクチャ

| OS | アーキテクチャ | 推奨インストール方法 |
| :--- | :--- | :--- |
| **macOS (Apple Silicon)** | Apple M1 / M2 / M3 / M4 (`arm64`) | `brew install sha256san/tap/envdoctor` またはワンライナー |
| **macOS (Intel)** | Intel 64-bit (`x86_64`) | `brew install sha256san/tap/envdoctor` またはワンライナー |
| **Linux (Ubuntu / Debian)** | Intel/AMD (`amd64`), ARM64 (`arm64`) | `sudo apt install envdoctor` またはワンライナー |
| **Linux (汎用 / RHEL / Arch / Alpine)** | `x86_64`, `aarch64` (Graviton/Raspberry Pi) | ワンライナー または GitHub Releases バイナリ |
| **Windows** | 64-bit (`x86_64`) | `cargo install` または Releases バイナリ |

---

## インストール & クイックスタート

### 1. ワンライナーで自動インストール (OS & アーキテクチャ自動判別)

Linux (x86_64 / ARM64) および macOS (Apple Silicon / Intel) の両方に対応しています：

```bash
curl -fsSL https://raw.githubusercontent.com/sha256san/envdr/main/install.sh | sudo bash
```

### 2. macOS (Homebrew Tap)

```bash
brew install sha256san/tap/envdoctor
# または短縮名
brew install sha256san/tap/envdr
```

### 3. Ubuntu / Debian (APT パッケージ)

```bash
# APT リポジトリの登録
echo "deb [trusted=yes] https://raw.githubusercontent.com/sha256san/envdr/main/docs/apt stable main" | sudo tee /etc/apt/sources.list.d/envdoctor.list
sudo apt update

# envdoctor (または envdr) をインストール
sudo apt install -y envdoctor
```

### 4. ソースからビルドして実行

```bash
git clone https://github.com/sha256san/envdr.git
cd envdr
cargo build --release
sudo cp target/release/envdr /usr/local/bin/
```

---

## 使い方 & コマンド

### 1. 全体診断 (未インストール言語は自動非表示)
システム全体を包括的にスキャンします。開発環境にインストールされている言語のみがすっきり表示されます：

```bash
envdr
# または
envdoctor
```

### 2. 言語・ツールのダイレクト診断 (フラグ指定)
チェックしたい言語やツールを直接フラグで指定して素早く診断できます：

```bash
# Python のみ診断
envdr --python

# Go と Rust を同時に診断
envdr --go --rust

# Docker と GPU を診断
envdr --docker --gpu

# 言語環境を一括診断 / 指定言語を診断
envdr -l                     # インストール済み全言語
envdr --language python,go   # Python と Go を指定診断

# 開発ツールを一括診断 / 指定ツールを診断
envdr -t                     # 全開発ツール
envdr --tool docker,gpu      # Docker と GPU を指定診断
```

### 3. 特定環境の個別診断 (`check`)
```bash
envdr check python
envdr check rust
envdr check docker
envdr check git
```

### 4. 安全な自動修復 (`fix`)
検出された問題に対する解決策を表示・適用します：

```bash
# 修復プランをプレビュー (Dry-run: 変更は加えません)
envdr fix

# 特定カテゴリのみ修復プランを表示
envdr fix git

# 実際に修復コマンドを実行して適用
envdr fix --apply
```

---

## 出力フォーマット

### ターミナル表示 (デフォルト)
```text
  OK   System & Environment
   ✔ OS & Architecture (Ubuntu 24.04 (x86_64))
   ✔ Hardware Resources
   ▲ PATH Environment Variable
     • Total entries in PATH: 39
     • Non-existent path: /snap/bin
     Warning: Found 1 non-existent path(s) in PATH
       Cause: Directories specified in PATH do not exist on the filesystem
       Impact: Commands installed in these directories may fail to execute
     💡 Recommendation: Remove non-existent directories from your PATH in ~/.bashrc or ~/.zshrc
        Verify: $ echo $PATH
   ✔ Package Manager & Cache (APT package cache is up-to-date)

  OK   Shell & Configuration
   ✔ Login Shell (GNU bash, version 5.2.21)
   ✔ Configuration: ~/.bashrc

 FAIL  Docker & Containerization
   ✔ Docker CLI (Docker version 27.1.1)
   ✖ Docker Daemon & Permissions
     Error: Current user does not have permission to communicate with Docker daemon socket (/var/run/docker.sock)
       Cause: The active user does not belong to the 'docker' group
       Impact: Docker CLI commands cannot manage containers without root / sudo privileges
     💡 Recommendation: Add current user to 'docker' group and activate group membership
        $ sudo usermod -aG docker $USER && newgrp docker
        Verify: $ docker ps

 ────────────────────────────────────────────────────────────
 Summary
   ✔ 18 checks passed
   ▲ 1 warnings
   ✖ 1 errors
   ℹ 2 informational

 Issues:
   1. [WARN] System & Environment: Found 1 non-existent path(s) in PATH
      Impact: Commands installed in these directories may fail to execute
   2. [ERROR] Docker & Containerization: Current user does not have permission to communicate with Docker daemon socket (/var/run/docker.sock)
      Impact: Docker CLI commands cannot manage containers without root / sudo privileges
      Fix: $ sudo usermod -aG docker $USER && newgrp docker

 Run `envdr fix` to preview or apply available fixes.
 Run `envdr --help` for more options.
```

### JSON 出力 (`--format json`)
CI/CD パイプラインでの自動検証やメトリクス収集に適しています：

```bash
envdoctor doctor --format json
```

### Markdown 出力 (`--format markdown`)
GitHub Issue や Pull Request に貼り付けて環境共有できます：

```bash
envdoctor doctor --format markdown
# ファイルに保存する場合
envdoctor doctor --format markdown -o report.md
```

---

## プロジェクト構成

```text
envdoctor/
├── Cargo.toml                  # パッケージ設定 & 依存関係
├── Cargo.lock                  # 依存関係ロックファイル
├── CHANGELOG.md                # リリース変更履歴
├── LICENSE                     # MIT ライセンス
├── README.md                   # プロジェクトドキュメント (日英バイリンガル)
├── install.sh                  # マルチプラットフォーム自動インストーラー (Linux / macOS)
├── Dockerfile                  # コンテナ環境テスト・検証設定
├── .dockerignore               # Docker ビルドコンテキスト最適化
├── docs/                       # ドキュメント & APT リポジトリホスティング
│   └── apt/                    # 静的 APT リポジトリ (dists, pool)
├── scripts/                    # 自動化・パッケージングスクリプト
│   ├── package.sh              # マルチアーキテクチャパッケージャー (.tar.gz & .deb)
│   ├── test-docker.sh          # コンテナ統合自動テストスイート
│   └── create-apt-repo.sh      # APT リポジトリ自動生成スクリプト
└── src/
    ├── main.rs                 # envdoctor CLI メインバイナリエントリポイント
    ├── lib.rs                  # コア診断エンジンオーケストレーションライブラリ
    ├── bin/
    │   └── envdr.rs            # envdr 短縮コマンドエントリポイント
    ├── cli/
    │   └── mod.rs              # コマンドライン引数・フラグ定義 (clap)
    ├── core/
    │   ├── mod.rs              # 診断モデル、ステータス、重要度定義
    │   └── fixer.rs            # AutoFixer 自動修復プラン生成 & 実行
    ├── system/
    │   ├── mod.rs              # OS・ハードウェアリソース検査
    │   └── package_manager.rs  # パッケージマネージャー判定 & キャッシュ鮮度診断
    ├── checkers/               # 各種診断モジュール実装
    │   ├── mod.rs              # CheckerRegistry & 診断ターゲット振り分け
    │   ├── system.rs           # システム環境・ハードウェア診断チェッカー
    │   ├── shell.rs            # シェル設定ファイル診断 (.bashrc, .zshrc)
    │   ├── python.rs           # Python & 仮想環境チェッカー
    │   ├── rust.rs             # Rust ツールチェーンチェッカー
    │   ├── go.rs               # Go ツールチェーン & GOPATH チェッカー
    │   ├── node.rs             # Node.js, npm, yarn, pnpm チェッカー
    │   ├── cpp.rs              # C/C++ コンパイラ & ビルドシステムチェッカー
    │   ├── java.rs             # Java, JDK, Maven, Gradle チェッカー
    │   ├── ruby.rs             # Ruby, gem, bundler チェッカー
    │   ├── php.rs              # PHP & Composer チェッカー
    │   ├── dotnet.rs           # .NET SDK & ランタイムチェッカー
    │   ├── swift.rs            # Swift ツールチェーンチェッカー
    │   ├── dart.rs             # Dart & Flutter SDK チェッカー
    │   ├── zig.rs              # Zig コンパイラチェッカー
    │   ├── lua.rs              # Lua & Luarocks チェッカー
    │   ├── generic.rs          # 宣言的汎用言語チェッカービルダー
    │   ├── git.rs              # Git 設定 & SSH 鍵チェッカー
    │   ├── docker.rs           # Docker CLI, デーモン, Compose チェッカー
    │   └── gpu.rs              # GPU (NVIDIA CUDA / AMD ROCm) チェッカー
    ├── output/                 # 出力フォーマッター
    │   ├── mod.rs
    │   ├── terminal.rs         # ターミナル用カラー出力
    │   ├── json.rs             # 構造化 JSON 出力
    │   └── markdown.rs         # Markdown レポート出力
    └── utils/                  # 共通ユーティリティ
        ├── mod.rs
        ├── command.rs          # 安全な外部コマンド実行
        ├── path.rs             # PATH 解析・シンボリックリンク/シム検証
        └── mask.rs             # 機密情報 (トークン/APIキー) マスキング
```

---

## テストの実行

```bash
cargo test
```

Dockerコンテナ内でのテスト実行：
```bash
./scripts/test-docker.sh
```

---

## ご意見・ご要望・連絡先

機能の追加要望、新しい言語・ツールのチェッカーリクエスト、バグ報告などは、以下のいずれかよりお気軽にお寄せください：

- **Issue / バグ報告・機能リクエスト**: [GitHub Issues](https://github.com/sha256san/envdr/issues)
- **ディスカッション・アイデア共有**: [GitHub Discussions](https://github.com/sha256san/envdr/discussions)
- **メール連絡先**: [contact@example.com](mailto:sha256san@gmail.com)
- **リポジトリ**: [https://github.com/sha256san/envdr](https://github.com/sha256san/envdr)

---

## ライセンス

本プロジェクトは [MIT License](LICENSE) のもとで公開されています。
