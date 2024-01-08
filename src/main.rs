use anyhow::{Ok, Result};
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
        let handle = tokio::spawn(async move {
            p_client.print_name(first, last).await;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }

    // Fetch data from multiple URLs.
    let mut url_handles = vec![];
    let urls = vec![
        "https://jsonplaceholder.typicode.com/todos/1",
        "https://jsonplaceholder.typicode.com/todos/2",
        "https://jsonplaceholder.typicode.com/todos/3",
        "https://jsonplaceholder.typicode.com/todos/4",
        "https://jsonplaceholder.typicode.com/todos/5",
    ];

    for url in urls {
        let p_client = client.clone();
        let url_handle = tokio::spawn(async move {
            p_client.get_data(url.to_string()).unwrap();
        });
        url_handles.push(url_handle);
    }
    for handle in url_handles {
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

    fn get_data(&self, url: String) -> Result<String> {
        let body: String = ureq::get(url.as_str()).call()?.into_string()?;
        println!("{:?}", body);
        Ok(body)
    }
}
