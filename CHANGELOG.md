# 変更履歴 (CHANGELOG.md)

すべての重要な変更はこのファイルに記録されます。
フォーマットは [Keep a Changelog](https://keepachangelog.com/ja/1.0.0/) に基づいています。

---

## [Unreleased]

### Added
- 短縮バイナリ名 `envdr` を `Cargo.toml` に追加
- メジャー言語チェッカーの新規追加:
  - Go 診断 (`checkers::go`): `go`, `gofmt`, `GOPATH`, `GOROOT`, `golangci-lint`
  - Java / JVM 診断 (`checkers::java`): `java`, `javac`, `JAVA_HOME`, `mvn`, `gradle`, `kotlinc`, SDKMAN
  - Ruby 診断 (`checkers::ruby`): `ruby`, `gem`, `bundle`, `rbenv`, `rvm`
  - PHP 診断 (`checkers::php`): `php`, `composer`
  - C# / .NET 診断 (`checkers::dotnet`): `dotnet` CLI, SDK/Runtime一覧, `DOTNET_ROOT`
  - Swift 診断 (`checkers::swift`): `swift`, `swiftc`
  - Dart & Flutter 診断 (`checkers::dart`): `dart`, `flutter`
  - Zig 診断 (`checkers::zig`): `zig`
  - Lua 診断 (`checkers::lua`): `lua`, `luajit`, `luarocks`
- 汎用言語診断ビルダー基盤 (`checkers::generic::GenericLanguageChecker`)
- 未インストール言語の自動非表示機能（全体診断時にインストールされていない言語を自動スキップしノイズを低減）
- CLI 言語別ダイレクトフラグ (`--python`, `--rust`, `--go`, `--node`, `--java`, `--ruby`, `--php`, `--dotnet`, `--cpp`, `--swift`, `--dart`, `--zig`, `--lua`)
- CLI ツール別ダイレクトフラグ (`--docker`, `--gpu`, `--git`, `--shell`, `--system`)
- カテゴリ別診断オプション (`--language` / `-l`, `--tool` / `-t`)
- パッケージマネージャーへの新言語インストールコマンドマッピング追加
- 公式ドキュメント `README.md` の作成（特徴、クイックスタート、診断対象、使い方、出力例、アーキテクチャ）
- パッケージマネージャー自動検出モジュール (`src/system/package_manager.rs`)
- シェル環境・設定ファイル診断チェッカー (`src/checkers/shell.rs`)
- 安全な自動修復コマンド `envdoctor fix` (`src/core/fixer.rs`)
- dry-run モード (`--dry-run`) および適用モード (`--apply`)
- PyTorch / ML フレームワークの GPU 認識診断 (`src/checkers/python.rs`)
- リンカー (`ld`, `lld`, `mold`) 診断 (`src/checkers/cpp.rs`)
- Docker GPU パススルーコンテナランタイム検出 (`src/checkers/docker.rs`)
- Git コミット署名設定 (`commit.gpgsign`, `user.signingkey`) 診断 (`src/checkers/git.rs`)
- プロジェクト標準ドキュメント構造の整備 (`SPEC.md`, `TODO.md`, `MEMORY.md`, `AGENTS.md`, `CHANGELOG.md`)
- `Cargo.toml` による Rust プロジェクト初期化
- コア診断基盤 (`core::Checker`, `Severity`, `Status`, `DiagnosticResult`, `Issue`, `Recommendation`)
- システム情報・PATH解析モジュール (`system`)
- 主要診断チェッカー (`checkers::system`, `python`, `rust`, `git`, `node`, `cpp`, `docker`, `gpu`)
- 3種類の出力フォーマッター (`output::TerminalFormatter`, `JsonFormatter`, `MarkdownFormatter`)
- 安全なコマンド実行と機密情報サニタイズユーティリティ (`utils`)
- CLI インターフェース (`clap` による `doctor`, `check`, `report`, `fix` コマンド)

---

## [0.1.0] - 2026-08-12
### Initial Release
- 初回MVPリリース
