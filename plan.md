# envdoctor 開発計画書

## 1. プロジェクト概要

### 1.1 プロジェクト名

**envdoctor**

### 1.2 コンセプト

> 開発環境の「動かない」を自動診断し、原因と解決方法を提示するクロスプラットフォームCLIツール。

`envdoctor` は、Python、Rust、C/C++、Node.js、Docker、Git、GPU環境などの開発環境を検査し、

- インストールされているか
- バージョンは適切か
- PATHが正しいか
- 実行ファイルがどこから呼び出されているか
- 複数バージョンが混在していないか
- 必要な環境変数が設定されているか
- 権限に問題がないか
- GPUが認識されているか
- DockerからGPUへアクセスできるか
- Python仮想環境が正しく設定されているか
- Rust/Cargo環境が正しく設定されているか

などを自動的に調査する。

単なる「インストール済み／未インストール」の確認ではなく、

```text
環境
 ↓
検査
 ↓
異常検出
 ↓
原因推定
 ↓
重要度判定
 ↓
解決方法提示
```

まで行うことを目標とする。

---

# 2. 開発背景

## 2.1 開発環境の複雑化

現在のソフトウェア開発では、1つのプログラムを動かすだけでも多数のソフトウェアが必要になる。

例えばPythonによるAI開発では、

```text
Windows / Linux
        ↓
GPU Driver
        ↓
ROCm / CUDA
        ↓
Python
        ↓
pip
        ↓
Virtual Environment
        ↓
PyTorch
        ↓
GPU Runtime
        ↓
AI Application
```

という複数のレイヤーが存在する。

このうち1つでも設定を間違えると、

```text
ModuleNotFoundError
Command not found
Permission denied
DLL not found
Library not found
CUDA is not available
HIP is not available
Python version mismatch
```

などの問題が発生する。

しかし、エラーそのものが根本原因を示しているとは限らない。

例えば、

```text
python
```

は動くのに、

```text
pip
```

が別のPythonに紐づいているケースがある。

また、

```text
python
```

と

```text
python3
```

で異なるPythonを実行している場合もある。

`envdoctor` はこのような**環境そのものの問題を検査するツール**として設計する。

---

# 3. プロジェクトの目的

## 3.1 最終目的

開発者が環境トラブルに遭遇した際、

```bash
envdoctor
```

を実行するだけで、

```text
何が正常なのか
何がおかしいのか
なぜおかしいのか
どう直せばいいのか
```

を把握できるようにする。

---

## 3.2 解決したい問題

### 問題1：PATHの問題

```text
python → /usr/bin/python
pip    → /usr/local/bin/pip
```

のように異なる環境を参照している。

### 問題2：複数バージョンの混在

```text
Python 3.10
Python 3.11
Python 3.12
Python 3.13
```

が同時にインストールされている。

### 問題3：仮想環境の問題

```text
venv
conda
uv
system Python
```

などが混在している。

### 問題4：GPU環境の問題

特にAI開発では、

```text
GPU Driver
ROCm
CUDA
HIP
PyTorch
Docker
```

の組み合わせが複雑になる。

### 問題5：依存関係の問題

```text
Python
 └── PyTorch
      └── CUDA / ROCm
```

のような依存関係のどこで問題が発生しているのか分からない。

---

# 4. 想定ユーザー

## 4.1 メインターゲット

### 初級～中級開発者

```text
Python
Rust
C/C++
JavaScript
```

などを使い始めたユーザー。

## 4.2 AI開発者

```text
PyTorch
TensorFlow
CUDA
ROCm
Docker
```

を使用するユーザー。

## 4.3 Linuxユーザー

Ubuntuなどで、

```text
PATH
permission
package
library
driver
```

の問題に遭遇するユーザー。

## 4.4 学生

高専・大学などで、

```text
Python
C
C++
Rust
Git
Docker
```

を学習している学生。

特に、

> 「先生やWebの記事と同じコマンドを実行したのに動かない」

という問題を対象にする。

---

# 5. 対応OS

## Version 1.0

優先順位は以下とする。

