# envdoctor 開発ロードマップ & TODO (TODO.md)

このドキュメントでは、`envdoctor` の未実装機能一覧およびリリースごとのタスク進捗を管理します。

---

## 📌 フェーズ 1: MVP (v0.1.0) - 基盤診断 & CLI 【完了】

### 1.1 コア基盤
- [x] プロジェクト構成定義 (`SPEC.md`, `TODO.md`, `MEMORY.md`, `AGENTS.md`, `CHANGELOG.md`)
- [x] Cargo.toml 依存関係設定 (clap, serde, colored, which, sysinfo 等)
- [x] 共通型定義 (`Severity`, `Status`, `Issue`, `Recommendation`, `DiagnosticResult`)
- [x] 診断エンジン抽象化 (`Checker` トレイト、レジストリ)
- [x] 安全なコマンド実行ユーティリティ & パス解決モジュール
- [x] 機密情報 (APIキー、トークン) の自動マスキングフィルター

### 1.2 出力フォーマッター
- [x] ターミナル出力 (カラー、アイコン、ツリー・テーブル表示)
- [x] 構造化 JSON 出力 (`--format json`)
- [x] Markdown レポート出力 (`--format markdown`)

### 1.3 主要チェッカー実装
- [x] **システム診断 (`system`)**: OS、カーネル、メモリ、CPU、PATH環境変数の正当性・重複チェック
- [x] **Python 診断 (`python`)**: python/python3/pip 検出、仮想環境チェック、バージョン不一致検出
- [x] **Rust 診断 (`rust`)**: rustc/cargo/rustup 検出、ツールチェーン、~/.cargo/bin PATH チェック
- [x] **Git 診断 (`git`)**: git コマンド、user.name / user.email 設定、SSH 鍵存在確認
- [x] **Node.js 診断 (`node`)**: node/npm/yarn/pnpm 検出、グローバル PATH 整合性
- [x] **C/C++ 診断 (`cpp`)**: gcc/clang/make/cmake 検出
- [x] **Docker 診断 (`docker`)**: docker CLI、デーモン接続、グループ権限、docker compose
- [x] **GPU 診断 (`gpu`)**: NVIDIA (nvidia-smi, CUDA)、AMD (rocm-smi, HIP)

---

## 📌 フェーズ 2: 診断の深化 & 高度な原因推定 (v0.2.0) 【完了】

### 2.1 高度な環境診断
- [x] Python: `site-packages` 競合、PyTorch/TensorFlow の GPU 認識チェック
- [x] Docker: GPU パススルーコンテナ実行環境 (nvidia-ctk 等) の検出
- [x] C/C++: リンカー (`ld`, `lld`, `mold`)、ビルドシステム検出
- [x] シェル設定診断 (`shell`): `.bashrc`, `.zshrc`, `.profile` 内の不正な `export PATH` 重複や構文エラー

### 2.2 原因推定ルールの拡充
- [x] エラーパターンと解決策ナレッジベースの構造化
- [x] OS固有のパッケージマネージャー（apt, dnf, pacman, brew, winget, choco）に応じたインストールコマンドの動的生成

---

## 📌 フェーズ 3: 安全な自動修復 & 対話モード (v0.3.0) 【完了】

- [x] `envdoctor fix` コマンドの実装 (`src/core/fixer.rs`)
- [x] dry-run モード (`--dry-run`) の徹底 (デフォルト動作)
- [x] `--apply` による安全な自動修復実行
- [x] 特定ターゲット指定修復 (`envdoctor fix git`, `envdoctor fix python` 等)
- [ ] ユーザーへの対話型確認プロンプト (Yes/No)
- [ ] 修復操作のバックアップ・ロールバック記録

---

## 📌 フェーズ 4: 多言語対応・未インストール言語非表示 & CLIフラグ拡張 (v0.4.0) 【完了】

### 4.1 多言語チェッカーの実装 & 拡充
- [x] **Go 診断 (`go`)**: `go`, `gofmt`, `GOPATH`, `GOROOT`, `golangci-lint`
- [x] **Java / JVM 診断 (`java`)**: `java`, `javac`, `JAVA_HOME`, `mvn`, `gradle`, `sdkman`, `kotlinc`
- [x] **Ruby 診断 (`ruby`)**: `ruby`, `gem`, `bundle`, `rbenv`, `rvm`
- [x] **PHP 診断 (`php`)**: `php`, `composer`
- [x] **C# / .NET 診断 (`dotnet`)**: `dotnet` CLI, SDK/Runtime一覧, `DOTNET_ROOT`
- [x] **Swift 診断 (`swift`)**: `swift`, `swiftc`
- [x] **Dart & Flutter 診断 (`dart`)**: `dart`, `flutter`
- [x] **Zig 診断 (`zig`)**: `zig`
- [x] **Lua 診断 (`lua`)**: `lua`, `luajit`, `luarocks`
- [x] **汎用言語チェッカー (`generic`)**: 宣言的な言語定義ビルダー (`GenericLanguageChecker`)

### 4.2 未インストール言語の自動非表示 & フィルタリング
- [x] 全体診断 (`envdr`) 時、未インストールの言語を自動的に除外・非表示化
- [x] 明示指定時 (`envdr --ruby`, `envdr --language ruby`) は未インストールでも診断・インストール推奨を表示

### 4.3 CLI フラグ & バイナリ体系
- [x] 短縮バイナリ `envdr` の追加 (`Cargo.toml`)
- [x] 言語別ダイレクトフラグ (`--python`, `--rust`, `--go`, `--node`, `--java`, `--ruby`, `--php`, `--dotnet`, `--cpp`, `--swift`, `--dart`, `--zig`, `--lua`)
- [x] ツール別ダイレクトフラグ (`--docker`, `--gpu`, `--git`, `--shell`, `--system`)
- [x] カテゴリ一括オプション (`--language` / `-l`, `--tool` / `-t`)

---

## 📌 フェーズ 5: エコシステム & インテグレーション (v1.0.0)

- [ ] YAML/TOML 定義による外部カスタムチェッカー・プラグイン機構
- [ ] GitHub Actions 向け Action (`actions/envdoctor`)
- [ ] VS Code 拡張機能向け Language Server / Status Bar 連携
- [ ] パフォーマンス最適化（各チェッカーの並列並行実行）

---

## 💡 タスク管理ルール
1. タスク着手時は対象項目を `[ ]` から確認し、完了時に `[x]` に更新する。
2. 仕様の変更や新規要件が発生した場合は `SPEC.md` と `TODO.md` の双方を同期更新する。
3. リリース完了時は `CHANGELOG.md` に反映する。
