use std::collections::HashMap;

use crate::{Match, Mock, Request};
use http_types::headers::{HeaderName, HeaderValue, HeaderValues};
use http_types::headers::{CONNECTION, UPGRADE};

pub struct WsMock {
    inner: Mock,
}

struct WebsocketMatcher;

impl Match for WebsocketMatcher {
    fn matches(&self, request: &Request) -> bool {
        is_upgrade_request(request)
    }
}

// impl WsMock {
//     pub fn at<T: Into<String>>(path: T) -> Self {
//         let inner = Mock::given
//     pub fn given<M: 'static + Match>(matcher: M) -> MockBuilder {
//         MockBuilder {
//             matchers: vec![Matcher(Box::new(matcher))],
//         }
//     }
//         Self { inner }
//     }
// }

// pub struct MockBuilder {
//     path: String,
// }

fn is_upgrade_request(req: &Request) -> bool {
    is_in_header(&req.headers, CONNECTION, "Upgrade")
        && is_in_header(&req.headers, UPGRADE, "websocket")
        && is_in_header(&req.headers, sec_websocket_version(), "13")
}

fn is_in_header(
    headers: &HashMap<HeaderName, HeaderValues>,
    header: HeaderName,
    value: &str,
) -> bool {
    let Some(values) = headers.get(&header) else {
        return false;
    };
    for val in values {
        if val.as_str().eq_ignore_ascii_case(value) {
            return true;
        }
    }
    false
}

fn sec_websocket_version() -> HeaderName {
    HeaderName::from_string("Sec-Websocket-Version".into()).unwrap()
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, str::FromStr as _};

    use super::{is_upgrade_request, sec_websocket_version};
    use crate::Request;
    use http_types::{
        headers::{HeaderValue, HeaderValues, CONNECTION, UPGRADE},
        Method, Url,
    };
    fn make_upgrade_request() -> Request {
        let mut headers = HashMap::new();
        headers.insert(
            CONNECTION,
            HeaderValues::from(vec![HeaderValue::from_str("Upgrade").unwrap()]),
        );
        headers.insert(
            UPGRADE,
            HeaderValues::from(vec![HeaderValue::from_str("websocket").unwrap()]),
        );
        headers.insert(
            sec_websocket_version(),
            HeaderValues::from(vec![HeaderValue::from_str("13").unwrap()]),
        );
        Request {
            url: Url::parse("ws://127.0.0.1:8080").unwrap(),
            method: Method::Get,
            headers,
            body: vec![],
        }
    }
    #[test]
    fn upgrade_request_is_correctly_detected() {
        let req = make_upgrade_request();
        assert!(is_upgrade_request(&req));
    }
    #[test]
    fn request_without_connection_header_is_not_an_upgrade_request() {
        let mut req = make_upgrade_request();
        req.headers.remove(&UPGRADE);
        assert!(!is_upgrade_request(&req));
    }
    #[test]
    fn request_without_upgrade_header_is_not_an_upgrade_request() {
        let mut req = make_upgrade_request();
        req.headers.remove(&CONNECTION);
        assert!(!is_upgrade_request(&req));
    }
    #[test]
    fn request_without_websocket_version_header_is_not_an_upgrade_request() {
        let mut req = make_upgrade_request();
        req.headers.remove(&sec_websocket_version());
        assert!(!is_upgrade_request(&req));
    }
}