1. Ubuntu / Debian系Linux
2. Windows 11
3. macOS

### Linux

最初に重点的に対応する。

対象：

- Ubuntu
- Debian
- Fedora
- Arch Linux

ただし、初期版ではUbuntuを最優先とする。

### Windows

Windows 10/11を対象。

PowerShellとcmdの双方から実行可能にする。

### macOS

Apple SiliconおよびIntel Macを対象とする。

ただし、1ヶ月目ではLinux/Windowsを優先する。

---

# 6. 技術選定

## 6.1 プログラミング言語

### Rust

採用理由：

- クロスプラットフォーム対応
- 高速
- バイナリ単体配布が容易
- メモリ安全
- CLIツールとの相性が良い
- 外部依存を少なくできる
- 将来的にシステム診断機能を拡張しやすい

---

# 7. 基本アーキテクチャ

```text
┌─────────────────────────────┐
│          envdoctor          │
├─────────────────────────────┤
│ CLI                         │
│ ├── check                   │
│ ├── doctor                  │
│ ├── report                  │
│ ├── fix                     │
│ └── config                  │
├─────────────────────────────┤
│ Diagnostic Engine            │
│ ├── OS Checker               │
│ ├── PATH Checker             │
│ ├── Python Checker           │
│ ├── Rust Checker             │
│ ├── C/C++ Checker            │
│ ├── Node Checker             │
│ ├── Git Checker              │
│ ├── Docker Checker           │
│ ├── GPU Checker              │
│ ├── CUDA Checker             │
│ └── ROCm Checker             │
├─────────────────────────────┤
│ Environment Collector        │
├─────────────────────────────┤
│ Report Generator             │
└─────────────────────────────┘
```

---

# 8. CLI設計

## 8.1 基本コマンド

```bash
envdoctor
```

環境全体を診断する。

## 8.2 詳細診断

```bash
envdoctor doctor
```

全項目を詳細に検査する。

## 8.3 特定言語のみ

```bash
envdoctor check python
envdoctor check rust
envdoctor check node
envdoctor check cpp
```

## 8.4 GPU

```bash
envdoctor check gpu
envdoctor check rocm
envdoctor check cuda
```

## 8.5 Docker

```bash
envdoctor check docker
envdoctor check docker-gpu
```

## 8.6 レポート

```bash
envdoctor report
envdoctor report --format json
envdoctor report --format markdown
envdoctor report --format text
```

---

# 9. 出力設計

## 9.1 通常出力

```text
envdoctor v0.1.0

System
  OS             ✓ Ubuntu 22.04
  Architecture   ✓ x86_64

Python
  Python         ✓ 3.13.14
  pip            ✓ 26.1.2
  PATH           ✓
  Virtual Env    ✓

Rust
  rustc          ✓ 1.89.0
  cargo          ✓ 1.89.0

Git
  git            ✓ 2.43.0

Docker
  docker         ✓ 28.x
  daemon         ✓ running

GPU
  AMD GPU        ✓ detected
  ROCm           ✓ detected
  HIP            ✓ detected

Summary
  ✓ Passed: 17
  ⚠ Warning: 2
  ✗ Failed: 1
```

---

# 10. エラーの重要度

4段階を採用する。

```text
INFO
WARNING
ERROR
CRITICAL
```

## INFO

問題ではないが参考になる情報。

```text
INFO:
Multiple Python versions detected.
```

## WARNING

動作する可能性はあるが、問題につながる可能性がある。

```text
WARNING:
python and pip point to different installations.
```

## ERROR

現在の環境に問題がある。

```text
ERROR:
PyTorch cannot access the GPU.
```

## CRITICAL

主要な開発環境が利用できない。

```text
CRITICAL:
Python executable was not found.
```

---

# 11. Python診断

## 11.1 検査対象

```text
python
python3
python3.x
pip
pip3
venv
virtualenv
conda
uv
pyenv
```

## 11.2 PATH検査

Linux/macOSでは、

```bash
which python
which pip
```

相当の処理を行う。

Windowsでは、

```powershell
where python
where pip
```

相当の処理を行う。

