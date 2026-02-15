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

impl Extractor for GitInfo {
    type Error = GitError;

    fn extract() -> Result<Self, Self::Error> {
        let repo = gix::discover(".").map_err(|e| GitError::Discover {
            inner_error: e.into(),
        })?;

        let head = repo.head_ref().map_err(|e| GitError::Head {
            inner_error: e.into(),
        })?;

        let branch = head.as_ref().map(|r| r.name().shorten().to_string());

        let head_commit = repo.head_commit().map_err(|e| GitError::Head {
            inner_error: e.into(),
        })?;

        let commit_hash = head_commit.id().to_string();
        let commit_short_hash = commit_hash[..7.min(commit_hash.len())].to_string();

        let commit_obj = head_commit.decode().map_err(|e| GitError::Decode {
            inner_error: e.into(),
        })?;

        let committer = commit_obj.committer().map_err(|e| GitError::Decode {
            inner_error: e.into(),
        })?;
        let author = commit_obj.author().map_err(|e| GitError::Decode {
            inner_error: e.into(),
        })?;
        let message = commit_obj.message();

        let commit_timestamp = committer.seconds();
        let commit_message = Some(message.title.to_string());
        let author_name = Some(author.name.to_string());
        let author_email = Some(author.email.to_string());

        let dirty = repo.is_dirty().map_err(|e| GitError::Status {
            inner_error: e.into(),
        })?;

        let tags = repo
            .references()
            .ok()
            .and_then(|refs| {
                refs.tags().ok().map(|tags| {
                    tags.filter_map(Result::ok)
                        .filter_map(|mut r| {
                            let target = r.peel_to_id().ok()?;
                            if target == head_commit.id() {
                                Some(r.name().shorten().to_string())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
            })
            .unwrap_or_default();

        let remote_url = repo
            .find_default_remote(gix::remote::Direction::Push)
            .and_then(Result::ok)
            .and_then(|remote| {
                remote
                    .url(gix::remote::Direction::Push)
                    .map(|u| u.to_bstring().to_string())
            });

        let describe = repo
            .head_commit()
            .ok()
            .and_then(|commit| {
                commit
                    .describe()
                    .names(gix::commit::describe::SelectRef::AllTags)
                    .try_resolve()
                    .ok()
                    .flatten()
                    .map(|resolution| {
                        resolution
                            .format_with_dirty_suffix(None)
                            .map(|f| f.to_string())
                            .ok()
                    })
                    .flatten()
            });

        let commit_count = repo.head_commit().ok().and_then(|commit| {
            commit
                .ancestors()
                .sorting(gix::revision::walk::Sorting::ByCommitTime(
                    Default::default(),
                ))
                .all()
                .ok()
                .map(|iter| iter.filter_map(Result::ok).count() as u64)
        });

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
