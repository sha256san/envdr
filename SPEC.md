# envdoctor 仕様書 (SPEC.md)

## 1. 概要 (Overview)

`envdoctor` は、開発環境の「動かない」「バージョンが合わない」「PATHが通っていない」などの問題を自動診断し、根本原因と具体的な解決策（コマンド・手順）を提示するクロスプラットフォーム（Linux / macOS / Windows）CLIツールである。

---

## 2. コア設計思想 (Core Concepts)

1. **「状態の確認」だけでなく「原因推定」と「解決策」まで提示する**
   - 単に `python not found` と表示するだけでなく、`python3` のみ存在する場合はエイリアス設定やシンボリックリンクの作成を案内する。
2. **非破壊・安全性の優先**
   - 診断は環境を一切破壊しない（リードオンリー動作）。修正機能 (`fix`) は dry-run を基本とし、ユーザーの明示的な確認なしに変更を行わない。
3. **CI/CD および AI 連携**
   - 人間が見やすいリッチなターミナル出力に加え、CI/CDでパースしやすい構造化 JSON や、GitHub Issue / PR にそのまま貼れる Markdown 出力を標準サポート。
4. **プライバシーと秘密情報の保護**
   - 環境変数や設定ファイルに含まれるトークン、パスワード、APIキー、機密パスを自動でマスキングする。

---

## 3. CLI コマンド仕様 (CLI Interface)

### 3.1 コマンド & バイナリ体系

`envdoctor` は短縮コマンド `envdr` でも実行可能です。

```bash
# 全体診断 (インストール済み言語 + システム + ツールを自動検出・診断)
envdoctor [doctor] [OPTIONS]
envdr [OPTIONS]

# 言語別診断フラグ
envdr --python                 # Python のみ診断
envdr --rust                   # Rust のみ診断
envdr --go                     # Go のみ診断
envdr --node                   # Node.js/JS/TS のみ診断
envdr --java                   # Java/JVM のみ診断
envdr --ruby                   # Ruby のみ診断
envdr --php                    # PHP のみ診断
envdr --dotnet                 # .NET/C# のみ診断
envdr --cpp                    # C/C++ のみ診断
envdr --swift                  # Swift のみ診断
envdr --dart                   # Dart/Flutter のみ診断
envdr --zig                    # Zig のみ診断
envdr --lua                    # Lua のみ診断
envdr --python --rust          # 複数言語の組み合わせ診断

# ツール別診断フラグ
envdr --docker                 # Docker のみ診断
envdr --gpu                    # GPU/CUDA/ROCm のみ診断
envdr --git                    # Git のみ診断
envdr --shell                  # Shell のみ診断
envdr --system                 # System のみ診断

# カテゴリ一括オプション
envdr -l, --language [<LANG>]  # 言語診断 (例: envdr -l, envdr --language python,go)
envdr -t, --tool [<TOOL>]      # ツール診断 (例: envdr -t, envdr --tool docker,gpu)

# 特定ターゲットの直接診断
envdoctor check <TARGET> [OPTIONS]

# レポート生成
envdoctor report [OPTIONS]

# 安全な自動修復支援 (Phase 3)
envdoctor fix [TARGET] [--apply] [--dry-run]
```

### 3.2 未インストール言語の自動非表示仕様
- **全体診断 (`envdr`) / 全言語診断 (`envdr --language`)**: システムにインストールされていない言語（バイナリ未検出）は自動的に除外・非表示となり、余計なエラーで画面が埋まるのを防止します。
- **個別指定時 (`envdr --ruby`, `envdr --language ruby`, `envdr check ruby`)**: 対象言語が未インストールであっても明示的に診断され、エラー表示とともに OS パッケージマネージャーに対応したインストールコマンド（`apt`, `brew`, `dnf`, `pacman` 等）が推薦されます。

### 3.3 共通オプション

- `-l, --language [<LANG>]`: プログラミング言語環境の診断（指定なしで全インストール済み言語、またはカンマ区切りで指定）
- `-t, --tool [<TOOL>]`: 開発ツールの診断（指定なしで全ツール、またはカンマ区切りで指定）
- `-f, --format <FORMAT>`: 出力形式 (`terminal` [デフォルト], `json`, `markdown`)
- `-v, --verbose`: 詳細情報（環境変数、PATH一覧、検出ファイル詳細など）を表示
- `-q, --quiet`: エラー・警告のみを表示
- `-o, --output <FILE>`: レポート結果を指定ファイルに出力

---

## 4. 診断ステータスと重要度 (Severity & Status)

各診断項目は以下のステータスと重要度を持つ：

| ステータス | 記号 / アイコン | レベル | 説明 |
| :--- | :---: | :--- | :--- |
| **OK** | `✔` (緑) | 正常 | 正常に検出され、設定・バージョンに問題がない |
| **INFO** | `ℹ` (青) | 情報 | 補足情報（複数バージョンの存在、推奨設定の提案等） |
| **WARNING** | `⚠` (黄) | 警告 | 直ちにエラーではないが、将来問題になり得る状態（PATH順序の不整合、古いバージョン等） |
| **ERROR** | `✖` (赤) | エラー | 実行不可または壊れている（コマンド未検出、パーミッション拒否等） |
| **CRITICAL** | `🔥` (マゼンタ) | 致命的 | 開発継続不能な重大不整合（ドライバー不一致、ライブラリ破損等） |

