use tokio::runtime::Runtime;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;
use futures_util::{StreamExt, SinkExt};

pub fn send_websocket_message_command() {
    let rt = Runtime::new().unwrap(); // Create the Tokio runtime

    rt.block_on(async {
        let websocket_url = ""; // Example WebSocket URL
        let message = "Hello, WebSocket!";
        if let Err(e) = send_websocket_message(websocket_url, message).await {
            println!("Error: {}", e);
        }
    });
}

pub async fn send_websocket_message(
    uri: &str,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(uri)?;

    let (mut socket, response) = connect_async(url).await?;
    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());

    // Send a message to the WebSocket server
    socket.send(Message::Text(message.to_string())).await?;
    println!("Message sent: {}", message);

    // Optionally, receive a message from the WebSocket server
    while let Some(msg) = socket.next().await {
        let msg = msg?;
        println!("Received a reply: {}", msg);
    }

    Ok(())
}
