use hyper::Uri as HyperUri;
use std::path::Path;

#[derive(Debug)]
pub struct Uri {
    hyper_uri: HyperUri,
}

impl Uri {
    pub fn new(socket: impl AsRef<Path>, path: &str) -> Self {
        let host = hex::encode(socket.as_ref().to_string_lossy().as_bytes());
        let host_str = format!("unix://{}:0{}", host, path);
        let hyper_uri: HyperUri = host_str.parse().unwrap();

        Self { hyper_uri }
    }
}

impl From<Uri> for HyperUri {
    fn from(uri: Uri) -> Self {
        uri.hyper_uri
    }
}

#[cfg(test)]
mod tests {
    use super::Uri;
    use hyper::Uri as HyperUri;

    #[test]
    fn test_unix_uri_into_hyper_uri() {
        let unix: HyperUri = Uri::new("foo.sock", "/").into();
        let expected: HyperUri = "unix://666f6f2e736f636b:0/".parse().unwrap();
        assert_eq!(unix, expected);
    }
}