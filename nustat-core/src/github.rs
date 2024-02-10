use serde::{Deserialize, Serialize};

pub const USER_CONTENT_BASE_URL: &str = "https://raw.githubusercontent.com/shellrow/nustat";
pub const REPOSITORY_INFO_URL: &str = "https://api.github.com/repos/shellrow/nustat/branches/main";

#[derive(Debug, Deserialize, Serialize)]
pub struct RepositoryInfo {
    pub name: String,
    pub commit: Commit,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Commit {
    pub sha: String,
    pub node_id: String,
    pub commit: CommitDetails,
    pub url: String,
    pub html_url: String,
    pub comments_url: String,
    pub author: User,
    pub committer: User,
    pub parents: Vec<Parent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommitDetails {
    pub author: Committer,
    pub committer: Committer,
    pub message: String,
    pub tree: Tree,
    pub url: String,
    pub comment_count: u32,
    pub verification: Verification,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Committer {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tree {
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Verification {
    pub verified: bool,
    pub reason: String,
    pub signature: Option<String>,
    pub payload: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub r#type: String,
    pub site_admin: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Parent {
    pub sha: String,
    pub url: String,
    pub html_url: String,
}
