# AsyncAPI Parser for rust

AsyncAPI parser for rust. This project aims to implement all the functionalities in [asyncapi/parser-js](https://github.com/asyncapi/parser-js).


### Usage 
The package exposes two functions:
- `validate()` - function that validates the passed AsyncAPI document and returns a result object with a vector of all the errors in the document. 
- `parse()` - Not implemented yet.


### Examples 

#### Example of validating spec file. 
```rust
use parser_rs::validate;

fn main() {
    let result = validate("./asyncapi.yml");
    match result {
        Ok(()) => println!("Sucessfully Validated"),
        Err(errors) => {
            for error in errors {
                println!("{}", error);
            }
        }
    }
}
```

