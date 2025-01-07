use std::collections::HashMap;
use url::Url;

/// Deprecated.  Use [`Options`](crate::Options).
#[deprecated]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, derive_getters::Getters, derive_new::new,
)]
pub struct User {
    #[new(into)]
    url: Url,
    #[new(into)]
    api: String,
}

impl User {
    #[deprecated]
    #[tracing::instrument(skip_all)]
    pub fn body(&self) -> String {
        let mut body = self.url.to_string();
        body.push_str(&format!("?&UserID={}", self.api));
        body
    }

    #[deprecated]
    #[tracing::instrument(skip_all)]
    pub fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("UserID".to_string(), self.api.clone());
        params
    }
}
