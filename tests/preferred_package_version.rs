#![allow(deprecated)]

use canton_api_client::apis::{configuration::Configuration, default_api};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const ROUTE: &str = "/v2/interactive-submission/preferred-package-version";

fn at(text: &str) -> jiff::Timestamp {
    text.parse().unwrap()
}

/// The query the client sent for one call with the given `vetting_valid_at`.
async fn sent_query(vetting_valid_at: Option<&str>) -> Vec<(String, String)> {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(ROUTE))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;
    let configuration = Configuration {
        base_path: server.uri(),
        ..Configuration::new()
    };

    let _ = default_api::get_v2_interactive_submission_preferred_package_version(
        &configuration,
        "splice-amulet",
        Some(vec!["alice::1220ab".to_string()]),
        vetting_valid_at.map(at),
        None,
    )
    .await;

    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 1);
    requests[0].url.query_pairs().into_owned().collect()
}

#[tokio::test]
async fn vetting_valid_at_is_sent_as_rfc_3339() {
    let query = sent_query(Some("2026-11-19T10:00:00Z")).await;
    assert!(
        query.contains(&(
            "vetting_valid_at".to_string(),
            "2026-11-19T10:00:00Z".to_string()
        )),
        "{query:?}"
    );
}

#[tokio::test]
async fn vetting_valid_at_keeps_its_fraction_in_utc() {
    let query = sent_query(Some("2026-11-19T12:00:00.123456+02:00")).await;
    assert!(
        query.contains(&(
            "vetting_valid_at".to_string(),
            "2026-11-19T10:00:00.123456Z".to_string()
        )),
        "{query:?}"
    );
}

#[tokio::test]
async fn vetting_valid_at_is_left_out_when_absent() {
    let query = sent_query(None).await;
    assert!(
        query.iter().all(|(key, _)| key != "vetting_valid_at"),
        "{query:?}"
    );
    assert!(query.contains(&("package-name".to_string(), "splice-amulet".to_string())));
}
