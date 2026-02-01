use crate::app::AppState;
use axum::Json;
use axum::extract::State;
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Response {
    success: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SeriesAdd {
    id: String,
    title: String,
    year: Option<i32>,
    overview: Option<String>,
    poster_url: Option<String>,
    title_slug: String,
}

#[derive(Deserialize)]
struct SeriesDownloadedStatistics {
    percent_of_episodes: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SeriesDownload {
    id: String,
    statistics: Option<SeriesDownloadedStatistics>,
}

#[derive(Deserialize)]
struct SeriesDelete {
    id: String,
}

#[derive(Deserialize)]
enum Request {
    SeriesAdd { series: SeriesAdd },
    Download { series: SeriesDownload },
    SeriesDelete { series: SeriesDelete },
}

fn handleSeriesAdd(series: SeriesAdd, state: AppState) -> () {
    println!("{}", series.id)
}

fn handleSeriesDownload(series: SeriesDownload, state: AppState) -> () {
    println!("{}", series.id)
}

fn handleSeriesDelete(series: SeriesDelete, state: AppState) -> () {
    // let connection = state
    //     .db
    //     .lock()
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // connection.execute("DELETE FROM media WHERE external_id = ?1", (series.id));
}

pub async fn handler(
    State(state): State<AppState>,
    Json(payload): Json<Request>,
) -> Result<Json<Response>, StatusCode> {
    let result = match payload {
        Request::SeriesAdd { series } => handleSeriesAdd(series, state),
        Request::Download { series } => handleSeriesDownload(series, state),
        Request::SeriesDelete { series } => handleSeriesDelete(series, state),
    };
    Ok(Json(Response { success: true }))
}
