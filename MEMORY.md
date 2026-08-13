# envdoctor 知識・意思決定ログ (MEMORY.md)

このドキュメントでは、開発を通じて得られた知見、設計上の決定事項（ADR）、プラットフォーム固有の落とし穴と解決パターンを記録・蓄積します。

---

## 1. アーキテクチャ決定事項 (Architecture Decision Records)

### ADR-001: 診断エンジンに Rust を採用
- **理由**:
  1. シングルバイナリで配布可能（ユーザーの環境に Python や Node.js が壊れていても単体で動く）。
  2. 起動が極めて高速（ミリ秒単位）。
  3. クロスプラットフォーム対応が容易。
  4. 厳格な型システムにより、OS固有エラーを堅牢にハンドリング可能。

### ADR-002: 非同期ではなく同期 + 軽量マルチスレッド
- **理由**:
  1. 診断の多くは OS コマンド呼び出し（`which`, `systeminfo`, `nvidia-smi`）やファイル存在確認が中心。
  2. 重厚な非同期ランタイム（Tokio）を使わず、`std::thread` や `rayon` による並列実行でバイナリサイズとビルド時間を最小化。

### ADR-004: パッケージマネージャーの動的抽象化
- **理由**:
  1. Linux ディストリビューション（Debian/Ubuntu, Fedora, Arch, Alpine）や macOS (Homebrew), Windows (Winget/Choco) ごとにパッケージ名やコマンドが異なる。
  2. 実行環境にインストールされているパッケージマネージャーを動的に判定し、ユーザーがそのままコピー＆ペースト、または `fix --apply` で実行できるようにする。

### ADR-005: `fix` コマンドの Safe-by-Default (Dry-run by default)
- **理由**:
  1. システム環境やシェル設定の変更は不可逆な副作用を伴う可能性がある。
  2. デフォルト動作は必ず「計画（Plan）の表示」にとどめ、`--apply` を明示した場合にのみ実行する。

### ADR-006: 未インストール言語チェッカーのコンテキスト別フィルタリング
- **理由**:
  1. 開発者のマシンには通常 1〜3 種類のプログラミング言語しか入っていない。未インストールの全メジャー言語（Ruby, PHP, Zig, Swift 等）を全診断時にエラー表示するとノイズ過多で重要な警告が埋もれる。
  2. そのため全体診断（`envdr`）では `is_installed() == true` な言語のみを表示する。
  3. 一方でユーザーが明示的に `envdr --ruby` や `envdr --language ruby` を指定した場合は、未インストールであることを診断結果（Status::Error）として表示し、OS別のインストール推奨コマンドを案内する。

### ADR-007: 汎用言語診断基盤 (`GenericLanguageChecker`) と `envdr` エイリアス
- **理由**:
  1. 将来的な新言語・カスタム言語の追加を容易にするため、実行ファイル・バージョン取得・環境変数・周辺ツールを宣言的に定義可能なビルダー基盤を構築。
  2. CLI 実行の手間を軽減するため、`Cargo.toml` の `[[bin]]` により `envdoctor` と `envdr` を同等のファーストクラスバイナリとして提供。

---

## 2. プラットフォーム固有の知見 & 落とし穴 (Platform Gotchas)

### 2.1 PATH の扱い
- **Linux / macOS**: デリミタは `:`
- **Windows**: デリミタは `;`
- **注意点**:
  - `std::env::split_paths` を使うことでクロスプラットフォームに安全に分割可能。
  - WSL2 環境では Windows 側の PATH が多数インポートされるため、存在しないパスの警告が大量に出る傾向がある。

### 2.2 シェル設定ファイルの検査
- `~/.bashrc` や `~/.zshrc` で `export PATH=/some/path` のように `$PATH` を含めずに上書きしてしまうと、標準コマンド（`ls`, `cat` 等）が使えなくなる事故が発生しやすい。これを警告する。

### 2.3 Python と Pip のバージョン不一致問題
- **典型事例**: `python3` は 3.12 を指しているが、`pip` は古い 3.10 のグローバルパッケージを指しているケース。
- **診断法**:
  - `python3 -m pip --version` と `pip --version` の出力を比較。
  - 実行ファイルパスと sys.executable の対応関係を検証。

### 2.4 GPU 検出の分離 (NVIDIA vs AMD ROCm)
- **NVIDIA**:
  - `nvidia-smi` コマンドおよび `/dev/nvhost*`, `/dev/nvidia*` の存在。
  - CUDA Driver と CUDA Runtime のバージョンの違い（Driver >= Runtime である必要がある）。
- **AMD ROCm**:
  - `rocm-smi` / `rocminfo` コマンド。
  - Linux における `/dev/kfd` および `/dev/dri/renderD*` のパーミッション（ユーザーが `render` / `video` グループに所属しているかが最重要）。

### 2.5 機密情報のサニタイズ
- 環境変数一覧や設定ダンプを出力・共有する際、以下のキーパターンを含むものは値を `********` でマスクする:
  - `KEY`, `TOKEN`, `SECRET`, `PASSWORD`, `PASSWD`, `AUTH`, `PRIVATE`, `CREDENTIAL`, `API`

---

## 3. 今後の改善・実験メモ
- [ ] 診断ルールのプラグイン化（Wasm または Lua による軽量スクリプティング検討）
- [ ] 自動修復スクリプトの生成機能（`envdoctor fix --generate-script` で bash / PowerShell スクリプトを出力）
