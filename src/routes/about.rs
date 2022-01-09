use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct About {
    text: String,
}

pub async fn about() -> Json<About> {
    let about = About {
        text: "システム開発会社ではたらくしがない人間。ある程度SEとして経験を積んだあとはITエンジニアとして働いてみたいが適性があまり無い気がする".to_string(),
    };

    Json(about)
}
