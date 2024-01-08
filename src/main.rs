use std::collections::HashMap;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let mut data = HashMap::new();
    data.insert(String::from("John"), String::from("Snow"));
    data.insert(String::from("Peter"), String::from("Piper"));

    let title = String::from("Mr.");
    let client = Arc::new(Client::builder().with_title(title).build());
    println!("{:?}", &client);

    let mut handles = vec![];
    let mut data = HashMap::new();
    data.insert(String::from("John"), String::from("Snow"));
    data.insert(String::from("Peter"), String::from("Piper"));

    for (first, last) in data {
        let p_client = client.clone();
        let handle = tokio::spawn(async move { p_client.print_name(first, last).await; });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
}

#[derive(Debug)]
struct Client {
    title: String,
}

struct ClientBuilder {
    title: Option<String>,
}

impl ClientBuilder {
    fn new() -> Self {
        Self { title: None }
    }

    fn with_title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }

    fn build(&self) -> Client {
        Client {
            title: self.title.clone().unwrap(),
        }
    }
}

impl Client {
    fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    async fn print_name(&self, first: String, last: String) {
        println!("{} {} {}", self.title, first, last);
    }
}