---

## 5. 診断対象モジュール仕様 (Checkers)

### 5.1 システム情報 (`system`)
- OS種別、ディストリビューション、カーネルバージョン、CPUアーキテクチャ
- システムメモリ、空きディスク容量
- `PATH` 環境変数の構造解析（重複パス、存在しないパス、アクセス不可パスの検出）

### 5.2 シェル環境診断 (`shell`)
- 現在のログインシェル（bash, zsh, fish 等）のバージョンと設定ファイルパス
- `~/.bashrc`, `~/.zshrc`, `~/.profile` 内の重複 PATH export、存在しない参照の検査

### 5.3 プログラミング言語診断 (Languages)
- **Python (`python`)**: `python3`/`python`/`pip` の実体パス、仮想環境 (venv/conda/uv/poetry)、PyTorch GPU 認識
- **Rust (`rust`)**: `rustc`, `cargo`, `rustup`, ツールチェーン, `~/.cargo/bin` PATH
- **Node.js / JS / TS (`node`)**: `node`, `npm`, `yarn`, `pnpm`, `bun`, `deno`, バージョンマネージャー (nvm/fnm/volta)
- **Go (`go`)**: `go`, `gofmt`, `GOPATH`, `GOROOT`, `GOPATH/bin` PATH, `golangci-lint`
- **Java / JVM (`java`)**: `java`, `javac`, `JAVA_HOME`, `mvn` (Maven), `gradle`, `kotlinc`, SDKMAN
- **Ruby (`ruby`)**: `ruby`, `gem`, `bundle` (Bundler), `rbenv`, `rvm`, `asdf`
- **PHP (`php`)**: `php` CLI, `composer`
- **C# / .NET (`dotnet`)**: `dotnet` CLI, SDK一覧 (`--list-sdks`), Runtime一覧 (`--list-runtimes`), `DOTNET_ROOT`
- **C / C++ (`cpp`)**: コンパイラ (`gcc`, `g++`, `clang`, `clang++`), リンカー (`mold`, `lld`, `ld`), ビルドツール (`make`, `cmake`, `ninja`)
- **Swift (`swift`)**: `swift`, `swiftc`
- **Dart & Flutter (`dart`)**: `dart`, `flutter`
- **Zig (`zig`)**: `zig`
- **Lua (`lua`)**: `lua`, `luajit`, `luarocks`
- **汎用言語チェッカー (`generic`)**: 任意の言語を宣言的に定義・診断可能

### 5.4 開発ツール診断 (Tools)
- **Git (`git`)**: `git` CLI, `user.name`/`user.email` 設定, SSH 鍵, コミット署名
- **Docker (`docker`)**: `docker` CLI, デーモン稼働, ソケット権限, Docker Compose, GPU パススルー
- **GPU / CUDA / ROCm (`gpu`)**: NVIDIA (`nvidia-smi`, `nvcc`), AMD ROCm (`rocm-smi`, `rocminfo`, `/dev/kfd`), 権限グループ

---

## 6. 出力データ構造 (Data Schema)

### JSON 出力フォーマット例

```json
{
  "version": "0.1.0",
  "timestamp": "2026-08-12T15:00:00Z",
  "system": {
    "os": "linux",
    "distribution": "Ubuntu 24.04 LTS",
    "arch": "x86_64",
    "kernel": "6.8.0-generic"
  },
  "summary": {
    "ok": 12,
    "info": 3,
    "warning": 2,
    "error": 1,
    "critical": 0
  },
  "results": [
    {
      "category": "python",
      "name": "Python 3 Interpreter",
      "status": "OK",
      "version": "3.12.3",
      "path": "/usr/bin/python3",
      "details": ["Symlinked from /usr/bin/python3.12"],
      "issues": [],
      "recommendations": []
    },
    {
      "category": "python",
      "name": "Pip Alignment",
      "status": "WARNING",
      "message": "pip executable points to different Python installation",
      "issues": [
        "pip (/usr/local/bin/pip) is bound to Python 3.10, but active python is 3.12"
      ],
      "recommendations": [
        "Use 'python3 -m pip' instead of calling 'pip' directly",
        "Or reinstall pip for Python 3.12: python3 -m ensurepip --upgrade"
      ]
    }
  ]
}
```

---

## 7. セキュリティとプライバシー規約

1. **トークン・パスワードの保護**:
   - `*_KEY`, `*_TOKEN`, `*_SECRET`, `*_PASSWORD`, `*_AUTH` 等の環境変数は常に `********` に置換。
2. **ユーザーディレクトリの抽象化（オプション）**:
   - レポート共有用に `/home/username` を `~` または `<USER_HOME>` に置換可能にする。
