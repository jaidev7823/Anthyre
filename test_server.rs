use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting test server on localhost:1421...");
    
    let listener = TcpListener::bind("localhost:1421").await?;
    println!("✅ Server listening, try visiting http://localhost:1421/test in your browser");
    
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("📡 Connection from: {}", addr);
        
        let mut buffer = [0u8; 2048];
        let n = socket.read(&mut buffer).await?;
        let request = String::from_utf8_lossy(&buffer[..n]);
        
        println!("📥 Request:\n{}", request);
        
        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<h1>✅ Server is working!</h1>";
        socket.write_all(response.as_bytes()).await?;
        
        break; // Exit after first request
    }
    
    Ok(())
}