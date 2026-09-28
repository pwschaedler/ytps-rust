pub mod login {
    use google_youtube3::{FieldMask, YouTube, hyper_rustls, hyper_util, yup_oauth2};
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum LoginError {
        #[error("Could not detect valid TLS certs")]
        InvalidCertsError,
    }

    /// Login using Google OAuth to enable actions on user data.
    ///
    /// # Panics
    ///
    /// Panics if login flow fails.
    pub fn execute() {
        println!("Logging in!");
        let login_res = request_login();
        assert!(login_res.is_ok(), "Unable to login.");
    }

    /// Request login flow using OAuth.
    ///
    /// Most of this right now is based on the example code from google-youtube3's documentation for
    /// constructing the user flow.
    ///
    /// # Errors
    ///
    /// Returns a `LoginError` when an HTTPS connection cannot be made.
    fn request_login() -> Result<(), LoginError> {
        let secret = yup_oauth2::ApplicationSecret::default();
        let connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .map_err(|_| LoginError::InvalidCertsError)?
            .https_only()
            .enable_all_versions()
            .build();

        let executor = hyper_util::rt::TokioExecutor::new();
        let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
            yup_oauth2::client::CustomHyperClientBuilder::from(
                hyper_util::client::legacy::Client::builder(executor).build(connector),
            ),
        )
        .build(); // TODO .await.unwrap();
        // Some questions here:
        // 1. Why legacy client?
        // 2. Need to make this async so I can await.

        // There's more to do here too.
        // https://docs.rs/google-youtube3/latest/google_youtube3/index.html

        Ok(())
    }
}
