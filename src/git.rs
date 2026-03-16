use std::path::Path;

fn read_head(git_dir: &Path) -> Option<String> {
    let contents = std::fs::read_to_string(git_dir.join("HEAD")).ok()?;
    let trimmed = contents.trim();
    if let Some(r) = trimmed.strip_prefix("ref: refs/heads/") {
        Some(r.to_string())
    } else if trimmed.len() >= 7 && trimmed.bytes().all(|b| b.is_ascii_hexdigit()) {
        Some(trimmed[..7].to_string())
    } else {
        None
    }
}

pub fn branch(start: &str) -> Option<String> {
    let mut dir = Path::new(start);
    loop {
        let dot_git = dir.join(".git");
        if dot_git.is_dir() {
            return read_head(&dot_git);
        }
        if dot_git.is_file() {
            let contents = std::fs::read_to_string(&dot_git).ok()?;
            let target = contents.trim().strip_prefix("gitdir: ")?;
            let git_dir = if Path::new(target).is_absolute() {
                target.into()
            } else {
                dir.join(target)
            };
            return read_head(&git_dir);
        }
        dir = dir.parent()?;
    }
}
