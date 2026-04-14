use std::path::Path;

fn resolve_git_dir(start: &str) -> Option<std::path::PathBuf> {
    let mut dir = Path::new(start);
    loop {
        let dot_git = dir.join(".git");
        if dot_git.is_dir() {
            return Some(dot_git);
        }
        if dot_git.is_file() {
            let contents = std::fs::read_to_string(&dot_git).ok()?;
            let target = contents.trim().strip_prefix("gitdir: ")?;
            let git_dir = if Path::new(target).is_absolute() {
                target.into()
            } else {
                dir.join(target)
            };
            return Some(git_dir);
        }
        dir = dir.parent()?;
    }
}

fn short_hex(s: &str) -> Option<String> {
    if s.len() >= 7 && s.bytes().all(|b| b.is_ascii_hexdigit()) {
        Some(s[..7].to_string())
    } else {
        None
    }
}

/// Find a tag name pointing at the given full SHA by scanning packed-refs then loose tags.
fn find_tag(git_dir: &Path, full_sha: &str) -> Option<String> {
    // Check packed-refs first (single file, covers most tags)
    if let Ok(packed) = std::fs::read_to_string(git_dir.join("packed-refs")) {
        for line in packed.lines() {
            if line.starts_with('#') { continue; }
            let mut parts = line.split_whitespace();
            if let (Some(sha), Some(refname)) = (parts.next(), parts.next()) {
                if sha == full_sha {
                    if let Some(tag) = refname.strip_prefix("refs/tags/") {
                        return Some(tag.to_string());
                    }
                }
            }
        }
    }

    // Fall back to loose tags in refs/tags/
    let tags_dir = git_dir.join("refs/tags");
    if let Ok(entries) = std::fs::read_dir(&tags_dir) {
        for entry in entries.flatten() {
            if let Ok(contents) = std::fs::read_to_string(entry.path()) {
                if contents.trim() == full_sha {
                    return entry.file_name().to_str().map(str::to_string);
                }
            }
        }
    }

    None
}

#[derive(Default)]
pub struct GitInfo {
    pub branch: Option<String>,
    pub sha: Option<String>,
    pub tag: Option<String>,
}

/// Returns branch name, short SHA, and tag (if HEAD points to a tagged commit).
pub fn info(start: &str) -> GitInfo {
    let Some(git_dir) = resolve_git_dir(start) else {
        return GitInfo::default();
    };
    let Ok(contents) = std::fs::read_to_string(git_dir.join("HEAD")) else {
        return GitInfo::default();
    };
    let trimmed = contents.trim();

    let (branch, full_sha) = if let Some(refpath) = trimmed.strip_prefix("ref: ") {
        let branch = refpath.strip_prefix("refs/heads/").map(str::to_string);
        let full = std::fs::read_to_string(git_dir.join(refpath))
            .ok()
            .map(|s| s.trim().to_string());
        (branch, full)
    } else if short_hex(trimmed).is_some() {
        (short_hex(trimmed), Some(trimmed.to_string()))
    } else {
        return GitInfo::default();
    };

    let sha = full_sha.as_deref().and_then(short_hex);
    let tag = full_sha.as_deref().and_then(|s| find_tag(&git_dir, s));

    GitInfo { branch, sha, tag }
}
