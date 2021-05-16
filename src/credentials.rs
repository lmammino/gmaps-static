#[derive(Clone)]
pub struct Credentials<S: AsRef<str>> {
    pub api_key: S,
    pub secret_key: Option<S>,
}

impl<S: AsRef<str>> Credentials<S> {
    pub fn new(api_key: S) -> Self {
        Credentials {
            api_key,
            secret_key: None,
        }
    }

    pub fn with_secret_key(api_key: S, secret_key: S) -> Self {
        Credentials {
            api_key,
            secret_key: Some(secret_key),
        }
    }

    pub fn has_secret_key(&self) -> bool {
        self.secret_key.is_some()
    }
}

impl<S: AsRef<str>> From<S> for Credentials<S> {
    fn from(api_key: S) -> Self {
        Credentials::new(api_key)
    }
}
