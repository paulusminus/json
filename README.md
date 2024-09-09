# json

Library crate that publishes a Json trait that can be used to serialize to or deserialize from json.

## Example

```no_run
use serde::{Deserialize, Serialize};
use json::Json;
 
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Person {
   name: String,
}
 
let person = Person { name: "Paul".to_owned() };
assert_eq!(person.to_json().unwrap(), *r#"{"name":"Paul"}"#);

let person_copy = Person::from_json(person.to_json().unwrap()).unwrap();
assert_eq!(person, person_copy);
```