#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use std::io::{Read, Write};

pub use error::Error;
use serde::{de::DeserializeOwned, Serialize};
type Result<T, E = Error> = std::result::Result<T, E>;

mod error;

trait ErrInto<T> {
    fn err_into(self) -> Result<T>;
}

impl<T, E: Into<Error>> ErrInto<T> for Result<T, E> {
    fn err_into(self) -> Result<T> {
        self.map_err(Into::into)
    }
}

/// Methods to serialize to or deserialize from json
pub trait Json {
    /// Convert a Serializable object to a Json String
    ///
    /// Example
    ///
    /// ```
    /// use serde::{Deserialize, Serialize};
    /// use json::Json;
    ///
    /// #[derive(Deserialize, Serialize)]
    /// struct Person {
    ///   name: String,
    /// }
    ///
    /// let person = Person { name: "Paul".to_owned() };
    /// let s = person.to_json().unwrap();
    /// assert_eq!(&s, r#"{"name":"Paul"}"#);
    /// ```
    fn to_json(&self) -> Result<String>;

    /// Write a serializable object
    ///
    /// ```
    /// use std::io::{Write, stdout};
    /// use serde::{Deserialize, Serialize};
    /// use json::Json;
    ///
    /// #[derive(Deserialize, Serialize)]
    /// struct Person {
    ///     name: String
    /// }
    ///
    /// let person = Person { name: "Paul".to_owned() };
    /// let mut v = Vec::<u8>::new();
    /// person.to_json_writer(&mut v).unwrap();
    /// v.flush().unwrap();
    /// assert_eq!(std::str::from_utf8(v.as_slice()).unwrap(), r#"{"name":"Paul"}"#);
    /// ```
    fn to_json_writer<W>(&self, w: &mut W) -> Result<()>
    where
        W: Write;

    /// Convert a String to a deserializeable object
    ///
    /// Example
    ///
    /// ```
    /// use serde::{Deserialize, Serialize};
    /// use json::Json;
    ///
    /// #[derive(Deserialize, Serialize)]
    /// struct Person {
    ///     name: String
    /// }
    ///
    /// let JSON_STRING: &str = r#"{"name":"Paul"}"#;
    /// let person = Person::from_json(JSON_STRING).unwrap();
    /// assert_eq!(person.name, *"Paul");
    /// ```
    fn from_json<S>(s: S) -> Result<Self>
    where
        Self: Sized,
        S: AsRef<str>;

    /// Read a deserializeable object
    ///
    /// Example
    ///
    /// ```
    /// use serde::{Deserialize, Serialize};
    /// use json::Json;
    ///
    /// #[derive(Deserialize, Serialize)]
    /// struct Person {
    ///     name: String
    /// }
    ///
    /// let JSON_STRING: &str = r#"{"name":"Paul"}"#;
    /// let person = Person::from_json_reader(JSON_STRING.as_bytes()).unwrap();
    /// assert_eq!(person.name, *"Paul");
    /// ```
    fn from_json_reader<R>(r: R) -> Result<Self>
    where
        Self: Sized,
        R: Read;
}

impl<T> Json for T
where
    T: Serialize + DeserializeOwned,
{
    fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).err_into()
    }

    fn to_json_writer<W>(&self, w: &mut W) -> Result<()>
    where
        W: Write,
    {
        serde_json::to_writer(w, self).err_into()
    }

    fn from_json<S>(s: S) -> Result<Self>
    where
        Self: Sized,
        S: AsRef<str>,
    {
        serde_json::from_str::<T>(s.as_ref()).err_into()
    }

    fn from_json_reader<R>(r: R) -> Result<Self>
    where
        Self: Sized,
        R: Read,
    {
        serde_json::from_reader::<_, T>(r).err_into()
    }
}

