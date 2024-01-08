use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let mut data = HashMap::new();
    data.insert(String::from("John"), String::from("Snow"));
    data.insert(String::from("Peter"), String::from("Piper"));

    let title = String::from("Mr.");
    let client = Client::builder().with_title(title).build();
    println!("{:?}", &client);

    // General Async Pattern
    let mut handles = vec![];
    for i in 0..10 {
        let handle = tokio::spawn(async move {
            println!("Hello from task {}", i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }

    // What I'm trying to do
    let mut new_handles = vec![];
    let mut new_data = HashMap::new();
    new_data.insert(String::from("John"), String::from("Snow"));
    new_data.insert(String::from("Peter"), String::from("Piper"));

    for (first, last) in new_data {
        let handle = tokio::spawn(async move { client.print_name(first, last) });
        new_handles.push(handle);
    }
    for handle in new_handles {
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