## 11.3 Python/Pip対応確認

例えば、

```text
python → /usr/bin/python3
pip    → /usr/local/bin/pip
```

だった場合、

```text
WARNING

python and pip may belong to different environments.

Recommended:
python -m pip
```

と表示する。

---

# 12. Rust診断

検査対象：

```text
rustc
cargo
rustup
cargo home
rustup home
toolchain
target
```

## 12.1 Toolchain

```bash
rustup show
```

相当の情報を取得する。

例えば、

```text
Default toolchain:
stable-x86_64-unknown-linux-gnu

Installed:
stable
nightly
```

を検出する。

---

# 13. C/C++診断

検査対象：

```text
gcc
g++
clang
clang++
cmake
make
ninja
nasm
lld
```

## 13.1 コンパイラ確認

```text
gcc      ✓
g++      ✓
clang    ✓
cmake    ✓
ninja    ✓
```

## 13.2 コンパイルテスト

単にバージョンを確認するだけでなく、

```text
temporary source
        ↓
compiler
        ↓
executable
        ↓
run
        ↓
success/failure
```

という実際のコンパイルテストを行う。

---

# 14. Node.js診断

検査対象：

```text
node
npm
npx
yarn
pnpm
bun
```

さらに、

```text
nvm
fnm
```

などのバージョン管理ツールも検出する。

---

# 15. Git診断

検査対象：

```text
git
ssh
git config
credential helper
```

## 15.1 Git設定

```bash
git config --global user.name
git config --global user.email
```

などを確認する。

ただし、メールアドレスなどの個人情報を通常の診断レポートに出さない。

```text
user.name     configured
user.email    configured
```

のように状態だけ表示する。

---

# 16. Docker診断

## 検査項目

```text
Docker installed
Docker daemon
Docker socket
Docker Compose
Container runtime
```

## 16.1 GPU Docker

AMD GPUの場合、

```text
/dev/kfd
/dev/dri
ROCm runtime
Docker permissions
```

などを検査する。

NVIDIAの場合は、

```text
nvidia-smi
NVIDIA Container Toolkit
```

などを検査する。

---

# 17. GPU診断

GPU診断は`envdoctor`の重要機能の1つとする。

## 17.1 NVIDIA

```text
GPU
Driver
CUDA
nvidia-smi
CUDA runtime
Container Toolkit
```

## 17.2 AMD

```text
GPU
amdgpu
ROCm
HIP
rocminfo
rocm-smi
```

## 17.3 Intel

将来的に、

```text
Intel GPU
oneAPI
Level Zero
```

にも対応する。

---

# 18. ROCm診断

特にAMD GPUユーザー向けに詳細診断を実装する。

```text
ROCm
├── ROCm version
├── HIP
├── rocminfo
├── rocm-smi
├── /dev/kfd
├── /dev/dri
├── permissions
└── GPU detection
```

## 18.1 PyTorch連携

Python環境で、

```python
import torch

torch.cuda.is_available()
```

相当の確認を行う。

さらに、

```text
torch.version.hip
torch.cuda.device_count()
torch.cuda.get_device_name()
```

などを確認する。

---

# 19. 「診断」から「原因推定」へ

envdoctorの最大の特徴として、

> 単なるチェックツールではなく、異常の原因を推定する。

ことを目指す。

## 例

### 状況

```text
GPU detected ✓
ROCm detected ✓
PyTorch detected ✓

torch.cuda.is_available()
→ false
```

単純なツール：

```text
ERROR: GPU unavailable
```

envdoctor：

```text
ERROR

PyTorch cannot access the AMD GPU.

Detected:
  GPU          ✓
  amdgpu       ✓
  ROCm         ✓
  HIP          ✓
  PyTorch      ✓

Possible causes:

1. PyTorch was installed without ROCm support.
2. The current Python environment differs from the expected environment.
3. ROCm libraries are not available in LD_LIBRARY_PATH.
4. Device permissions may be incorrect.

Most likely:
PyTorch/ROCm environment mismatch.

Run:
envdoctor check rocm --verbose
```

という出力を目指す。

