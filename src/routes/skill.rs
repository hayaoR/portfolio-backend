use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Skill {
    id: usize,
    title: String,
    content: String,
}

pub async fn skills() -> Json<Vec<Skill>> {
    let skills = vec![
        Skill {
            id: 1,
            title: "Rust".to_string(),
            content: "好きな言語だが使い所が難しい。".to_string(),
        },
        Skill {
            id: 2,
            title: "Elm".to_string(),
            content: "好きな言語。ただJavaScriptとの相互作用するところが辛い。".to_string(),
        },
        Skill {
            id: 3,
            title: "Haskell".to_string(),
            content: "好きな言語。Haskell力を上げたい".to_string(),
        },
    ];

    Json(skills)
}
