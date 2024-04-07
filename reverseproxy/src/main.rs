use std::{
    fs::File,
    io::{self, BufReader},
};

use hyper::{
    server::{conn::AddrIncoming, accept::Accept},
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use hyper_rustls::TlsAcceptor;
use rustls::{Certificate, PrivateKey, ServerConfig};

#[tokio::main]
async fn main() {
    let cfg = load_key_pair("./cert.pem", "./key.pem");
    let addr = format!("127.0.0.1:{}", 8080).parse().unwrap();
    //get_incomming(true);
    let incoming = AddrIncoming::bind(&addr).unwrap();
    if true {
        let acceptor = TlsAcceptor::builder()
            .with_tls_config(cfg)
            .with_all_versions_alpn()
            .with_incoming(incoming);

        let service = make_service_fn(|_| async { Ok::<_, io::Error>(service_fn(echo)) });
        let server = Server::builder(acceptor).serve(service);
        server.await.unwrap();
    } else {
        let service = make_service_fn(|_| async { Ok::<_, io::Error>(service_fn(echo)) });
        let server = Server::builder(incoming).serve(service);
        server.await.unwrap();
    }
}

fn get_incomming(tls: bool) -> either::Either<AddrIncoming, TlsAcceptor> {
    let addr = format!("127.0.0.1:{}", 8080).parse().unwrap();
    let incoming = AddrIncoming::bind(&addr).unwrap();
    if !tls {
        return either::Left(incoming);
    }
    let cfg = load_key_pair("./cert.pem", "./key.pem");
    let acceptor = TlsAcceptor::builder()
        .with_tls_config(cfg)
        .with_all_versions_alpn()
        .with_incoming(incoming);
    either::Right(acceptor)
}

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        // Help route.
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POST /echo\n");
        }
        // Echo service route.
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        }
        // Catch-all 404.
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };
    Ok(response)
}

fn load_key_pair(cert_path: &str, key_path: &str) -> ServerConfig {
    let cert = load_certificates_from_pem(cert_path).unwrap();
    let key = load_private_key_from_file(key_path).unwrap();

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert, key)
        .unwrap();
    config
}

fn load_private_key_from_file(path: &str) -> Result<PrivateKey, Box<dyn std::error::Error>> {
    let key_file = File::open(path).expect("cannot open key file");
    let mut key_reader = BufReader::new(key_file);
    let mut keys =
        rustls_pemfile::pkcs8_private_keys(&mut key_reader).expect("cannot read key file");
    match keys.len() {
        0 => Err(format!("No PKCS8-encoded private key found in {path}").into()),
        1 => Ok(PrivateKey(keys.remove(0))),
        _ => Err(format!("More than one PKCS8-encoded private key found in {path}").into()),
    }
}

fn load_certificates_from_pem(path: &str) -> std::io::Result<Vec<Certificate>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let certs = rustls_pemfile::certs(&mut reader)?;

    Ok(certs.into_iter().map(Certificate).collect())
}
