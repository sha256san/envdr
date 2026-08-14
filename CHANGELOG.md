# 変更履歴 (CHANGELOG.md)

すべての重要な変更はこのファイルに記録されます。
フォーマットは [Keep a Changelog](https://keepachangelog.com/ja/1.0.0/) に基づいています。

---

## [Unreleased]

### Added
- **診断結果のUX構造改革 (Problem-Cause-Impact-Fix-Verify パイプライン)**:
  - `Issue` モデルへの `cause`（根本原因）および `impact`（開発への影響）フィールドの追加
  - `Recommendation` モデルへの `verification`（修正確認コマンド）フィールドの追加
  - ターミナル出力、Markdown レポート、JSON レポートでの Cause / Impact / Verify 統合出力
- **サマリー (Summary) の Numbered Issues リスト化**:
  - 診断レポート末尾に検出された全問題を重要度タグ（`[CRITICAL]`, `[ERROR]`, `[WARN]`, `[INFO]`）付きで番号一覧表示
  - 問題ごとの影響（Impact）および修正コマンド（Fix）のサマリー抜粋表示
  - `envdr fix` による自動修復への誘導アクションフッターの追加
- **各診断チェッカーの改善**:
  - `SystemChecker`: 箇条書きによる無効 PATH 一覧およびコマンド実行失敗への影響説明
  - `PythonChecker`: Python と pip の実行環境不一致を厳密検知し、`python -m pip` の修正コマンドと影響を提示
  - `GoChecker`: `$GOPATH/bin` 未設定時の影響と PATH 追加コマンドの提示
  - `DockerChecker`: ソケット権限不足・デーモン停止時の影響とグループ追加コマンド・検証コマンドの提示
  - `GitChecker`: グローバルユーザー情報未設定時の影響と設定コマンドの提示
  - `GpuChecker`: ROCm `/dev/kfd` アクセス権限不足時の影響とグループ追加コマンドの提示
  - `ShellChecker`: 破壊的 PATH 上書き構文の検知とシステムコマンド利用不可への影響説明

---

## [0.2.0] - 2026-08-14

### Added
- **マルチアーキテクチャ対応 & 配布基盤**:
  - Linux ARM64 (`aarch64-unknown-linux-gnu` / `musl`) 向けパッケージング & `.deb` 対応
  - macOS Apple Silicon (M1/M2/M3/M4: `aarch64-apple-darwin`) 向けネイティブバイナリ対応
  - macOS Intel (`x86_64-apple-darwin`) 向けバイナリ対応 & Homebrew Tap (`sha256san/tap/envdoctor`)
- **マルチプラットフォーム対応インストーラー (`install.sh`)**:
  - OS (`Linux`, `Darwin`) および CPU アーキテクチャ (`x86_64`, `aarch64`/`arm64`) の自動判別ロジック
  - Apple Silicon / Linux ARM64 への自動ダウンロード & 配置
- **厳密なインストール判定基準 (Installation Criteria Integrity)**:
  - 壊れたシンボリックリンクや実行権限なしファイルの自動除外
  - `asdf`, `pyenv`, `rbenv`, `nvm`, `jenv` などのダミー空シム（実体バージョン未選択）の自動検出・除外
- **パッケージマネージャー鮮度・最新性診断**:
  - APT キャッシュ更新日時の検証（最終 `apt update` から7日以上経過時に警告・推奨）
  - Homebrew, Pacman, DNF などのメタデータ鮮度チェック
  - システム診断 (`envdr --system`) への「Package Manager & Cache」診断項目追加
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
