use std::env;

pub mod signature;

#[derive(Clone)]
pub enum Credentials {
    ApiKey(String, Option<String>),
    ClientId(String, String),
}

impl Credentials {
    pub fn get_key_name(&self) -> String {
        String::from(match self {
            Credentials::ApiKey(_, _) => "key",
            Credentials::ClientId(_, _) => "client",
        })
    }

    pub fn get_key(&self) -> String {
        match &self {
            Credentials::ApiKey(api_key, _) => api_key.clone(),
            Credentials::ClientId(client_id, _) => client_id.clone(),
        }
    }

    pub fn get_secret(&self) -> Option<String> {
        match &self {
            Credentials::ApiKey(_, maybe_secret) => maybe_secret.clone(),
            Credentials::ClientId(_, secret) => Some(secret.clone()),
        }
    }

    pub fn with_api_key<S: AsRef<str>>(api_key: S) -> Credentials {
        Credentials::ApiKey(String::from(api_key.as_ref()), None)
    }

    pub fn with_secret_key<S: AsRef<str>>(api_key: S, secret: S) -> Credentials {
        Credentials::ApiKey(
            String::from(api_key.as_ref()),
            Some(String::from(secret.as_ref())),
        )
    }

    pub fn with_client<S: AsRef<str>>(client_id: S, secret: S) -> Credentials {
        Credentials::ClientId(
            String::from(client_id.as_ref()),
            String::from(secret.as_ref()),
        )
    }

    pub fn try_from_env<S: AsRef<str>>(
        client_id_var_name: S,
        api_key_var_name: S,
        secret_key_var_name: S,
    ) -> Result<Credentials, String> {
        let client_id = env::var(client_id_var_name.as_ref());
        let api_key = env::var(api_key_var_name.as_ref());
        let secret_key = env::var(secret_key_var_name.as_ref());

        if client_id.is_err() && api_key.is_err() {
            return Err(format!(
                "Either '{}' or '{}' must be set as environment variables",
                client_id_var_name.as_ref(),
                api_key_var_name.as_ref()
            ));
        }

        if let Ok(client_id) = client_id {
            // Constructs an instance of ClientIdCredentials
            if secret_key.is_err() {
                return Err(format!(
                    "Secret key not found (expected in environmen variable '{}'). You need to provide a signature when providing a value for environment variable '{}'",
                    secret_key_var_name.as_ref(),
                    client_id_var_name.as_ref()
                ));
            }

            return Ok(Credentials::with_client(client_id, secret_key.unwrap()));
        }

        // Constructs an instance of ApiKeyCredentials
        Ok(Credentials::ApiKey(api_key.unwrap(), secret_key.ok()))
    }

    pub fn try_from_default_env() -> Result<Credentials, String> {
        Self::try_from_env("CLIENT_ID", "API_KEY", "SECRET_KEY")
    }
}

impl<S: AsRef<str>> From<S> for Credentials {
    fn from(api_key: S) -> Self {
        Credentials::with_api_key(api_key)
    }
}
