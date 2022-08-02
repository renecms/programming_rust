#![allow(dead_code)]

use async_std::io::prelude::*;
use async_std::net;

pub async fn many_requests_surf(urls: &[String]) -> Vec<Result<String, surf::Error>> {
    let client = surf::client();

    let mut handles = vec![];
    for url in urls {
        let request = client.get(&url).recv_string();
        handles.push(async_std::task::spawn(request));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }
    results
}

pub async fn many_requests_local(
    requests: Vec<(String, u16, String)>,
) -> Vec<std::io::Result<String>> {
    use async_std::task;

    let mut handles = vec![];

    for (host, port, path) in requests {
        handles.push(task::spawn_local(async move {
            cheapo_request(&host, port, &path).await
        }))
    }

    let mut results = vec![];

    for handle in handles {
        results.push(handle.await);
    }
    results
}

pub async fn many_requests_local_wrapper(
    requests: Vec<(String, u16, String)>,
) -> Vec<std::io::Result<String>> {
    use async_std::task;

    let mut handles = vec![];

    for (host, port, path) in requests {
        handles.push(task::spawn_local(async move {
            cheapo_request(&host, port, &path).await
        }))
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }
    results
}

pub async fn many_requests(requests: Vec<(String, u16, String)>) -> Vec<std::io::Result<String>> {
    use async_std::task;

    let mut handles = vec![];

    for (host, port, path) in requests {
        handles.push(task::spawn(async move {
            cheapo_request(&host, port, &path).await
        }))
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }
    results
}

// wrapper for cheapo_request
async fn cheapo_owning_request(host: String, port: u16, path: String) -> std::io::Result<String> {
    cheapo_request(&host, port, &path).await
}

async fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    let mut socket = net::TcpStream::connect((host, port)).await?;

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response).await?;

    Ok(response)
}

pub fn print_collection(input: impl Iterator<Item = Result<String, impl std::fmt::Display>>) {
    for result in input {
        match result {
            Ok(response) => println!("{}", response),
            Err(err) => eprintln!("error: {}", err),
        }
    }
}