---

# 20. 修正機能

## 20.1 初期版では自動修正しない

安全性を考慮し、Version 0.1では、

```text
diagnose
+
recommend
```

までとする。

勝手に、

```bash
sudo apt remove ...
sudo apt install ...
```

などを実行しない。

## 20.2 将来的なFix機能

```bash
envdoctor fix python-path
```

など。

ただし、

```text
The following changes will be made:

1. Modify PATH
2. Change default Python
3. Create configuration file

Continue? [y/N]
```

のように、**ユーザーの明示的な承認を必須にする**。

---

# 21. 自動修正の安全設計

以下の流れを採用する。

```text
Analyze
 ↓
Explain
 ↓
Preview
 ↓
User approval
 ↓
Backup
 ↓
Change
 ↓
Verify
```

無条件に以下を実行することは避ける。

```text
sudo rm
sudo apt remove
rm -rf
レジストリの無条件変更
PATHの無条件上書き
```

---

# 22. 診断結果のJSON化

機械から利用できるようにJSONを出力する。

```json
{
  "system": {
    "os": "linux",
    "architecture": "x86_64"
  },
  "checks": {
    "python": {
      "status": "ok",
      "version": "3.13.14"
    },
    "rust": {
      "status": "ok"
    },
    "rocm": {
      "status": "warning"
    }
  }
}
```

これにより、

```text
envdoctor
      ↓
JSON
      ↓
GUI
Web UI
CI
VS Code Extension
```

などへの発展が可能になる。

---

# 23. Markdownレポート

```bash
envdoctor report --format markdown
```

で、

```markdown
# Environment Report

## System

- OS: Ubuntu 22.04
- Architecture: x86_64

## Python

- Python: 3.13.14
- pip: 26.1.2
- Status: OK

## GPU

- GPU: AMD Radeon
- ROCm: 7.x
- Status: WARNING
```

を生成する。

GitHub Issueなどに貼り付けやすくする。

---

# 24. プライバシー設計

`envdoctor`は環境情報を大量に扱うため、

**デフォルトでは外部へ何も送信しない。**

## 24.1 原則

```text
No telemetry
No account
No cloud
No tracking
```

を基本方針とする。

## 24.2 外部送信

将来的にAIによる診断サービスを追加する場合も、

```text
Local diagnosis
        ↓
User confirmation
        ↓
Optional upload
```

とする。

---

# 25. AI機能

AIは最初から必須にしない。

まず、

```text
ルールベース診断
```

を完成させる。

## 25.1 理由

診断結果は、

```text
python not found
```

のように機械的に判定できるものが多い。

AIを使う必要がない。

## 25.2 将来的なAI診断

```text
envdoctor
    ↓
diagnostic JSON
    ↓
LLM
    ↓
human-readable explanation
```

とする。

AIには、

```text
何が起きているか
なぜ起きている可能性があるか
どの順番で確認すべきか
```

を説明させる。

---

# 26. プラグインシステム

将来的には診断機能を追加できるようにする。

```text
envdoctor
├── core
├── python
├── rust
├── docker
├── rocm
├── cuda
└── plugins
```

例えば、

```bash
envdoctor check rust
```

はRust checkerを呼び出す。

## Plugin API

将来的に、

```rust
trait Checker {
    fn name(&self) -> &str;
    fn check(&self) -> DiagnosticResult;
}
```

のようなインターフェースを設計する。

---

# 27. CI/CD対応

GitHub Actionsなどで、

```yaml
- name: Environment check
  run: envdoctor --ci
```

と実行できるようにする。

## CIモード

```bash
envdoctor --ci
```

では、

```text
PASS
WARNING
FAIL
```

だけを機械的に返す。

終了コードは以下を基本案とする。

```text
0 = OK
1 = Warning
2 = Error
3 = Critical
```

---

# 28. GitHub Actions

例えば、

```yaml
name: Environment Check

on:
  push:
  pull_request:

jobs:
  envdoctor:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Run envdoctor
        uses: example/envdoctor-action@v1
```

という利用方法を目標とする。

---

# 29. VS Code拡張

