use poem_openapi::ApiResponse;


// Have a different ApiResponse for each request
/*
#[derive(ApiResponse)]
pub enum Redirect {
    /// Permanent GET redirect
    #[oai(status = "301")]
    MovedPermanently(#[oai(header = "Location")] String),
    /// Permanent Non GET redirect
    #[oai(status = "308")]
    PermanentlyRedirect(#[oai(header = "Location")] String),

    /// Temporary redirect, method not preserved
    #[oai(status = "302")]
    Found(#[oai(header = "Location")] String),
    /// Source moved (used after PUT or POST)
    #[oai(status = "303")]
    SeeOther(#[oai(header = "Location")] String),
    /// Temporary redirect, method preserved
    #[oai(status = "307")]
    TemporaryRedirect(#[oai(header = "Location")] String),

    /// Many choices for redirects
    #[oai(status = "300")]
    ManyChoices(#[oai(header = "Location")] String),
    /// Indicate that the redirect cache is still fresh
    #[oai(status = "304")]
    NotModified(#[oai(header = "Location")] String),
}

*/
