use url::Url;

/// Returns if a URL is a valid URL string supported by the library
#[must_use]
pub fn url_is_valid(url: &str) -> bool {
    Url::parse(url).map_or(false, |url| {
        // Supported URL schemes
        ["http", "https", "ftp", "file"].contains(&url.scheme())
    })
}
