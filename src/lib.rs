pub mod login {
    use anyhow::Result;
    use google_youtube3::{YouTube, hyper_rustls, hyper_util, yup_oauth2};

    use thiserror::Error;
    use tokio::runtime;

    const _CLIENT_ID: &str =
        "565952211844-5t5nqju0v5oj4rvboght0kqrqv38de65.apps.googleusercontent.com";

    #[derive(Error, Debug)]
    pub enum LoginError {
        #[error("Could not detect valid TLS certs")]
        InvalidCertsError,

        #[error("Could not authenticate using OAuth flow")]
        AuthenticationError,
    }

    /// Login using Google OAuth to enable actions on user data.
    ///
    /// # Errors
    ///
    /// Returns an error if login flow fails.
    pub fn execute() -> Result<()> {
        println!("Logging in!");
        let async_rt = runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        async_rt.block_on(request_login())?;
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
    async fn request_login() -> Result<(), LoginError> {
        let secret = yup_oauth2::ApplicationSecret::default();
        let connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .map_err(|_| LoginError::InvalidCertsError)?
            .https_only()
            .enable_all_versions()
            .build();

        let executor = hyper_util::rt::TokioExecutor::new();
        // Why legacy client here?
        let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
            yup_oauth2::client::CustomHyperClientBuilder::from(
                hyper_util::client::legacy::Client::builder(executor).build(connector),
            ),
        )
        .build()
        .await
        .map_err(|_| LoginError::AuthenticationError)?;

        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build(
                    hyper_rustls::HttpsConnectorBuilder::new()
                        .with_native_roots()
                        .map_err(|_| LoginError::InvalidCertsError)?
                        .https_or_http()
                        .enable_all_versions()
                        .build(),
                );

        // Sample functionality using login
        let hub = YouTube::new(client, auth);
        let _result = hub.videos().list(&vec!["status".into()]).doit().await; // TODO Need to do something here

        // Here the console writes a URI segment to authenticate. Prepend:
        // https://accounts.google.com/o/oauth2/v2/auth
        // Append client ID.
        // How do we do this for the user? Open browser, grab that URL, and eventually save a token
        // or something to keep logged in.

        Ok(())
    }
}