将来的な拡張として、

```text
VS Code
 ↓
envdoctor
 ↓
Problems
```

という連携を実装する。

例えば、

```text
ENVIRONMENT

⚠ Python environment mismatch

Python:
  /usr/bin/python3

pip:
  ~/.local/bin/pip

Recommended:
  python -m pip
```

と表示する。

---

# 30. GUI

CLIを完成させた後、GUIを検討する。

```text
┌──────────────────────────────┐
│ envdoctor                    │
├──────────────────────────────┤
│                              │
│ System                 ✓     │
│ Python                 ✓     │
│ Rust                   ✓     │
│ C/C++                  ✓     │
│ Node                   ✓     │
│ Docker                 ⚠     │
│ GPU                    ✓     │
│                              │
├──────────────────────────────┤
│ Issues                       │
│                              │
│ ⚠ Python/Pip mismatch        │
│                              │
│ [View Details]               │
└──────────────────────────────┘
```

ただし、**1ヶ月目ではGUIを作らない。**

CLIを優先する。

---

# 31. ディレクトリ構成

```text
envdoctor/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── CONTRIBUTING.md
│
├── src/
│   ├── main.rs
│   │
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── check.rs
│   │   ├── doctor.rs
│   │   ├── report.rs
│   │   └── fix.rs
│   │
│   ├── core/
│   │   ├── mod.rs
│   │   ├── checker.rs
│   │   ├── diagnostic.rs
│   │   ├── severity.rs
│   │   └── result.rs
│   │
│   ├── system/
│   │   ├── mod.rs
│   │   ├── os.rs
│   │   ├── arch.rs
│   │   └── environment.rs
│   │
│   ├── checkers/
│   │   ├── mod.rs
│   │   ├── python.rs
│   │   ├── rust.rs
│   │   ├── cpp.rs
│   │   ├── node.rs
│   │   ├── git.rs
│   │   ├── docker.rs
│   │   ├── gpu.rs
│   │   ├── rocm.rs
│   │   └── cuda.rs
│   │
│   ├── output/
│   │   ├── mod.rs
│   │   ├── terminal.rs
│   │   ├── json.rs
│   │   └── markdown.rs
│   │
│   └── utils/
│       ├── mod.rs
│       ├── command.rs
│       └── path.rs
│
├── tests/
│   ├── python.rs
│   ├── rust.rs
│   ├── docker.rs
│   └── output.rs
│
└── docs/
    ├── architecture.md
    ├── checks.md
    └── plugin.md
```

---

# 32. 使用するRust Crate

候補：

```toml
[dependencies]

clap = "..."
serde = "..."
serde_json = "..."
anyhow = "..."
thiserror = "..."
owo-colors = "..."
which = "..."
sysinfo = "..."
```

ただし、依存関係は必要最小限にする。

---

# 33. CLIフレームワーク

`clap`を使用する。

例：

```rust
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
```

コマンド：

```text
envdoctor
envdoctor doctor
envdoctor check
envdoctor check python
envdoctor check rust
envdoctor check gpu
envdoctor report
```

---

# 34. 診断結果の内部構造

例えば、

```rust
pub struct DiagnosticResult {
    pub name: String,
    pub status: Status,
    pub message: String,
    pub details: Vec<String>,
    pub recommendations: Vec<String>,
}
```

とする。

## Status

```rust
pub enum Status {
    Info,
    Ok,
    Warning,
    Error,
    Critical,
}
```

---

# 35. コマンド実行の抽象化

OSによってコマンドが異なるため、

```text
Linux
  which

Windows
  where

macOS
  which
```

などを吸収する。

例えば、

```rust
trait Platform {
    fn find_command(&self, name: &str) -> Option<PathBuf>;
}
```

などの設計を検討する。

---

# 36. セキュリティ

`envdoctor`は外部コマンドを実行するため、コマンドインジェクション対策を行う。

悪い例：

```text
Command::new("sh")
    .arg(format!("{} {}", command, user_input))
```

を避ける。

可能な限り、

```rust
Command::new("python")
    .arg("--version")
```

