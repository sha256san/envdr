use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Recommendation};
use crate::utils::command::run_cmd_first_line;
use crate::utils::path::find_executable;

pub struct LuaChecker;

impl Checker for LuaChecker {
    fn id(&self) -> &'static str {
        "lua"
    }

    fn title(&self) -> &'static str {
        "Lua Runtime & Tools"
    }

    fn kind(&self) -> CheckerKind {
        CheckerKind::Language
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["luajit"]
    }

    fn is_installed(&self) -> bool {
        find_executable("lua").is_some() || find_executable("luajit").is_some()
    }

    fn check(&self) -> CategoryResult {
        let mut result = CategoryResult::new(self.id(), self.title());

        let lua_p = find_executable("lua");
        let luajit_p = find_executable("luajit");

        if lua_p.is_some() || luajit_p.is_some() {
            if let Some(p) = lua_p {
                let path_str = p.to_string_lossy().to_string();
                let mut lua_item = DiagnosticItem::ok("Lua Interpreter");
                lua_item.path = Some(path_str.clone());

                if let Some(v) = run_cmd_first_line(&path_str, &["-v"]) {
                    lua_item.version = Some(v);
                }
                result.items.push(lua_item);
            }

            if let Some(p) = luajit_p {
                let path_str = p.to_string_lossy().to_string();
                let mut jit_item = DiagnosticItem::ok("LuaJIT (Just-In-Time Compiler)");
                jit_item.path = Some(path_str.clone());

                if let Some(v) = run_cmd_first_line(&path_str, &["-v"]) {
                    jit_item.version = Some(v);
                }
                result.items.push(jit_item);
            }

            if let Some(rocks_p) = find_executable("luarocks") {
                let mut rocks_item = DiagnosticItem::ok("LuaRocks (Package Manager)");
                rocks_item.path = Some(rocks_p.to_string_lossy().to_string());
                if let Some(v) = run_cmd_first_line(&rocks_p.to_string_lossy(), &["--version"]) {
                    rocks_item.version = Some(v);
                }
                result.items.push(rocks_item);
            }
        } else {
            let mut lua_item = DiagnosticItem::error(
                "Lua Runtime",
                "Neither 'lua' nor 'luajit' was found on PATH",
            );
            let install_cmd = crate::system::package_manager::get_install_command("lua")
                .unwrap_or_else(|| "sudo apt install lua5.4 luarocks".to_string());
            lua_item.recommendations.push(Recommendation::full(
                "Install Lua",
                install_cmd,
                "Lua interpreter is required for Lua scripting and Neovim plugins.",
            ));
            result.items.push(lua_item);
        }

        result
    }
}
