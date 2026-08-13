# 🩺 envdoctor

<div align="center">

**開発環境の「動かない」を自動診断し、根本原因と解決コマンドを提示するクロスプラットフォームCLIツール**

[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey.svg)]()

[特徴](#-特徴) •
[クイックスタート](#-クイックスタート) •
[診断対象](#-診断対象ツール--環境) •
[使い方](#-使い方--コマンド) •
[出力フォーマット](#-マルチフォーマット出力) •
[安全な自動修復](#-安全な自動修復-fix)

</div>

---

## 💡 What is envdoctor?

`envdoctor` は、Python、Rust、C/C++、Node.js、Docker、Git、GPU（CUDA/ROCm）などの開発環境を検査し、**問題の発見・原因の推定・具体的な解決コマンドの提示** までをワンストップで行う診断ツールです。

単に「インストールされているか」だけでなく、以下のような複雑な環境問題を発見・解決します：

```text
環境スキャン  ──▶  異常・競合検出  ──▶  原因推定  ──▶  解決コマンド提示 (Dry-Run / Apply)
```

- ❌ `python3` と `pip` のバージョン不一致（別バージョンの Python パスを参照している）
- ❌ `PATH` 内の存在しないディレクトリや危険な上書き構文
- ❌ Docker デーモンのソケット権限不足（ユーザーが `docker` グループ未所属）
- ❌ Git コミットの署名設定・ユーザー情報未設定
- ❌ PyTorch が CPU モードで動作している（CUDA/ROCm が認識されていない）
- ❌ Linux における AMD ROCm の `/dev/kfd` アクセス権限不足

---

## ✨ 特徴

- ⚡ **超高速・依存関係ゼロ**: Rust 製シングルバイナリ（~1MB）。Python や Node.js の壊れた環境でも単体動作。
- 🛡️ **安全第一（Safe by Default）**: 診断処理は完全リードオンリー。修復機能（`fix`）はデフォルトで dry-run（実行予定コマンドのプレビュー表示）。
- 📦 **OS・パッケージマネージャー動的対応**: `apt`, `dnf`, `pacman`, `brew`, `winget` などを自動判定し、環境に最適なコマンドを提示。
- 🎨 **リッチなターミナル UI**: 直感的なステータスバッジ（`OK`, `INFO`, `WARN`, `FAIL`）と改善レコメンデーション。
- 📊 **CI/CD & レポート連携**: 構造化 `JSON` 出力および GitHub Issue/PR にそのまま貼れる `Markdown` 出力を標準サポート。
- 🔒 **プライバシー保護**: トークン、パスワード、API キーなどの機密情報を自動マスキング。

---

## 🛠️ 診断対象ツール & 環境

| カテゴリ | コマンド引数 / フラグ | 検査項目 |
| :--- | :--- | :--- |
| **System** | `system`, `--system` | OS、カーネル、CPU/メモリ、`PATH` の重複・壊れたパス |
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

## 🚀 クイックスタート

### 1. ビルド & 実行

`envdoctor` は短縮バイナリ `envdr` としても提供されています。

```bash
# リポジトリのクローン
git clone https://github.com/sha256san/envdr.git
cd envdr

# 全体診断を実行（インストール済み言語 + システム + ツール）
cargo run --bin envdr --
```

---

## 📖 使い方 & コマンド

### 1. 全体診断 (未インストール言語は自動非表示)
システム全体を包括的にスキャンします。開発環境にインストールされている言語のみがすっきり表示されます。

```bash
envdr
# または
envdoctor
```

### 2. 言語・ツールのダイレクト診断 (フラグ指定)
チェックしたい言語やツールを直接フラグで指定して素早く診断できます。

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
envdr check go
envdr check git
```

### 4. 安全な自動修復 (`fix`)
検出された問題に対する解決策を表示・適用します。

```bash
# 修復プランをプレビュー (Dry-run: 変更は加えません)
envdr fix

# 特定カテゴリのみ修復プランを表示
envdr fix git

# 実際に修復コマンドを実行して適用
envdr fix --apply
```

---

## 📊 マルチフォーマット出力

### ターミナル表示 (デフォルト)
```text
 🩺  envdoctor  -  Developer Environment Diagnostic Tool
 ────────────────────────────────────────────────────────────
 ℹ OS: Ubuntu 26.04 | Arch: x86_64 | Kernel: 6.18.33.2-microsoft-standard-WSL2
 ────────────────────────────────────────────────────────────

  OK   Rust Toolchain
   ✔ rustc (Compiler) (rustc 1.97.1)
   ✔ Cargo (Build Tool) (cargo 1.97.1)
   ✔ rustup (Toolchain Manager) (rustup 1.29.0)

 WARN  Git & Version Control
   ✔ Git CLI (git version 2.53.0)
   ▲ Git Author Configuration
     Warning: Git user.name is not set globally
     Warning: Git user.email is not set globally
     💡 Recommendation: Set Git global identity
        $ git config --global user.name "Your Name" && git config --global user.email "you@example.com"
```

### JSON 出力 (`--format json`)
CI/CD パイプラインでの自動検証やメトリクス収集に適しています。

```bash
envdoctor doctor --format json
```

### Markdown 出力 (`--format markdown`)
GitHub Issue や Pull Request に貼り付けて環境共有できます。

```bash
envdoctor doctor --format markdown
# ファイルに保存する場合
envdoctor doctor --format markdown -o report.md
```

---

## 🏗️ プロジェクト構成

```text
envdoctor/
├── SPEC.md        # 要件・仕様定義書
├── TODO.md        # ロードマップ & 未実装タスク一覧
├── MEMORY.md      # AI学習記録・アーキテクチャ設計決定 (ADR)
├── AGENTS.md      # AIエージェント開発規範・コーディング規約
├── CHANGELOG.md   # 変更履歴
├── Cargo.toml     # パッケージ設定
└── src/
    ├── main.rs    # CLI エントリポイント
    ├── cli/       # コマンド定義
    ├── core/      # 診断エンジン・共通型・AutoFixer
    ├── system/    # OS・ハードウェア・パッケージマネージャー検出
    ├── checkers/  # 各環境の診断ロジック
    ├── output/    # ターミナル / JSON / Markdown 出力
    └── utils/     # コマンド実行ラッパー・パス解決・マスキング
```

---

## 🧪 テストの実行

```bash
cargo test
```

---

## 📜 ライセンス

本プロジェクトは [MIT License](LICENSE) のもとで公開されています。
