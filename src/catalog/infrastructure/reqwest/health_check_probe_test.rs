use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use crate::catalog::{
    application::ports::HealthCheckProbe, infrastructure::reqwest::ReqwestHealthCheckProbe,
};

fn respond_with(response: &'static [u8]) -> (String, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut request = [0; 1024];
        assert!(stream.read(&mut request).unwrap() > 0);
        write_response(stream, response);
    });
    (format!("http://{address}"), server)
}

fn write_response(mut stream: TcpStream, response: &[u8]) {
    stream.write_all(response).unwrap();
}

#[tokio::test]
async fn probe_returns_the_response_status_without_following_redirects() {
    let probe = ReqwestHealthCheckProbe::new().unwrap();
    let (url, server) = respond_with(
        b"HTTP/1.1 302 Found\r\nLocation: http://127.0.0.1:9/redirected\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
    );

    let status = probe.status_code(&url, false, Duration::from_secs(2)).await;

    server.join().unwrap();
    assert_eq!(status, Some(302));
}

#[tokio::test]
async fn probe_returns_none_when_the_endpoint_cannot_be_reached() {
    let probe = ReqwestHealthCheckProbe::new().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);

    let status = probe
        .status_code(&format!("http://{address}"), false, Duration::from_secs(2))
        .await;

    assert_eq!(status, None);
}
