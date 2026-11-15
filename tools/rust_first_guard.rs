// Rust-first CI policy guard: stdlib only, intentionally small and auditable.
// Usage: rust_first_guard [repository_root]
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn is_python_script(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|s| s.to_str()).map(str::to_ascii_lowercase).as_deref(),
        Some("py" | "pyw" | "ipynb")
    )
}

fn is_python_test_command(line: &str) -> bool {
    // This is a conservative line-oriented CI check, not YAML parsing.
    let line = line.trim_start();
    if line.starts_with('#') {
        return false;
    }
    let lower = line.to_ascii_lowercase();
    let shell = if let Some((_, rest)) = lower.split_once("run:") {
        if lower.trim_start().starts_with("- name:") {
            return false;
        }
        rest.trim_start()
    } else {
        lower.trim_start_matches("- ").trim()
    };
    shell.starts_with("pytest ")
        || shell == "pytest"
        || shell.starts_with("python ")
        || shell.starts_with("python3 ")
        || shell.starts_with("python -")
        || shell.starts_with("python3 -")
        || shell.contains(" -m pytest")
        || shell.contains(" -m unittest")
        || shell.contains(" pytest ")
}

fn walk(root: &Path, relative: &Path, errors: &mut Vec<String>) -> std::io::Result<()> {
    let directory = root.join(relative);
    if !directory.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(&directory)? {
        let entry = entry?;
        let path = entry.path();
        let child = relative.join(entry.file_name());
        let metadata = fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() {
            errors.push(format!("symlink in policy-scanned path: {}", child.display()));
            continue;
        }
        if metadata.is_dir() {
            walk(root, &child, errors)?;
        } else if metadata.is_file() {
            if is_python_script(&child) {
                errors.push(format!("Python test/tooling script forbidden: {}", child.display()));
            }
        }
    }
    Ok(())
}

fn check_workflows(root: &Path, errors: &mut Vec<String>) -> std::io::Result<()> {
    let dir = root.join(".github/workflows");
    if !dir.exists() { return Ok(()); }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if fs::symlink_metadata(&path)?.file_type().is_symlink() {
            errors.push(format!("symlinked workflow not allowed: {}", path.display()));
            continue;
        }
        let is_yaml = matches!(path.extension().and_then(|s| s.to_str()), Some("yml" | "yaml"));
        if !is_yaml || !path.is_file() { continue; }
        let content = fs::read_to_string(&path)?;
        for (line_number, line) in content.lines().enumerate() {
            if is_python_test_command(line) {
                errors.push(format!("Python CI test driver in {}:{}: {}", path.display(), line_number + 1, line.trim()));
            }
        }
    }
    Ok(())
}

fn check(root: &Path) -> std::io::Result<Vec<String>> {
    let mut errors = Vec::new();
    for p in ["tests", "test", "scripts", "tools"] {
        walk(root, Path::new(p), &mut errors)?;
    }
    check_workflows(root, &mut errors)?;
    Ok(errors)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        eprintln!("usage: rust_first_guard [repository_root]");
        std::process::exit(2);
    }
    let root = PathBuf::from(args.get(1).map(String::as_str).unwrap_or("."));
    match check(&root) {
        Ok(issues) if issues.is_empty() => {
            println!("Rust-first testing policy: PASS");
        }
        Ok(issues) => {
            for issue in &issues { eprintln!("{issue}"); }
            eprintln!("Rust-first testing policy: FAIL ({} issue(s))", issues.len());
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Rust-first testing policy: could not scan root: {e}");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_python_test_and_script_paths() {
        assert!(is_python_script(Path::new("tests/test_api.py")));
        assert!(is_python_script(Path::new("scripts/ci.PY")));
        assert!(is_python_script(Path::new("tools/notebook.ipynb")));
        assert!(!is_python_script(Path::new("src/lib.rs")));
        assert!(!is_python_script(Path::new("contracts/reference.rs")));
    }

    #[test]
    fn rejects_python_test_drivers_and_preserves_rust_commands() {
        for line in [
            "run: python3 -m unittest discover -s tests",
            "run: pytest -q",
            "python -m pytest tests",
            "python3 scripts/check.py",
            "run: python ./tools/test.py",
        ] {
            assert!(is_python_test_command(line), "should reject: {line}");
        }
        for line in [
            "run: cargo test --locked",
            "run: rustc --test tools/check.rs",
            "# run: python -m pytest",
            "- name: python migration documentation",
        ] {
            assert!(!is_python_test_command(line), "should allow: {line}");
        }
    }

    #[test]
    fn default_product_source_not_in_test_scopes() {
        let p = PathBuf::from("contracts/reference.py");
        assert!(is_python_script(&p));
        assert!(!["tests", "test", "tools", "scripts"].iter().any(|d| p.starts_with(d)));
    }
}
