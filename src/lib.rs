use std::io::{Read, Write};

use error::Error;
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

pub trait Json {
    fn to_json(&self) -> Result<String>;

    fn to_json_writer<W>(&self, w: W) -> Result<()>
    where
        W: Write;

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

    #[derive(Deserialize, Serialize)]
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
