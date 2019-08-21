//! The bases crate base on RFC4648
//!
//! bases 库是基于规范文档 RFC4648 编写的库
//!
//! This crate is very simple
//!
//! 这个库非常简单易用
//!
//! Usually, we use encode () and decode (). But if Base64 is used for URLs, encode_url() and decode_url() should be used.
//!
//! 通常情况下，我们会用到 encode() 和 decode()。但如果 Base64 被用于 URL，那么就应该使用 encode_url() 和 decode_url()
//!
//! **encode**
//!
//! Given a string, return an encode result. This is the inverse of decode().
//!
//! **decode**
//!
//! Given a String, return a decode result, it's original of encode(). This is the inverse of decode().
//!
//! **encode_url**
//!
//! Like encode(), this is for url.
//!
//! **decode_url**
//!
//! Like decode(), this is for url
//!
//! # encode()
//! &str to String, "Bruce Done say: Hi,I'm frank。" -> "QnJ1Y2UgRG9uZSBzYXk6IEhpLEknbSBmcmFua+OAgg=="
//!
//! Example
//!
//!     use bases64::bases::Bases;
//!     let bases = Bases{};
//!     let res = bases.encode("Bruce Done say: Hi,I'm frank。");
//!     assert_eq!(res, String::from("QnJ1Y2UgRG9uZSBzYXk6IEhpLEknbSBmcmFua+OAgg=="));
//!
//! # decode()
//!
//! String to String, "QnJ1Y2UgRG9uZSBzYXk6IEhpLEknbSBmcmFua+OAgg==" -> "Bruce Done say: Hi,I'm frank。"
//!
//! Example
//!
//!     use bases64::bases::Bases;
//!     let bases = Bases{};
//!     let res = bases.decode(String::from("QnJ1Y2UgRG9uZSBzYXk6IEhpLEknbSBmcmFua+OAgg=="));
//!     assert_eq!(res, String::from("Bruce Done say: Hi,I'm frank。"));
//!
//! # encode_url()
//!
//! For url.
//!
//! &str to String, "Bruce Done say: Hi,I'm frank。" -> "QnJ1Y2UgRG9uZSBzYXk6IEhpLEknbSBmcmFua-OAgg=="
//!
//! Example
//!
//!     use bases64::bases::Bases;
//!     let bases = Bases{};
//!     let res = bases.encode_url("Bruce Done say: Hi,I'm frank。");
//!     assert_eq!(res, String::from("QnJ1Y2UgRG9uZSBzYXk6IEhpLEknbSBmcmFua-OAgg=="));
//!
//! # decode_url()
//!
//! For url.
//!
//! Example
//!
//!     use bases64::bases::Bases;
//!     let bases = Bases{};
//!     let res = bases.decode_url(String::from("QnJ1Y2UgRG9uZSBzYXk6IEhpLEknbSBmcmFua-OAgg=="));
//!     assert_eq!(res, String::from("Bruce Done say: Hi,I'm frank。"));
//!

pub mod bases;


