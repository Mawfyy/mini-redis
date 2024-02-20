use mini_redis::client;

#[tokio::main]
async fn main() {
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    if let Ok(frame) = client.set("hello", "World".into()).await {
        println!("{:?}", frame);
    } else {
        println!("Something");
    }

    // Get key "hello"
    let result = client.get("hello").await.unwrap();
    println!("got value from the server; result={:?}", result);
}
