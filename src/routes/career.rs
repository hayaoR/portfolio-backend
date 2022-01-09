use axum::Json;
use chrono::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct Career {
    id: usize,
    name: String,
    years_from: String,
    years_to: String,
    description: String,
}

pub async fn careers() -> Json<Vec<Career>> {
    let careers = vec![
        Career {
            id: 1,
            name: "ピーマン大学院".to_string(),
            years_from: Utc.ymd(2019, 4, 1).format("%Y/%m").to_string(),
            years_to: Utc.ymd(2021, 3, 31).format("%Y/%m").to_string(),
            description: "ピーマンの栽培法の研究".to_string(),
        },
        Career {
            id: 2,
            name: "ピーマンシステムズ".to_string(),
            years_from: Utc.ymd(2021, 4, 1).format("%Y/%m").to_string(),
            years_to: "".to_string(),
            description: "ピーマン栽培管理システムの構築PJに従事".to_string(),
        },
    ];

    Json(careers)
}
