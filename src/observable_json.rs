// TODO thin this out more so we avoid unnecessary deserialization errors

use serde_derive::{ Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct ObservableData {
    #[serde(rename = "props")]
    pub props: Props,

    // #[serde(rename = "page")]
    // pub page: String,

    // #[serde(rename = "query")]
    // pub query: Query,

    // #[serde(rename = "buildId")]
    // pub build_id: String,

    // #[serde(rename = "isFallback")]
    // pub is_fallback: bool,

    // #[serde(rename = "gssp")]
    // pub gssp: bool,

    // #[serde(rename = "scriptLoader")]
    // pub script_loader: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Props {
    #[serde(rename = "pageProps")]
    pub page_props: PageProps,

    // #[serde(rename = "__N_SSP")]
    // pub n_ssp: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageProps {
    #[serde(rename = "initialNotebook")]
    pub initial_notebook: InitialNotebook,

    // #[serde(rename = "initialAutosave")]
    // pub initial_autosave: bool,

    // #[serde(rename = "initialSafe")]
    // pub initial_safe: bool,

    // #[serde(rename = "initialCurrentUser")]
    // pub initial_current_user: Option<serde_json::Value>,

    // #[serde(rename = "initialContext")]
    // pub initial_context: InitialContext,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct InitialContext {
    // #[serde(rename = "id")]
    // pub id: String,

    // #[serde(rename = "github_login")]
    // pub github_login: String,

    // #[serde(rename = "avatar_url")]
    // pub avatar_url: String,

    // #[serde(rename = "login")]
    // pub login: String,

    // #[serde(rename = "name")]
    // pub name: String,

    // #[serde(rename = "bio")]
    // pub bio: String,

    // #[serde(rename = "home_url")]
    // pub home_url: String,

    // #[serde(rename = "type")]
    // pub initial_context_type: Option<String>,

    // #[serde(rename = "tier")]
    // pub tier: String,

    // #[serde(rename = "approved")]
    // pub approved: Option<bool>,

    // #[serde(rename = "description")]
    // pub description: Option<String>,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct InitialNotebook {
    // #[serde(rename = "id")]
    // pub id: String,

    #[serde(rename = "slug")]
    pub slug: String,

    // #[serde(rename = "trashed")]
    // pub trashed: bool,

    #[serde(rename = "description")]
    pub description: String,

    // #[serde(rename = "likes")]
    // pub likes: i64,

    // #[serde(rename = "publish_level")]
    // pub publish_level: String,

    // #[serde(rename = "forks")]
    // pub forks: i64,

    // #[serde(rename = "fork_of")]
    // pub fork_of: Option<serde_json::Value>,

    // #[serde(rename = "update_time")]
    // pub update_time: String,

    // #[serde(rename = "publish_time")]
    // pub publish_time: String,

    // #[serde(rename = "publish_version")]
    // pub publish_version: i64,

    // #[serde(rename = "latest_version")]
    // pub latest_version: i64,

    // #[serde(rename = "thumbnail")]
    // pub thumbnail: String,

    // #[serde(rename = "default_thumbnail")]
    // pub default_thumbnail: String,

    // #[serde(rename = "roles")]
    // pub roles: Vec<Option<serde_json::Value>>,

    // #[serde(rename = "sharing")]
    // pub sharing: Option<serde_json::Value>,

    // #[serde(rename = "owner")]
    // pub owner: InitialContext,

    // #[serde(rename = "creator")]
    // pub creator: InitialContext,

    // #[serde(rename = "authors")]
    // pub authors: Vec<InitialContext>,

    // #[serde(rename = "collections")]
    // pub collections: Vec<Option<serde_json::Value>>,

    #[serde(rename = "files")]
    pub files: Option<Vec<File>>,

    // #[serde(rename = "comments")]
    // pub comments: Vec<Option<serde_json::Value>>,

    // #[serde(rename = "commenting_lock")]
    // pub commenting_lock: Option<serde_json::Value>,

    // #[serde(rename = "suggestion_from")]
    // pub suggestion_from: Option<serde_json::Value>,

    // #[serde(rename = "suggestions_to")]
    // pub suggestions_to: Vec<Option<serde_json::Value>>,

    // #[serde(rename = "version")]
    // pub version: i64,

    #[serde(rename = "title")]
    pub title: String,

    // #[serde(rename = "license")]
    // pub license: Option<serde_json::Value>,

    // #[serde(rename = "copyright")]
    // pub copyright: String,

    #[serde(rename = "nodes")]
    pub nodes: Vec<Node>,

    // #[serde(rename = "resolutions")]
    // pub resolutions: Vec<Resolution>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "download_url")]
    pub download_url: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "create_time")]
    pub create_time: String,

    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "size")]
    pub size: i64,

    #[serde(rename = "mime_type")]
    pub mime_type: String,

    #[serde(rename = "content_encoding")]
    pub content_encoding: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "value")]
    pub value: String,

    #[serde(rename = "pinned")]
    pub pinned: bool,

    #[serde(rename = "mode")]
    pub mode: String,

    #[serde(rename = "data")]
    pub data: Option<serde_json::Value>,

    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resolution {
    #[serde(rename = "type")]
    pub resolution_type: String,

    #[serde(rename = "specifier")]
    pub specifier: String,

    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    #[serde(rename = "at")]
    pub at: String,

    #[serde(rename = "specifiers")]
    pub specifiers: Vec<String>,
}