のように引数を分離する。

---

# 37. 秘密情報の扱い

以下を診断対象に含める場合は注意する。

```text
API_KEY
TOKEN
PASSWORD
SECRET
PRIVATE_KEY
```

環境変数の値をそのまま表示しない。

悪い：

```text
OPENAI_API_KEY=sk-xxxxxxxx
```

良い：

```text
OPENAI_API_KEY=present
```

または、

```text
OPENAI_API_KEY=********
```

とする。

---

# 38. テスト戦略

## Unit Test

各Checkerを個別にテストする。

```text
PythonChecker
RustChecker
DockerChecker
GpuChecker
```

## Integration Test

実際の環境で、

```bash
envdoctor
```

を実行する。

## Snapshot Test

CLI出力を固定してテストする。

```text
expected.txt
actual.txt
```

を比較する。

---

# 39. 1ヶ月開発スケジュール

## Week 1：基盤

### Day 1

プロジェクト作成。

```bash
cargo new envdoctor
```

### Day 2

CLI設計。

```text
envdoctor
envdoctor doctor
envdoctor check
```

### Day 3

診断結果のデータ構造。

```text
DiagnosticResult
Status
Recommendation
```

### Day 4

OS検出。

```text
Linux
Windows
macOS
```

### Day 5

PATH診断。

```text
PATH
which
where
```

### Day 6

コマンド検出。

```text
python
rustc
cargo
git
node
docker
```

### Day 7

基本出力完成。

```text
✓
⚠
✗
```

---

# 40. Week 2：主要言語

### Day 8～9

Python checker。

### Day 10

Python/pip対応関係。

### Day 11

Rust checker。

### Day 12

C/C++ checker。

### Day 13

Node.js checker。

### Day 14

Git checker。

---

# 41. Week 3：Docker/GPU

### Day 15～16

Docker checker。

### Day 17

Docker Compose。

### Day 18

GPU検出。

### Day 19

NVIDIA checker。

### Day 20

AMD/ROCm checker。

### Day 21

PyTorch checker。

---

# 42. Week 4：公開品質

### Day 22

JSON出力。

### Day 23

Markdown出力。

### Day 24

エラー原因推定。

### Day 25

README。

### Day 26

テスト。

### Day 27

GitHub Actions。

### Day 28

Windowsビルド。

### Day 29

Linuxビルド。

### Day 30

Version 0.1.0公開。

---

# 43. Version 0.1 MVP

1ヶ月で必ず完成させる範囲。

```text
✓ Linux
✓ Windows
✓ OS detection
✓ Architecture detection
✓ PATH check
✓ Python check
✓ Rust check
✓ C/C++ check
✓ Node.js check
✓ Git check
✓ Docker check
✓ GPU detection
✓ Basic ROCm check
✓ JSON output
✓ Markdown output
✓ Error severity
✓ Basic recommendations
✓ README
✓ GitHub Release
```

---

# 44. Version 0.2

```text
Python virtual environment
Conda
uv
pyenv

Docker GPU
CUDA detailed diagnostics
ROCm detailed diagnostics

Compilation tests
Network tests
Permission tests
```

---

# 45. Version 0.3

```text
Automatic diagnosis
Dependency graph
Configuration analysis
CI mode
GitHub Action
```

---

# 46. Version 1.0

目標：

> 「開発環境で問題が起きたら、とりあえずenvdoctorを実行する」

状態。

```bash
envdoctor doctor
```

だけで、

```text
System
Languages
Package Managers
Compilers
Containers
GPU
Environment Variables
PATH
Permissions
Network
```

を総合診断できるようにする。

---

# 47. 将来的な拡張

## 47.1 AI診断

```text
Diagnostic Data
       ↓
LLM
       ↓
Explanation
```

## 47.2 Web UI

```text
localhost:xxxx
```

で診断結果を表示。

## 47.3 VS Code Extension

VS Code内から、

```text
Run envdoctor
```

を実行できるようにする。

## 47.4 JetBrains Plugin

将来的に、

```text
IntelliJ
CLion
PyCharm
RustRover
```

