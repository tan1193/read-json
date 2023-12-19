use  serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String
}

#[derive(Serialize, Deserialize)]

struct Article {
    article: String,
    author: String,
    paragraphs: Vec<Paragraph>
}

fn main() {
    let json = r#"
    {
        "article": "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.",
        "author": "unknown",
        "paragraphs": [
            {
                "name": "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety."
            },
            {
                "name": "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety."
            }
        ]
    }"#;

    let article: Article = read_json_typed(json);
    println!("\n\n The name of the first paragraph is: {}", article.paragraphs[0].name);
}

fn read_json_typed<T>(json: &str) -> T
where
    T: serde::de::DeserializeOwned,
{
   let parsed: T = serde_json::from_str(json).unwrap();
   return  parsed;
}
