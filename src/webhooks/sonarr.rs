// use crate::app::AppState;
// use axum::Json;
// use axum::extract::State;
// use http::StatusCode;
// use serde::{Deserialize, Serialize};
// use crate::db::NewSeries;
//
// #[derive(Serialize)]
// struct Response {
//     success: bool,
// }
//
// #[derive(Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct SeriesAdd {
//     id: i64,
//     title: String,
//     year: Option<i32>,
//     overview: Option<String>,
//     poster_url: Option<String>,
//     title_slug: String,
// }
//
// #[derive(Deserialize)]
// struct SeriesDownloadedStatistics {
//     percent_of_episodes: i32,
// }
//
// #[derive(Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct SeriesDownload {
//     id: String,
//     statistics: Option<SeriesDownloadedStatistics>,
// }
//
// #[derive(Deserialize)]
// struct SeriesDelete {
//     id: i64,
// }
//
// #[derive(Deserialize)]
// enum Request {
//     SeriesAdd { series: SeriesAdd },
//     Download { series: SeriesDownload },
//     SeriesDelete { series: SeriesDelete },
// }
//
// fn handle_series_add(series: SeriesAdd, state: AppState) -> Result<(), Err> {
//     let new_series = NewSeries {
//         external_id: series.id,
//         title: series.title,
//         title_slug: series.title_slug,
//         year: series.year,
//         overview: series.overview,
//         poster_url: series.poster_url,
//     };
//     state.db.create_series(new_series)
// }
//
// fn handle_series_download(series: SeriesDownload, state: AppState) -> () {
//     let new_series = NewSeries {
//         external_id: series.id,
//         title: series.title,
//         title_slug: series.title_slug,
//         year: series.year,
//         overview: series.overview,
//         poster_url: series.poster_url,
//     };
//     state.db.create_series(new_series)
// }
//
// fn handle_series_delete(series: SeriesDelete, state: AppState) -> Result<(), Err> {
//     state.db.delete_series(series.id)
// }
//
// pub async fn handler(
//     State(state): State<AppState>,
//     Json(payload): Json<Request>,
// ) -> Result<Json<Response>, StatusCode> {
//     let result = match payload {
//         Request::SeriesAdd { series } => handle_series_add(series, state),
//         Request::Download { series } => handle_series_download(series, state),
//         Request::SeriesDelete { series } => handle_series_delete(series, state),
//     };
//     Ok(Json(Response { success: true }))
// }
