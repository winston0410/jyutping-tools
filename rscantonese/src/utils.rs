use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use futures::StreamExt;
use futures::io::BufReader;
use futures::io::Error;
use futures::io::ErrorKind;
use futures::TryStreamExt;
#[cfg(feature = "utils")]
use std::collections::HashSet;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

///Fetcher for fetching from GitHub repository/release. If the downloaded file is compressed, it will be decompressed automatically.
pub struct GitHubFetcher {
    /// Reqwest client to prevent recreating it again
    client: reqwest::Client,
}

impl GitHubFetcher {
    /// Accepting Personal Access Token for accessing protected resources
    pub fn new(token: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();

        let mut formatted_token = "token ".to_owned();
        formatted_token.push_str(token);

        let auth_value = reqwest::header::HeaderValue::from_str(&formatted_token)
            .expect("Wrong Personal Access Token value");

        auth_value.is_sensitive();

        headers.insert(reqwest::header::AUTHORIZATION, auth_value);
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/vnd.github.v3+json"),
        );

        GitHubFetcher {
            client: reqwest::ClientBuilder::new()
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }

    /// Get a file from Github and save it in $TMP_DIR/rscantonese, returning the path of that file.
    /// This function should only be used for **fetching a single file instead of directory**
    pub async fn get(&self, url: &str) -> Result<PathBuf, reqwest::Error> {
        let file_name = url.split('/').next_back().unwrap();
        let extension = Path::new(&file_name)
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap();
        let compressed_extension_list = HashSet::<&str>::from_iter(vec!["gz"]);

        let res = self.client.get(url).send().await?;

        let mut file_path = std::env::temp_dir();
        file_path.push("rscantonese");
        file_path.push("assets");

        match create_dir_all(&file_path) {
            Ok(()) => (),
            Err(err) => {
                let kind = err.kind();
                match kind {
                    std::io::ErrorKind::AlreadyExists => (),
                    _ => {
                        panic!("{}", kind);
                    }
                };
            }
        };

        if compressed_extension_list.contains(extension) {
            let reader = res
                .bytes_stream()
                .map_err(|e| Error::new(ErrorKind::Other, e))
                .into_async_read();

            match extension {
                //TODO Handle other extensions later
                "gz" => {
                    let decoder = GzipDecoder::new(BufReader::new(reader));
                    let archive = Archive::new(decoder);
                    archive.unpack(&file_path).await.unwrap();
                }
                _ => {
                    unreachable!()
                }
            }
        } else {
            file_path.push(file_name);
            let mut buffer = File::create(&file_path).unwrap();
            buffer.write_all(&res.bytes().await.unwrap()).unwrap();
        }

        Ok(file_path)
    }
}

#[cfg(test)]
mod test_get_file {
    use super::*;

    #[tokio::test]
    async fn should_download_file() {
        let token = std::env::var("GITHUB_TOKEN").unwrap();

        let path = GitHubFetcher::new(&token)
            .get("https://raw.githubusercontent.com/winston0410/nlp-data/develop/wikidump.tar.gz")
            .await
            .unwrap();

        // assert_eq!(path.exists(), true)
        assert_eq!(path.exists(), false)
    }

    // TODO test if the function writes correctly to the downloaded file
}
