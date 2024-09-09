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
    /// assert_eq!(person.to_json().unwrap(), *r#"{"name":"Paul"}"#);
    /// ```
    fn to_json(&self) -> Result<String>;

    fn to_json_writer<W>(&self, w: W) -> Result<()>
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
    /// let person = Person::from_json(r#"{"name":"Paul"}"#).unwrap();
    /// assert_eq!(person.name, *"Paul");
    /// ```
    fn from_json<S>(s: S) -> Result<Self>
    where
        Self: Sized,
        S: AsRef<str>;
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

    fn to_json_writer<W>(&self, w: W) -> Result<()>
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

#[cfg(test)]
mod tests {
    use super::{ErrInto, Json};
    use serde::{Deserialize, Serialize};
    use std::fs::OpenOptions;

    #[derive(Debug, Deserialize, Serialize)]
    struct TestPerson {
        name: String,
        age: u32,
    }

    #[test]
    fn test_person_to_string() {
        let person = TestPerson {
            name: "Paul".to_owned(),
            age: 63,
        };
        let s = person.to_json().unwrap();
        assert_eq!(s, *"{\"name\":\"Paul\",\"age\":63}");
    }

    #[test]
    fn from_json() {
        const TEST_STRING: &str = "{\"name\":\"Paul\",\"age\":63}";
        let person = TestPerson::from_json(TEST_STRING).unwrap();
        assert_eq!(person.name, "Paul");
        assert_eq!(person.age, 63);
    }

    #[test]
    fn from_reader() {
        const TEST_FILE: &str = "test.json";
        let person = OpenOptions::new()
            .read(true)
            .open(TEST_FILE)
            .err_into()
            .and_then(TestPerson::from_json_reader)
            .unwrap();
        assert_eq!(person.name, *"Paul Min");
        assert_eq!(person.age, 74);
    }
}
