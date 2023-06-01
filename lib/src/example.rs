use sightnet_core::collection::Collection;
use sightnet_core::document::Document;
use sightnet_core::field::{FieldType, FieldValue};
use sightnet_core::file::File;

pub fn main() {
    let mut collection = Collection::new();
    collection.push_field("title", FieldType::String);

    let corpus = vec![
        "Egyptian Cretan Phoenician aleph Semitic Greek Alpha Etruscan A Roman/Cyrillic A Boeotian 800–700 BC Greek Uncial Latin 300 AD Uncial ",
        "In algebra, the letter \"A\" along with other letters at the beginning of the alphabet is used to represent known quantities, whereas the letters at the end of the alphabet (x,y,z) are used to denote unknown quantities.",
        "The Etruscans brought the Greek alphabet to their civilization in the Italian Peninsula and left the letter unchanged. The Romans later adopted the Etruscan alphabet to write the Latin language, and the resulting letter was preserved in the Latin alphabet used to write many languages, including English.",
    ];

    for i in corpus {
        let mut doc = Document::new();
        doc.push("title", FieldValue::from(i.to_string()));
        collection.push(doc, None);
    }

    collection.commit();
    println!("{:#?}", collection.search("algebra", None, None));
    File::save(&collection, "out.bin").unwrap();
}
