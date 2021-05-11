#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let adaptive = std::env::var_os("ADAPTIVE").is_some();
    let url = "https://dl.fbaipublicfiles.com/fasttext/supervised-models/lid.176.bin".parse()?;

    let mut config = rustls::ClientConfig::new();
    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
    config.key_log = std::sync::Arc::new(rustls::KeyLogFile::new());
    config.root_store = rustls_native_certs::load_native_certs().unwrap();

    let mut http = hyper::client::connect::HttpConnector::new();
    http.enforce_http(false);

    let https = hyper_rustls::HttpsConnector::from((http, config));

    let client: hyper::Client<_, hyper::Body> = hyper::Client::builder()
        .http2_adaptive_window(adaptive)
        .build(https);

    let mut response = client.get(url).await?;
    hyper::body::to_bytes(response.body_mut()).await?;

    Ok(())
}
