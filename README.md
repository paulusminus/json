![`build-badge`]
[![`mit-badge`]](https://opensource.org/licenses/MIT)

Library crate that publishes a Json trait that is used to
serialize to or deserialize from json.

## Example

```no_run
use serde::{Deserialize, Serialize};
use json::Json;
 
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Person {
   name: String,
}
 
let person = Person { name: "Paul".to_owned() };
let s = person.to_json().unwrap();
assert_eq!(&s, r#"{"name":"Paul"}"#);

let person_copy = Person::from_json(&s).unwrap();
assert_eq!(person, person_copy);
```

[`build-badge`]: https://github.com/paulusminus/json/actions/workflows/rust.yml/badge.svg
[`mit-badge`]: https://img.shields.io/badge/License-MIT-yellow.svg
