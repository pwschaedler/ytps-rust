pub mod login {
    use std::fs;
    use std::path::PathBuf;

    use anyhow::Result;
    use yup_oauth2;
    use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};

    use thiserror::Error;
    use tokio::runtime;

    #[derive(Error, Debug)]
    pub enum LoginError {
        #[error("Token cache directory could not be used")]
        TokenCacheError,

        #[error("Could not use provided JSON client secret to login")]
        BadClientSecretError,

        #[error("Could not authenticate using OAuth flow")]
        AuthenticationError,
    }

    /// Login using Google OAuth to enable actions on user data.
    ///
    /// # Errors
    ///
    /// Returns an error if login flow fails.
    pub fn execute(client_secret: PathBuf) -> Result<()> {
        let async_rt = runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        async_rt.block_on(request_login(client_secret))?;
        Ok(())
    }

    /// Request login flow using OAuth.
    ///
    /// Most of this right now is based on the example code from google-youtube3's documentation for
    /// constructing the user flow.
    ///
    /// # Errors
    ///
    /// Returns a `LoginError` when an HTTPS connection cannot be made.
    async fn request_login(client_secret: PathBuf) -> Result<(), LoginError> {
        let project_dir = directories::ProjectDirs::from("com.github", "pwschaedler", "ytps")
            .ok_or(LoginError::TokenCacheError)?;
        fs::create_dir_all(project_dir.cache_dir()).map_err(|_| LoginError::TokenCacheError)?;
        let token_cache_path = project_dir.cache_dir().join("token_cache.json");

        // The actual secret file is only necessary for initial login. Once we have the initial
        // token, it's no longer necessary.
        let secret = yup_oauth2::read_application_secret(client_secret)
            .await
            .map_err(|_| LoginError::BadClientSecretError)?;

        // Authenticator takes care of caching tokens, reading when available, and refreshing
        // when necessary.
        let auth =
            InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
                .persist_tokens_to_disk(token_cache_path)
                .build()
                .await
                .map_err(|_| LoginError::AuthenticationError)?;
        let scopes = &["https://www.googleapis.com/auth/youtube.readonly"];

        // Actual login flow happens here.
        let token = auth.token(scopes).await;
        match token {
            Ok(_) => {
                eprintln!("Login successful.");
                Ok(())
            }
            Err(_) => Err(LoginError::AuthenticationError),
        }
    }
}

pub mod refresh {
    use anyhow::Result;

    /// Refresh cached playlist items.
    ///
    /// # Errors
    ///
    /// Returns an error if playlist items could not be fetched.
    pub const fn execute() -> Result<()> {
        // Need to take an argument for playlist ID/name. Idea: make the argument optional and if
        // none is provided, it uses the last seen (based off cached value).
        Ok(())
    }
}
