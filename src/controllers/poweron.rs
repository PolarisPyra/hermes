use std::io::Read;

use axum::{
    body::Bytes,
    extract::Extension,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};
use base64::{Engine as _, engine::general_purpose};
use chrono::{Datelike, SecondsFormat, Timelike, Utc};
use flate2::read::ZlibDecoder;
use serde::Deserialize;
use serde_urlencoded;
use sqlx::{Pool, Postgres};
use tracing::{debug, info};

use crate::models::poweron::PowerOnResponse;

#[derive(Deserialize, Debug)]
pub struct PowerOnRequest {
    format_ver: Option<f32>,
    game_id: Option<String>,
    token: Option<String>,
    ver: Option<f32>,
}

fn decode_dfi(data: &[u8]) -> Result<String, String> {
    let decoded = general_purpose::STANDARD
        .decode(data)
        .map_err(|e| format!("Base64 decode error: {}", e))?;

    let mut decoder = ZlibDecoder::new(&decoded[..]);
    let mut decompressed = String::new();
    decoder
        .read_to_string(&mut decompressed)
        .map_err(|e| format!("Zlib decompress error: {}", e))?;

    Ok(decompressed
        .trim_end_matches(|c| c == '\r' || c == '\n')
        .to_string())
}

pub async fn poweron(
    Extension(_pool): Extension<Pool<Postgres>>,
    headers: HeaderMap,
    raw_body: Bytes,
) -> Result<impl IntoResponse, StatusCode> {
    // Check for Pragma: DFI
    if headers.get("Pragma").map(|v| v != "DFI").unwrap_or(true) {
        info!("Missing or invalid Pragma header");
        return Err(StatusCode::BAD_REQUEST);
    }

    // Decode & decompress request body
    let decoded = match decode_dfi(&raw_body) {
        Ok(s) => s,
        Err(err) => {
            info!("Failed to decode DFI body: {}", err);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    // Parse URL-encoded data into struct
    let payload: PowerOnRequest = match serde_urlencoded::from_str(&decoded) {
        Ok(p) => p,
        Err(err) => {
            info!("Failed to parse form data: {}", err);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let format_ver = match payload.format_ver {
        Some(v) => {
            debug!("format_ver received: {}", v);
            v
        }
        None => {
            info!("Missing format_ver");
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let now = Utc::now();
    let (year, month, day, hour, minute, second) = (
        now.year().to_string(),
        now.month().to_string(),
        now.day().to_string(),
        now.hour().to_string(),
        now.minute().to_string(),
        now.second().to_string(),
    );
    let utc_time_z = now.to_rfc3339_opts(SecondsFormat::Secs, true); // e.g., 2025-05-16T20:59:15Z

    let game_id = match &payload.game_id {
        Some(id) => id,
        None => {
            info!("Missing game_id");
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let ver = match payload.ver {
        Some(v) => v,
        None => {
            info!("Missing ver");
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let ver = ver.to_string().replace(".", "");
    let title_server_uri = format!("http://polaris.pyra:8080/{}/{}", game_id, ver);
    info!("{}", title_server_uri);

    let title_server_hostname = format!("http://title.polaris.pyra");

    let response = if (format_ver - 1.0).abs() < f32::EPSILON {
        PowerOnResponse {
            uri: Some(title_server_uri.clone()),
            place_id: Some("123".into()),
            host: Some(title_server_hostname.clone()),
            stat: Some(1),
            name: Some("Hermes".into()),
            nickname: Some("Hermes".into()),
            year: Some(year),
            month: Some(month),
            day: Some(day),
            hour: Some(hour),
            minute: Some(minute),
            second: Some(second),
            setting: Some("1".into()),
            region0: Some("1".into()),
            region_name0: Some("1".into()),
            region_name1: Some("".into()),
            region_name_2: Some("".into()),
            region_name_3: Some("".into()),
            country: Some("".into()),
            timezone: None,
            res_class: None,
            res_ver: None,
            allnet_id: None,
            utc_time: None,
            token: None,
        }
    } else if (format_ver - 2.0).abs() < f32::EPSILON {
        PowerOnResponse {
            uri: Some(title_server_uri.clone()),
            place_id: Some("123".into()),
            host: Some(title_server_hostname.clone()),
            stat: Some(1),
            name: Some("Hermes".into()),
            nickname: Some("Hermes".into()),
            year: Some(year),
            month: Some(month),
            day: Some(day),
            hour: Some(hour),
            minute: Some(minute),
            second: Some(second),
            setting: Some("1".into()),
            region0: Some("1".into()),
            region_name0: Some("1".into()),
            region_name1: Some("".into()),
            region_name_2: Some("".into()),
            region_name_3: Some("".into()),
            country: Some("".into()),
            timezone: Some("".into()),
            res_class: Some("".into()),
            res_ver: None,
            allnet_id: None,
            utc_time: None,
            token: None,
        }
    } else if (format_ver - 3.0).abs() < f32::EPSILON {
        PowerOnResponse {
            uri: Some(title_server_uri.clone()),
            place_id: Some("".into()),
            host: Some(title_server_hostname.clone()),
            stat: Some(1),
            name: Some("Hermes".into()),
            nickname: Some("Hermes".into()),
            year: Some(year),
            month: Some(month),
            day: Some(day),
            hour: Some(hour),
            minute: Some(minute),
            second: Some(second),
            setting: Some("1".into()),
            region0: Some("1".into()),
            region_name0: Some("1".into()),
            region_name1: Some("".into()),
            region_name_2: Some("".into()),
            region_name_3: Some("".into()),
            country: Some("JPN".into()),
            timezone: None,
            res_class: None,
            res_ver: Some(3),
            allnet_id: Some(123),
            utc_time: Some(utc_time_z),
            token: payload.token,
        }
    } else {
        info!("Unsupported format_ver: {}", format_ver);
        return Err(StatusCode::BAD_REQUEST);
    };

    let response_body = serde_urlencoded::to_string(&response).unwrap();
    let decoded = urlencoding::decode(&response_body).unwrap();

    debug!("Response body: {}", decoded);

    let mut headers = HeaderMap::new();
    headers.insert("Pragma", HeaderValue::from_static("DFI"));

    Ok((headers, decoded.to_string()))
}