などにも対応する。

---

# 48. GitHub公開戦略

リポジトリ名：

```text
envdoctor
```

READMEの最初に、

```text
# envdoctor

Diagnose your development environment.

One command to find out
why your development environment doesn't work.
```

とする。

---

# 49. READMEの構成

```text
envdoctor
│
├── What is envdoctor?
├── Features
├── Installation
├── Quick Start
├── Supported Platforms
├── Supported Tools
├── Examples
├── JSON Output
├── CI Usage
├── Privacy
├── Development
├── Contributing
└── License
```

---

# 50. インストール方法

最初はGitHub Releasesからバイナリを配布する。

```text
Windows
envdoctor-x86_64-pc-windows-msvc.exe

Linux
envdoctor-x86_64-unknown-linux-gnu

macOS
envdoctor-aarch64-apple-darwin
envdoctor-x86_64-apple-darwin
```

将来的には、

```bash
cargo install envdoctor
```

にも対応する。

---

# 51. GitHub Actionsによるリリース

タグ：

```text
v0.1.0
v0.2.0
v1.0.0
```

を作成すると、

```text
GitHub Actions
 ↓
Build
 ↓
Test
 ↓
Package
 ↓
Release
```

を自動化する。

---

# 52. ドキュメント

最低限、

```text
README.md
CONTRIBUTING.md
LICENSE
CHANGELOG.md
```

を用意する。

詳細仕様：

```text
docs/
├── architecture.md
├── diagnostics.md
├── security.md
└── plugin.md
```

---

# 53. ライセンス

OSSとして公開することを前提とする。

候補：

```text
MIT
Apache-2.0
```

Rustエコシステムとの親和性を考慮して選択する。

---

# 54. 成功指標

## Version 0.1

最低目標：

```text
GitHub Stars       10+
Downloads          50+
Issues             3+
External users     5+
```

ただし、Starsだけを成功指標にしない。

## 3ヶ月

目標：

```text
Stars              100+
Downloads          1,000+
Contributors       1+
```

---

# 55. より重要なKPI

```text
実際の利用回数
GitHub Issues
Pull Requests
再利用
外部記事での紹介
```

特に、

> 「envdoctorを使ったら問題が解決した」

というIssueが投稿されることを重要な成功指標とする。

---

# 56. 差別化戦略

既存のシステム情報ツールとの差別化として、

```text
System information
```

ではなく、

```text
Development environment diagnosis
```

を中心にする。

## 比較

### 通常のシステム情報ツール

```text
CPU
RAM
GPU
OS
Disk
```

### envdoctor

```text
Python
 ├── executable
 ├── pip
 ├── venv
 └── package

Rust
 ├── rustc
 ├── cargo
 └── toolchain

Docker
 ├── daemon
 ├── compose
 └── GPU

GPU
 ├── driver
 ├── runtime
 └── framework
```

つまり、

> **PCの情報ではなく「開発できる状態か」を検査する。**

---

# 57. 最大の差別化ポイント

最も重要なのは、

```text
✓ Installed
```

だけでは終わらせないこと。

例えば、

```text
Python ✓
pip ✓
PyTorch ✓
GPU ✓
```

でも、

```text
PyTorch → CPU
```

になっていたら、

```text
ERROR:
PyTorch is installed but GPU acceleration is unavailable.

Likely cause:
CPU-only PyTorch installation.
```

と判断する。

---

# 58. 診断チェーン

envdoctorのコア機能として、

```text
OS
 ↓
Driver
 ↓
Runtime
 ↓
Language
 ↓
Package Manager
 ↓
Library
 ↓
Application
```

を順番に検査する。

例えばAI環境：

```text
AMD GPU
 ↓
amdgpu
 ↓
ROCm
 ↓
HIP
 ↓
Python
 ↓
PyTorch
 ↓
torch.cuda.is_available()
 ↓
GPU computation
```

上から順番に確認する。

これにより、

> 「どのレイヤーで壊れているか」

を特定する。

---

# 59. 最終的なビジョン

`envdoctor`を単なるCLIツールで終わらせず、

