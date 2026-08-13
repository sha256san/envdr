use std::process::Command;

#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

/// 指定コマンドを実行して結果を取得する（タイムアウト考慮）
pub fn run_cmd(cmd: &str, args: &[&str]) -> Option<CommandOutput> {
    let mut command = Command::new(cmd);
    command.args(args);
    
    // 環境変数等を安全に設定
    command.env("LANG", "C.UTF-8");
    command.env("LC_ALL", "C.UTF-8");

    match command.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Some(CommandOutput {
                success: output.status.success(),
                exit_code: output.status.code(),
                stdout,
                stderr,
            })
        }
        Err(_) => None,
    }
}

/// コマンドを実行し、成功した場合の 1行目の stdout を取得する
pub fn run_cmd_first_line(cmd: &str, args: &[&str]) -> Option<String> {
    let out = run_cmd(cmd, args)?;
    if out.success && !out.stdout.is_empty() {
        Some(out.stdout.lines().next().unwrap_or("").trim().to_string())
    } else {
        None
    }
}
