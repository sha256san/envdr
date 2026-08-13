use crate::core::{CategoryResult, Checker, CheckerKind, DiagnosticItem, Recommendation};
use crate::utils::command::run_cmd_first_line;
use crate::utils::path::find_executable;

pub struct DartChecker;

impl Checker for DartChecker {
    fn id(&self) -> &'static str {
        "dart"
    }

    fn title(&self) -> &'static str {
        "Dart & Flutter SDK"
    }

    fn kind(&self) -> CheckerKind {
        CheckerKind::Language
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["flutter"]
    }

    fn is_installed(&self) -> bool {
        find_executable("dart").is_some() || find_executable("flutter").is_some()
    }

    fn check(&self) -> CategoryResult {
        let mut result = CategoryResult::new(self.id(), self.title());

        let dart_p = find_executable("dart");
        let flutter_p = find_executable("flutter");

        if dart_p.is_some() || flutter_p.is_some() {
            if let Some(p) = dart_p {
                let path_str = p.to_string_lossy().to_string();
                let mut dart_item = DiagnosticItem::ok("Dart SDK");
                dart_item.path = Some(path_str.clone());

                if let Some(v) = run_cmd_first_line(&path_str, &["--version"]) {
                    dart_item.version = Some(v);
                }
                result.items.push(dart_item);
            }

            if let Some(p) = flutter_p {
                let path_str = p.to_string_lossy().to_string();
                let mut flutter_item = DiagnosticItem::ok("Flutter SDK");
                flutter_item.path = Some(path_str.clone());

                if let Some(v) = run_cmd_first_line(&path_str, &["--version"]) {
                    flutter_item.version = Some(v);
                }
                result.items.push(flutter_item);
            }
        } else {
            let mut dart_item = DiagnosticItem::error(
                "Dart / Flutter SDK",
                "Neither 'dart' nor 'flutter' executable was found on PATH",
            );
            dart_item.recommendations.push(Recommendation::new(
                "Install Flutter SDK from https://docs.flutter.dev/get-started/install",
            ));
            result.items.push(dart_item);
        }

        result
    }
}
