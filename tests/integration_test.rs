use reqwest;
use serde_json::Value;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_hot_posts_endpoint() {
    let mock_server = wiremock::MockServer::start().await;

    // Setup mock
    Mock::given(method("GET"))
        .and(path("/hot.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "kind": "Listing",
            "data": {
                "after": "t3_test123",
                "before": null,
                "children": [
                    {
                        "kind": "t3",
                        "data": {
                            "id": "test123",
                            "name": "t3_test123",
                            "title": "Test Post",
                            "author": "testuser",
                            "subreddit": "test",
                            "subreddit_id": "t5_test",
                            "selftext": "",
                            "url": "https://example.com",
                            "domain": "example.com",
                            "permalink": "/r/test/comments/test123/",
                            "created_utc": 1234567890.0,
                            "score": 100,
                            "upvote_ratio": 0.95,
                            "num_comments": 10
                        }
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/hot.json", mock_server.uri()))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());

    let body: Value = response.json().await.unwrap();
    assert!(body["data"]["children"].is_array());
    assert_eq!(body["data"]["children"][0]["data"]["title"], "Test Post");
}

#[tokio::test]
async fn test_subreddit_about_endpoint() {
    let mock_server = wiremock::MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/r/rust/about.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "kind": "t5",
            "data": {
                "id": "2qhuy",
                "name": "t5_2qhuy",
                "display_name": "rust",
                "title": "The Rust Programming Language",
                "description": "A place for all things Rust.",
                "public_description": "A place for all things Rust.",
                "subscribers": 250000,
                "active_user_count": 500,
                "created_utc": 1284672000.0,
                "over18": false,
                "url": "/r/rust/"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/r/rust/about.json", mock_server.uri()))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());

    let body: Value = response.json().await.unwrap();
    assert_eq!(body["data"]["display_name"], "rust");
    assert_eq!(body["data"]["subscribers"], 250000);
}

#[tokio::test]
async fn test_user_about_endpoint() {
    let mock_server = wiremock::MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/user/spez/about.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "kind": "t2",
            "data": {
                "id": "1",
                "name": "spez",
                "created_utc": 1111111111.0,
                "link_karma": 100000,
                "comment_karma": 50000,
                "is_gold": false,
                "is_mod": true,
                "has_verified_email": true
            }
        })))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/user/spez/about.json", mock_server.uri()))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());

    let body: Value = response.json().await.unwrap();
    assert_eq!(body["data"]["name"], "spez");
    assert_eq!(body["data"]["link_karma"], 100000);
}

#[tokio::test]
async fn test_search_endpoint() {
    let mock_server = wiremock::MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/search.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "kind": "Listing",
            "data": {
                "after": null,
                "before": null,
                "children": [
                    {
                        "kind": "t3",
                        "data": {
                            "id": "search1",
                            "name": "t3_search1",
                            "title": "Search Result",
                            "author": "searcher",
                            "subreddit": "all",
                            "subreddit_id": "t5_all",
                            "selftext": "",
                            "url": "https://example.com",
                            "permalink": "/r/all/comments/search1/",
                            "created_utc": 1234567890.0,
                            "score": 50,
                            "num_comments": 5
                        }
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/search.json?q=rust", mock_server.uri()))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());

    let body: Value = response.json().await.unwrap();
    assert!(body["data"]["children"].is_array());
}
