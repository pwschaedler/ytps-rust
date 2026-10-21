pub mod cache {
    use anyhow::Result;
    use std::fs;
    use std::path::PathBuf;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum CacheError {
        #[error("Token cache directory could not be used")]
        TokenCacheError,
    }

    /// Ensure directory structure for project cache directory is created.
    ///
    /// # Errors
    ///
    /// Returns a `TokenCacheError` when project cache directory is not usable.
    pub fn create_cache_directory() -> Result<()> {
        let project_dir = directories::ProjectDirs::from("com.github", "pwschaedler", "ytps")
            .ok_or(CacheError::TokenCacheError)?;
        fs::create_dir_all(project_dir.cache_dir()).map_err(|_| CacheError::TokenCacheError)?;
        Ok(())
    }

    /// Retrieve path used for token cache.
    ///
    /// # Errors
    ///
    /// Returns a `TokenCacheError` when project cache directory is not usable.
    pub fn get_token_cache_path() -> Result<PathBuf> {
        let project_dir = directories::ProjectDirs::from("com.github", "pwschaedler", "ytps")
            .ok_or(CacheError::TokenCacheError)?;
        let token_cache_path = project_dir.cache_dir().join("token_cache.json");
        Ok(token_cache_path)
    }
}

pub mod login {
    use std::path::PathBuf;

    use anyhow::Result;
    use yup_oauth2;
    use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};

    use thiserror::Error;

    use crate::cache;

    #[derive(Error, Debug)]
    pub enum LoginError {
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
    pub async fn execute(client_secret: PathBuf) -> Result<()> {
        request_login(client_secret).await?;
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
    async fn request_login(client_secret: PathBuf) -> Result<()> {
        cache::create_cache_directory()?;
        let token_cache_path = cache::get_token_cache_path()?;

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
        let _ = auth.token(scopes).await?;
        eprintln!("Login successful.");
        Ok(())
    }

    /// Check if a user has an existing login token, meaning they're logged in.
    #[must_use]
    pub fn check_login_token() -> bool {
        cache::get_token_cache_path()
            .and_then(|path| path.try_exists().map_err(anyhow::Error::new))
            .unwrap_or(false)
    }
}

pub mod refresh {
    use crate::{cache, login};
    use anyhow::Result;
    use thiserror::Error;
    use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};

    extern crate google_youtube3 as youtube3;
    use youtube3::{FieldMask, YouTube, hyper_rustls, hyper_util};

    #[derive(Error, Debug)]
    pub enum RefreshError {
        #[error("Must log in before managing playlists. Run `ytps-rust login -h`")]
        NotLoggedInError,
    }

    /// Refresh cached playlist items.
    ///
    /// # Errors
    ///
    /// Returns an error if playlist items could not be fetched.
    pub async fn execute() -> Result<()> {
        // Need to take an argument for playlist ID/name. Idea: make the argument optional and if
        // none is provided, it uses the last seen (based off cached value).
        const _PLAYLIST_NAME: &str = "2024-05-13 Watch Later";
        const PLAYLIST_ID: &str = "PLMsroFQMqFwI082s1_mf4oU9ILn-09PfM";

        // Retrieve token from cache
        let token_cache_path = cache::get_token_cache_path()?;

        // Check if user is already logged in and error if not (basically do they have a cached
        // token)
        if !login::check_login_token() {
            return Err(RefreshError::NotLoggedInError.into());
        }

        let auth = InstalledFlowAuthenticator::builder(
            yup_oauth2::ApplicationSecret::default(),
            InstalledFlowReturnMethod::HTTPRedirect,
        )
        .persist_tokens_to_disk(token_cache_path)
        .build()
        .await
        .map_err(|_| login::LoginError::AuthenticationError)?;

        // Use the API to pull videos
        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build(
                    hyper_rustls::HttpsConnectorBuilder::new()
                        .with_native_roots()?
                        .https_or_http()
                        .enable_http2()
                        .build(),
                );
        let mut hub = YouTube::new(client, auth);
        let result = hub
            .playlist_items()
            .list(&vec!["snippet".into()])
            .playlist_id(PLAYLIST_ID)
            .doit()
            .await;
        if let Ok(items) = result {
            dbg!(items);
        }

        // Put that video data somewhere, give it a TTL

        // Extras for later: caching the playlist name, caching the videos, organizing video data in
        // SQLite or something

        Ok(())
    }
}
