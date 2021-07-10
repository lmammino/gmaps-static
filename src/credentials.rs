use std::env;

#[derive(Clone)]
pub struct Credentials {
    pub api_key: String,
    pub secret_key: Option<String>,
}

impl Credentials {
    pub fn new<S: AsRef<str>>(api_key: S) -> Self {
        Credentials {
            api_key: String::from(api_key.as_ref()),
            secret_key: None,
        }
    }

    pub fn with_secret_key<S: AsRef<str>>(api_key: S, secret_key: S) -> Self {
        Credentials {
            api_key: String::from(api_key.as_ref()),
            secret_key: Some(String::from(secret_key.as_ref())),
        }
    }

    pub fn try_from_env<S: AsRef<str>>(
        api_key_var_name: S,
        secret_key_var_name: S,
    ) -> Result<Credentials, String> {
        let api_key = env::var(api_key_var_name.as_ref()).map_err(|_| {
            format!(
                "Environment variable '{}' not set",
                api_key_var_name.as_ref()
            )
        })?;

        let secret_key = env::var(secret_key_var_name.as_ref()).ok();

        Ok(Credentials {
            api_key,
            secret_key,
        })
    }
}

impl<S: AsRef<str>> From<S> for Credentials {
    fn from(api_key: S) -> Self {
        Credentials::new(api_key)
    }
}
