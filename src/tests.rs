/// tests
use crate::DateTimeFixedOffset;

#[test]
fn test_dotted_date() {
    let date = "1970.12.31";
    let test = date.parse::<DateTimeFixedOffset>();
    assert!(test.is_ok());
    assert!(test
        .unwrap()
        .0
        .to_rfc3339()
        .starts_with("1970-12-31T00:00:00"));
}

#[test]
fn test_slash_date() {
    let date = "1970/12/31";
    let test = date.parse::<DateTimeFixedOffset>();
    assert!(test.is_ok());
    assert!(test
        .unwrap()
        .0
        .to_rfc3339()
        .starts_with("1970-12-31T00:00:00"));
}

#[test]
fn test_epoch_seconds() {
    let date = "1672903639";
    let test = date.parse::<DateTimeFixedOffset>();
    assert!(test.is_ok());
    assert!(test
        .unwrap()
        .0
        .to_rfc3339()
        .starts_with("2023-01-05T07:27:19"));
}

#[test]
fn test_epoch_milliseconds() {
    let date = "1672903639123";
    let test = date.parse::<DateTimeFixedOffset>();
    assert!(test.is_ok());
    assert!(test
        .unwrap()
        .0
        .to_rfc3339()
        .starts_with("2023-01-05T07:27:19.123"));
}

#[test]
fn test_epoch_microseconds() {
    let date = "1672903639123123";
    let test = date.parse::<DateTimeFixedOffset>();
    assert!(test.is_ok());
    assert!(test
        .unwrap()
        .0
        .to_rfc3339()
        .starts_with("2023-01-05T07:27:19.123123"));
}

#[test]
fn test_epoch_nanoseconds() {
    let date = "1672903639123123123";
    let test = date.parse::<DateTimeFixedOffset>();
    assert!(test.is_ok());
    assert!(test
        .unwrap()
        .0
        .to_rfc3339()
        .starts_with("2023-01-05T07:27:19.123123123"));
}

#[test]
fn test_m_d_yyyy_h_m_s_tt() {
    let date = "8/7/2023 8:23:50 AM";
    let test = date.parse::<DateTimeFixedOffset>();
    assert!(test.is_ok());
    assert!(test
        .unwrap()
        .0
        .to_rfc3339()
        .starts_with("2023-08-07T08:23:50+02:00"));
}

#[test]
fn test_mt_d_h_m_s_ms() {
    let date = "Dec 27 18:57:47.234"; //.746";
    let test = date.parse::<DateTimeFixedOffset>();
    assert!(test.is_ok());
    let test = test.unwrap();
    eprintln!("{}", test.0.to_rfc3339());
    assert!(test
        .0
        .to_rfc3339()
        .starts_with("2024-12-27T18:57:47.234+01:00"));
}

#[test]
fn test_mt_d_yy_h_m_s() {
    let date = "01 Mar 2024  17:47:00"; //.746";
    let test = date.parse::<DateTimeFixedOffset>();
    assert!(test.is_ok());
    let test = test.unwrap();
    eprintln!("{}", test.0.to_rfc3339());
    assert!(test.0.to_rfc3339().starts_with("2024-03-01T17:47:00+01:00"));
}

#[test]
fn test_yyyy_m_d_h_m_s() {
    let date = "2024 Mar 29 18:01:18"; //.746";
    let test = date.parse::<DateTimeFixedOffset>();
    assert!(test.is_ok());
    let test = test.unwrap();
    eprintln!("{}", test.0.to_rfc3339());
    assert!(test.0.to_rfc3339().starts_with("2024-03-29T18:01:18+01:00"));
}
