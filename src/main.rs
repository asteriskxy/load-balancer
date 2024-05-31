use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let initial_port = 8080;
    if let Some(port) = find_available_port(initial_port).await {
        let server_addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&server_addr)
            .await
            .expect("Failed to bind to address");
        println!("Server listening on {}", server_addr);

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                handle_client(socket).await;
            });
        }
    } else {
        println!("No available ports found");
    }
}

async fn find_available_port(initial_port: u16) -> Option<u16> {
    let mut port = initial_port;
    loop {
        match TcpListener::bind(format!("127.0.0.1:{}", port)).await {
            Ok(_) => return Some(port),
            Err(_) => {
                port = port.checked_add(1)?;
            }
        }
    }
}

async fn handle_client(mut socket: TcpStream) {
    let mut buffer = [0; 1024];
    match socket.read(&mut buffer).await {
        Ok(n) => {
            let request = String::from_utf8_lossy(&buffer[..n]);
            println!("Received: {}", request);

            let response = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello, World!";
            socket.write_all(response.as_bytes()).await.unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
