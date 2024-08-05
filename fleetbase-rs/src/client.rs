use reqwest::{Client, Error, Method, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use std::env;

//BASE_URL comes from env file
const BASE_URL = env::var("FLEETBASE_API_URL").expect("FLEETBASE_API_URL must be set");

pub struct FleetbaseClient {
    client: Client,
    token: String,
}

impl FleetbaseClient {
    pub fn new(token: String) -> Self {
        Self::new_with_base_url(token, BASE_URL.to_string())
    }

    pub fn new_with_base_url(token: String, base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        FleetbaseClient { client, token, base_url }
    }

    pub async fn request<T, U>(
        &self,
        method: Method,
        endpoint: Endpoint,
        body: Option<&T>,
    ) -> Result<U, Error>
    where
        T: Serialize + ?Sized,
        U: DeserializeOwned,
    {
        let url = format!("{}{}", BASE_URL, endpoint.to_string());
        let mut request_builder = self
            .client
            .request(method, &url)
            .header("Authorization", format!("Bearer {}", self.token));

        if let Some(body) = body {
            request_builder = request_builder.json(body);
        }

        let response = request_builder.send().await?;
        response.json::<U>().await
    }

    pub async fn get<U>(&self, endpoint: Endpoint) -> Result<U, Error>
    where
        U: DeserializeOwned,
    {
        self.request::<(), U>(Method::GET, endpoint, None).await
    }

    pub async fn post<T, U>(&self, endpoint: Endpoint, body: &T) -> Result<U, Error>
    where
        T: Serialize + ?Sized,
        U: DeserializeOwned,
    {
        self.request::<T, U>(Method::POST, endpoint, Some(body))
            .await
    }

    pub async fn put<T, U>(&self, endpoint: Endpoint, body: &T) -> Result<U, Error>
    where
        T: Serialize + ?Sized,
        U: DeserializeOwned,
    {
        self.request::<T, U>(Method::PUT, endpoint, Some(body))
            .await
    }

    pub async fn delete<U>(&self, endpoint: Endpoint) -> Result<U, Error>
    where
        U: DeserializeOwned,
    {
        self.request::<(), U>(Method::DELETE, endpoint, None).await
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestResponse {
        message: String,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestRequest {
        data: String,
    }

    fn create_client() -> FleetbaseClient {
        FleetbaseClient::new("test_token".to_string())
    }

    #[tokio::test]
    async fn test_get_request() {
        let mut server = mockito::Server::new();
        let client = create_client();

        let mock = server.mock("GET", "/service-quotes")
            .match_header("Authorization", "Bearer test_token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message": "Success"}"#)
            .create();

        let response: TestResponse = client.get(Endpoint::ServiceQuotes(ServiceQuotes::ServiceQuotes)).await.unwrap();

        mock.assert();
        assert_eq!(response, TestResponse { message: "Success".to_string() });
    }

    #[tokio::test]
    async fn test_post_request() {
        let mut server = mockito::Server::new();
        let client = create_client();

        let request_body = TestRequest { data: "test".to_string() };
        let mock = server.mock("POST", "/orders")
            .match_header("Authorization", "Bearer test_token")
            .match_body(r#"{"data":"test"}"#)
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message": "Created"}"#)
            .create();

        let response: TestResponse = client.post(Endpoint::Orders(Orders::Orders), &request_body).await.unwrap();

        mock.assert();
        assert_eq!(response, TestResponse { message: "Created".to_string() });
    }

    #[tokio::test]
    async fn test_put_request() {
        let mut server = mockito::Server::new();
        let client = create_client();

        let request_body = TestRequest { data: "updated".to_string() };
        let mock = server.mock("PUT", "/orders/123")
            .match_header("Authorization", "Bearer test_token")
            .match_body(r#"{"data":"updated"}"#)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message": "Updated"}"#)
            .create();

        let response: TestResponse = client.put(Endpoint::Orders(Orders::OrdersById("123".to_string())), &request_body).await.unwrap();

        mock.assert();
        assert_eq!(response, TestResponse { message: "Updated".to_string() });
    }

    #[tokio::test]
    async fn test_delete_request() {
        let mut server = mockito::Server::new();
        let client = create_client();

        let mock = server.mock("DELETE", "/orders/123")
            .match_header("Authorization", "Bearer test_token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message": "Deleted"}"#)
            .create();

        let response: TestResponse = client.delete(Endpoint::Orders(Orders::OrdersById("123".to_string()))).await.unwrap();

        mock.assert();
        assert_eq!(response, TestResponse { message: "Deleted".to_string() });
    }

    #[tokio::test]
    async fn test_error_handling() {
        let mut server = mockito::Server::new();
        let client = create_client();

        let mock = server.mock("GET", "/non-existent")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error": "Not Found"}"#)
            .create();

        let result: Result<TestResponse, _> = client.get(Endpoint::Places(Places::PlacesById("non-existent".to_string()))).await;

        mock.assert();
        assert!(result.is_err());
    }
}
