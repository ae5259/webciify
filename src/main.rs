mod structs;

use axum::{extract::Query, routing::get, Json, Router};
use image::imageops::FilterType;
use structs::{AsciiResponse, QueryParams};
use tapciify::{
    AsciiArtConverter, AsciiArtConverterOptions, CustomRatioResize, DEFAULT_FONT_RATIO,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(send_ascii));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn send_ascii(query: Query<QueryParams>) -> Json<AsciiResponse> {
    let img_bytes = reqwest::get(&query.image)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let image = image::load_from_memory(&img_bytes).unwrap();

    let ascii_image = image
        .resize_custom_ratio(
            Some(query.width),
            None,
            DEFAULT_FONT_RATIO,
            FilterType::Triangle,
        )
        .ascii_art(&AsciiArtConverterOptions::default())
        .unwrap();

    Json(AsciiResponse {
        message: ascii_image.to_string(),
    })
}