```text
                  envdoctor
                      │
       ┌──────────────┼──────────────┐
       │              │              │
      CLI            GUI          VS Code
       │              │              │
       └──────────────┼──────────────┘
                      │
               Diagnostic Engine
                      │
       ┌──────────────┼──────────────┐
       │              │              │
    Python          Rust           Docker
       │              │              │
      GPU            C/C++          Node
       │
    ROCm/CUDA
```

という**開発環境診断プラットフォーム**へ発展させる。

---

# 60. 1ヶ月後の完成形

ユーザーが新しいPCやOSをセットアップした直後に、

```bash
envdoctor
```

と実行する。

すると、

```text
╔════════════════════════════════════╗
║          envdoctor 0.1.0          ║
╚════════════════════════════════════╝

System
  ✓ Ubuntu 22.04
  ✓ x86_64

Development Tools
  ✓ Git
  ✓ Rust
  ✓ Python
  ✓ C/C++
  ⚠ Node.js

Python
  ✓ Python 3.13
  ⚠ pip mismatch
  ✓ venv

Docker
  ✓ Docker
  ✓ Compose

GPU
  ✓ AMD Radeon
  ✓ ROCm
  ✓ HIP
  ⚠ PyTorch GPU access

Issues
─────────────────────────────────────

[WARNING] pip points to another Python
[WARNING] PyTorch cannot access GPU

Recommendations
─────────────────────────────────────

1. Use:
   python -m pip

2. Check:
   envdoctor check rocm --verbose

Report saved:
./envdoctor-report.md
```

という結果が得られる。

---

# 61. 最初の開発目標

最初から全部作らない。

まず、

```text
envdoctor
```

で、

```text
OS
PATH
Python
Rust
Git
Docker
GPU
```

だけを検査する。

その後、

```text
原因推定
 ↓
JSON
 ↓
Markdown
 ↓
ROCm
 ↓
CUDA
 ↓
CI
 ↓
AI
```

と拡張する。

---

# 62. 最初のコミット

最初のコミットでは、以下だけを実装する。

```bash
cargo new envdoctor
```

↓

```text
envdoctor
```

↓

```text
System
  OS       ✓
  Arch     ✓

Environment
  PATH     ✓
```

ここまで動けばよい。

その後、

```text
Commit 1
Project skeleton

Commit 2
CLI

Commit 3
OS detection

Commit 4
PATH checker

Commit 5
Python checker

Commit 6
Rust checker

Commit 7
Docker checker

Commit 8
GPU checker

Commit 9
Report

Commit 10
Release v0.1.0
```

という形で進める。

---

# 63. 開発時の基本方針

## 原則1

**診断結果を推測だけで出さない。**

可能な限り実際にコマンドを実行して確認する。

## 原則2

**自動修正より診断を優先する。**

最初のバージョンでは特に重要。

## 原則3

**個人情報を収集しない。**

## 原則4

**外部通信をデフォルト無効にする。**

## 原則5

**CLIを最優先する。**

## 原則6

**1つの問題を確実に解決する。**

---

# 64. 最終目標

`envdoctor`の最終的なキャッチコピー：

> **Your development environment's doctor.**

または、

> **Diagnose your development environment in one command.**

日本語では、

> **開発環境の問題を、1コマンドで診断する。**

を目標とする。

---

# 65. まとめ

`envdoctor`は、

```text
単なる環境情報表示ツール
```

ではなく、

```text
開発環境
 ↓
検査
 ↓
異常検出
 ↓
原因推定
 ↓
修正方法提示
```

を行うツールとして開発する。

1ヶ月目は、

```text
Rust
CLI
OS
PATH
Python
Rust
C/C++
Node.js
Git
Docker
GPU
ROCm
JSON
Markdown
```

までをMVPとする。

その後、

```text
CUDA
詳細ROCm診断
CI
GitHub Action
VS Code
GUI
AI診断
自動修正
Plugin
```

へ拡張する。

最終的には、

> **「環境がおかしい。でも何がおかしいのか分からない」**

という開発者が最初に実行するツールになることを目指す。
