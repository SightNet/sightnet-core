# A fast search engine
## Example 
###### (src/example.rs)
```rust
pub fn main() {
    let mut collection = Collection::default();
    collection.push_field("title", FieldType::String);

    let corpus = vec![
        "Egyptian Cretan Phoenician aleph Semitic Greek Alpha Etruscan A Roman/Cyrillic A Boeotian 800–700 BC Greek Uncial Latin 300 AD Uncial ",
        "In algebra, the letter \"A\" along with other letters at the beginning of the alphabet is used to represent known quantities, whereas the letters at the end of the alphabet (x,y,z) are used to denote unknown quantities.",
        "The Etruscans brought the Greek alphabet to their civilization in the Italian Peninsula and left the letter unchanged. The Romans later adopted the Etruscan alphabet to write the Latin language, and the resulting letter was preserved in the Latin alphabet used to write many languages, including English.",
    ];

    for i in corpus {
        let mut doc = Document::new();
        let mut field_value = FieldValue::new();

        field_value.value_string = Some(i.to_string());

        doc.push("title", field_value);
        collection.push(doc);
    }

    collection.commit();
    println!("{:#?}", collection.search("algebra", None, None));
}
```
## Dictionary
* Field - key-value pair with type
* Document - Entry, Item, Row with fields(key-value)
* Collection - Array of documents
## Supported Field Types
* Int,
* Bool,
* String
## ...
##### After any changes, you should call function commit. It will update indexes. And you will search by new data.