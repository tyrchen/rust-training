use tokio::sync::{Semaphore, SemaphorePermit};

#[derive(Debug)]
pub struct HttpPool {
    remaining_clients: Semaphore,
}

#[derive(Debug)]
pub struct HttpClient<'a> {
    permit: SemaphorePermit<'a>,
}

impl<'a> HttpClient<'a> {
    pub fn new(permit: SemaphorePermit<'a>) -> Self {
        Self { permit }
    }

    /// send a get request
    pub async fn request(&self, url: &str) -> Result<String, reqwest::Error> {
        reqwest::get(url).await?.text().await
    }
}

impl Default for HttpPool {
    fn default() -> Self {
        Self::new(20)
    }
}

impl HttpPool {
    pub fn new(total: usize) -> Self {
        Self {
            remaining_clients: Semaphore::new(total),
        }
    }

    pub fn get(&self) -> Option<HttpClient<'_>> {
        match self.remaining_clients.try_acquire() {
            Ok(permit) => Some(HttpClient::new(permit)),
            Err(_) => None,
        }
    }

    pub fn remaining(&self) -> usize {
        self.remaining_clients.available_permits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pool = HttpPool::default();
        let mut clients: Vec<HttpClient> = (0..20).map(|_| pool.get().unwrap()).collect();
        println!("More clients? {:?}", pool.get());
        clients.pop();
        println!("More clients? {:?}", pool.get());
        println!("remaining clients: {}", pool.remaining());
        drop(clients);
        println!("remaining clients: {}", pool.remaining());
    }
}
