use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::extractor::{Extractor, git::error::GitError};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GitInfo {
    pub commit_hash: String,
    pub commit_short_hash: String,
    pub branch: Option<String>,
    pub dirty: bool,
    pub commit_timestamp: i64,
    pub commit_message: Option<String>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub tags: Vec<String>,
    pub remote_url: Option<String>,
    pub describe: Option<String>,
    pub commit_count: Option<u64>,
}

/// Run `git` with the given args in the current directory.
fn git(args: &[&str]) -> std::io::Result<std::process::Output> {
    Command::new("git").current_dir(".").args(args).output()
}

/// Run `git`, returning trimmed stdout on success, or `None` if the binary is
/// missing, the command exits non-zero, or the output is empty. Used for the
/// optional fields where absence is a valid, expected result.
fn git_opt(args: &[&str]) -> Option<String> {
    let out = git(args).ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    (!s.is_empty()).then_some(s)
}

/// Resolve the push URL of the default remote, preferring `origin`, otherwise
/// the first configured remote.
fn default_remote_url() -> Option<String> {
    let remotes = git_opt(&["remote"])?;
    let names: Vec<&str> = remotes.lines().collect();
    let name = if names.contains(&"origin") {
        "origin"
    } else {
        *names.first()?
    };
    git_opt(&["remote", "get-url", "--push", name])
}

impl Extractor for GitInfo {
    type Error = GitError;

    fn extract() -> Result<Self, Self::Error> {
        // Confirm we are inside a working tree before doing anything else.
        let inside = git(&["rev-parse", "--is-inside-work-tree"]).map_err(|e| {
            GitError::Discover {
                inner_error: e.into(),
            }
        })?;
        if !inside.status.success() {
            let msg = String::from_utf8_lossy(&inside.stderr).trim().to_string();
            return Err(GitError::Discover {
                inner_error: std::io::Error::other(msg).into(),
            });
        }

        // Pull the core commit fields in a single call.
        // Format: hash / unix timestamp / author name / author email / subject.
        let head = git(&["log", "-1", "--format=%H%n%ct%n%an%n%ae%n%s"]).map_err(|e| {
            GitError::Head {
                inner_error: e.into(),
            }
        })?;
        if !head.status.success() {
            let msg = String::from_utf8_lossy(&head.stderr).trim().to_string();
            return Err(GitError::Head {
                inner_error: std::io::Error::other(msg).into(),
            });
        }

        let stdout = String::from_utf8_lossy(&head.stdout);
        let mut lines = stdout.lines();
        let commit_hash = lines.next().unwrap_or_default().to_string();
        let commit_timestamp = lines
            .next()
            .and_then(|s| s.parse::<i64>().ok())
            .ok_or_else(|| GitError::Decode {
                inner_error: std::io::Error::other("failed to parse commit timestamp").into(),
            })?;
        let author_name = lines.next().map(str::to_string);
        let author_email = lines.next().map(str::to_string);
        let commit_message = lines.next().map(str::to_string);

        let commit_short_hash = commit_hash.chars().take(7).collect::<String>();

        // Detached HEAD has no symbolic branch name.
        let branch = git_opt(&["symbolic-ref", "--quiet", "--short", "HEAD"]);

        let status = git(&["status", "--porcelain"]).map_err(|e| GitError::Status {
            inner_error: e.into(),
        })?;
        if !status.status.success() {
            let msg = String::from_utf8_lossy(&status.stderr).trim().to_string();
            return Err(GitError::Status {
                inner_error: std::io::Error::other(msg).into(),
            });
        }
        let dirty = !String::from_utf8_lossy(&status.stdout).trim().is_empty();

        let tags = git_opt(&["tag", "--points-at", "HEAD"])
            .map(|s| s.lines().map(str::to_string).collect())
            .unwrap_or_default();

        let remote_url = default_remote_url();

        let describe = git_opt(&["describe", "--tags"]);

        let commit_count =
            git_opt(&["rev-list", "--count", "HEAD"]).and_then(|s| s.parse::<u64>().ok());

        Ok(GitInfo {
            commit_hash,
            commit_short_hash,
            branch,
            dirty,
            commit_timestamp,
            commit_message,
            author_name,
            author_email,
            tags,
            remote_url,
            describe,
            commit_count,
        })
    }
}
