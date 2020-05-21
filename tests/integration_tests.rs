extern crate limitador;

use limitador::errors::LimitadorError;
use limitador::limit::Limit;
use limitador::RateLimiter;
use std::collections::{HashMap, HashSet};

#[test]
fn add_a_limit() {
    let limit = Limit::new(
        "test_namespace",
        10,
        60,
        vec!["req.method == GET"],
        vec!["req.method", "app_id"],
    );

    let mut rate_limiter = RateLimiter::new();
    rate_limiter.add_limit(limit.clone()).unwrap();

    let mut expected_result = HashSet::new();
    expected_result.insert(limit);

    assert_eq!(
        rate_limiter.get_limits("test_namespace").unwrap(),
        expected_result
    )
}

#[test]
fn add_several_limits_in_the_same_namespace() {
    let namespace = "test_namespace";

    let limit_1 = Limit::new(
        namespace,
        10,
        60,
        vec!["req.method == POST"],
        vec!["req.method", "app_id"],
    );

    let limit_2 = Limit::new(
        namespace,
        5,
        60,
        vec!["req.method == GET"],
        vec!["req.method", "app_id"],
    );

    let mut rate_limiter = RateLimiter::new();
    rate_limiter.add_limit(limit_1.clone()).unwrap();
    rate_limiter.add_limit(limit_2.clone()).unwrap();

    let mut expected_result = HashSet::new();
    expected_result.insert(limit_1);
    expected_result.insert(limit_2);

    assert_eq!(rate_limiter.get_limits(namespace).unwrap(), expected_result)
}

#[test]
fn rate_limited() {
    let max_hits = 3;

    let limit = Limit::new(
        "test_namespace",
        max_hits,
        60,
        vec!["req.method == GET"],
        vec!["req.method", "app_id"],
    );

    let mut rate_limiter = RateLimiter::new();
    rate_limiter.add_limit(limit.clone()).unwrap();

    let mut values: HashMap<String, String> = HashMap::new();
    values.insert("namespace".to_string(), "test_namespace".to_string());
    values.insert("req.method".to_string(), "GET".to_string());
    values.insert("app_id".to_string(), "test_app_id".to_string());

    for _ in 0..max_hits {
        assert_eq!(false, rate_limiter.is_rate_limited(&values, 1).unwrap());
        rate_limiter.update_counters(&values, 1).unwrap();
    }
    assert_eq!(true, rate_limiter.is_rate_limited(&values, 1).unwrap());
}

#[test]
fn rate_limited_returns_err_when_no_namespace() {
    let rate_limiter = RateLimiter::new();

    let mut values: HashMap<String, String> = HashMap::new();
    values.insert("some_key".to_string(), "some_value".to_string());

    assert_eq!(
        rate_limiter.is_rate_limited(&values, 1).err().unwrap(),
        LimitadorError::MissingNamespace
    );
}

#[test]
fn update_counters_returns_err_when_no_namespace() {
    let mut rate_limiter = RateLimiter::new();

    let mut values: HashMap<String, String> = HashMap::new();
    values.insert("some_key".to_string(), "some_value".to_string());

    assert_eq!(
        rate_limiter.update_counters(&values, 1).err().unwrap(),
        LimitadorError::MissingNamespace
    );
}