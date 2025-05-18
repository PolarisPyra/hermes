use axum::{Json, Router, extract::State, routing::post};
use serde_json::json;
use sqlx::PgPool;
use tracing::{error, info};

pub async fn handle_game_login_api(_state: State<PgPool>) -> Json<serde_json::Value> {
    info!("ChunithmVerse GameLoginApi called");

    Json(json!({
        "returnCode": "1"
    }))
}

pub async fn handle_game_logout_api(_state: State<PgPool>) -> Json<serde_json::Value> {
    info!("ChunithmVerse GameLogoutApi called");

    Json(json!({
        "returnCode": "1"
    }))
}

pub async fn handle_game_charge_api(_state: State<PgPool>) -> Json<serde_json::Value> {
    info!("ChunithmVerse GetGameChargeApi called");

    Json(json!([]))
}

pub async fn handle_game_event_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetGameEventApi called");

    Json(json!([]))
}

pub async fn handle_game_idlist_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetGameIdlistApi called");
    Json(json!([]))
}

pub async fn handle_game_message_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetGameMessageApi called");
    Json(json!([]))
}

pub async fn handle_game_ranking_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetGameRankingApi called");
    Json(json!([]))
}

pub async fn handle_game_sale_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetGameSaleApi called");
    Json(json!([]))
}

pub async fn handle_game_settings_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetGameSettingsApi called");
    Json(json!({
        "gameSetting": {
            "dataVersion": "2.31.00",
            "isMaintenance": "false",
            "requestInterval": "10",
            "rebootStartTime": "2025-05-17 18:00:00",
            "rebootEndTime": "2025-05-17 19:00:00",
            "isBackgroundDistribute": "false",
            "maxCountCharacter": "300",
            "maxCountItem": "300",
            "maxCountMusic": "300"
        },
        "isDumpUpload": "false",
        "isAou": "false"
    }))
}

pub async fn handle_user_activity_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserActivityApi called");
    Json(json!([]))
}

pub async fn handle_user_character_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserCharacterApi called");
    Json(json!([]))
}

pub async fn handle_user_charge_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserChargeApi called");
    Json(json!([]))
}

pub async fn handle_user_recent_player_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserRecentPlayerApi called");
    Json(json!([]))
}

pub async fn handle_user_dataex_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserDataExApi called");
    Json(json!([]))
}

pub async fn handle_user_duel_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserDuelApi called");
    Json(json!([]))
}

pub async fn handle_user_rival_music_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserRivalMusicApi called");
    Json(json!([]))
}

pub async fn handle_user_favorite_item_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserFavoriteItemApi called");
    Json(json!([]))
}

pub async fn handle_user_favorite_music_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserFavoriteMusicApi called");
    Json(json!([]))
}

pub async fn handle_user_item_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserItemApi called");
    Json(json!([]))
}

pub async fn handle_user_login_bonus_api() {
    error!("ChunithmVerse GetUserLoginBonusApi called and its unimplemented");
}

pub async fn handle_user_map_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserMapApi called");
    Json(json!([]))
}

pub async fn handle_user_music_api() {
    error!("ChunithmVerse GetUserMusicApi called and its unimplemented");
}

pub async fn handle_user_option_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserOptionApi called");
    Json(json!(null))
}

pub async fn handle_user_optionex_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserOptionExApi called");
    Json(json!(null))
}

pub async fn handle_user_preview_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserPreviewApi called");
    Json(json!({}))
}

pub async fn handle_user_recent_rating_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserRecentRatingApi called");
    Json(json!([]))
}

pub async fn handle_user_team_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserTeamApi called");
    Json(json!([]))
}

pub async fn handle_team_course_setting_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetTeamCourseSettingApi called");
    Json(json!([]))
}

pub async fn handle_team_course_setting_api_proto() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetTeamCourseSettingApiProto called");
    Json(json!([]))
}

pub async fn handle_team_course_rule_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetTeamCourseRuleApi called");
    Json(json!([]))
}

pub async fn handle_team_course_rule_api_proto() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetTeamCourseRuleApiProto called");
    Json(json!([]))
}

pub async fn handle_upsert_user_all_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse UpsertUserAllApi called");
    Json(json!({"returnCode": "1"}))
}

pub async fn handle_upsert_user_chargelog_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse UpsertUserChargelogApi called");
    Json(json!({"returnCode": "1"}))
}

pub async fn handle_upsert_client_bookkeeping_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse UpsertClientBookkeepingApi called");
    Json(json!({"returnCode": "1"}))
}

pub async fn handle_upsert_client_develop_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse UpsertClientDevelopApi called");
    Json(json!({"returnCode": "1"}))
}

pub async fn handle_upsert_client_error_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse UpsertClientErrorApi called");
    Json(json!({"returnCode": "1"}))
}

pub async fn handle_upsert_client_testmode_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse UpsertClientTestmodeApi called");
    Json(json!({"returnCode": "1"}))
}

pub async fn handle_user_netbattle_data_api() {
    error!("ChunithmVerse GetUserNetBattleDataApi called and its unimplemented");
}

pub async fn handle_cm_get_user_preview_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse CmGetUserPreviewApi called");
    Json(json!({}))
}

pub async fn handle_cm_get_user_data_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse CmGetUserDataApi called");
    Json(json!([]))
}

pub async fn handle_user_data_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserDataApi called");
    Json(json!({}))
}

pub async fn handle_game_map_area_condition_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetGameMapAreaConditionApi called");
    Json(json!([]))
}

pub async fn handle_game_course_level_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetGameCourseLevelApi called");
    Json(json!([]))
}

pub async fn handle_game_uc_condition_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetGameUcConditionApi called");
    Json(json!([]))
}

pub async fn handle_user_uc_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserUcApi called");
    Json(json!([]))
}

pub async fn handle_user_rec_music_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserRecMusicApi called");
    Json(json!([]))
}

pub async fn handle_user_rec_rating_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserRecRatingApi called");
    Json(json!([]))
}

pub async fn handle_user_symbol_chat_setting_api() {
    error!("ChunithmVerse GetUserNetBattleDataApi called and its unimplemented");
}

pub async fn handle_user_map_area_api() {
    error!("ChunithmVerse GetUserMapAreaApi called and its unimplemented");
}

pub async fn handle_user_region_api() -> Json<serde_json::Value> {
    info!("ChunithmVerse GetUserRegionApi called");
    Json(json!([]))
}

pub fn chunithm_verse_routes() -> Router<PgPool> {
    Router::new().nest(
        "/SDHD/231",
        Router::new().nest(
            "/ChuniServlet",
            Router::new()
                .route("/GameLoginApi", post(handle_game_login_api))
                .route("/GameLogoutApi", post(handle_game_logout_api))
                .route("/GetGameChargeApi", post(handle_game_charge_api))
                .route("/GetGameEventApi", post(handle_game_event_api))
                .route("/GetGameIdlistApi", post(handle_game_idlist_api))
                .route("/GetGameMessageApi", post(handle_game_message_api))
                .route("/GetGameRankingApi", post(handle_game_ranking_api))
                .route("/GetGameSaleApi", post(handle_game_sale_api))
                .route("/GetGameSettingsApi", post(handle_game_settings_api))
                .route("/GetUserActivityApi", post(handle_user_activity_api))
                .route("/GetUserCharacterApi", post(handle_user_character_api))
                .route("/GetUserChargeApi", post(handle_user_charge_api))
                .route(
                    "/GetUserRecentPlayerApi",
                    post(handle_user_recent_player_api),
                )
                .route("/GetUserDataExApi", post(handle_user_dataex_api))
                .route("/GetUserDuelApi", post(handle_user_duel_api))
                .route("/GetUserRivalMusicApi", post(handle_user_rival_music_api))
                .route(
                    "/GetUserFavoriteItemApi",
                    post(handle_user_favorite_item_api),
                )
                .route(
                    "/GetUserFavoriteMusicApi",
                    post(handle_user_favorite_music_api),
                )
                .route("/GetUserItemApi", post(handle_user_item_api))
                .route("/GetUserLoginBonusApi", post(handle_user_login_bonus_api))
                .route("/GetUserMapApi", post(handle_user_map_api))
                .route("/GetUserMusicApi", post(handle_user_music_api))
                .route("/GetUserOptionApi", post(handle_user_option_api))
                .route("/GetUserOptionExApi", post(handle_user_optionex_api))
                .route("/GetUserPreviewApi", post(handle_user_preview_api))
                .route(
                    "/GetUserRecentRatingApi",
                    post(handle_user_recent_rating_api),
                )
                .route("/GetUserTeamApi", post(handle_user_team_api))
                .route(
                    "/GetTeamCourseSettingApi",
                    post(handle_team_course_setting_api),
                )
                .route(
                    "/GetTeamCourseSettingApiProto",
                    post(handle_team_course_setting_api_proto),
                )
                .route("/GetTeamCourseRuleApi", post(handle_team_course_rule_api))
                .route(
                    "/GetTeamCourseRuleApiProto",
                    post(handle_team_course_rule_api_proto),
                )
                .route("/UpsertUserAllApi", post(handle_upsert_user_all_api))
                .route(
                    "/UpsertUserChargelogApi",
                    post(handle_upsert_user_chargelog_api),
                )
                .route(
                    "/UpsertClientBookkeepingApi",
                    post(handle_upsert_client_bookkeeping_api),
                )
                .route(
                    "/UpsertClientDevelopApi",
                    post(handle_upsert_client_develop_api),
                )
                .route(
                    "/UpsertClientErrorApi",
                    post(handle_upsert_client_error_api),
                )
                .route(
                    "/UpsertClientTestmodeApi",
                    post(handle_upsert_client_testmode_api),
                )
                .route(
                    "/GetUserNetBattleDataApi",
                    post(handle_user_netbattle_data_api),
                )
                .route("/CmGetUserPreviewApi", post(handle_cm_get_user_preview_api))
                .route("/CmGetUserDataApi", post(handle_cm_get_user_data_api))
                .route("/GetUserDataApi", post(handle_user_data_api))
                .route(
                    "/GetGameMapAreaConditionApi",
                    post(handle_game_map_area_condition_api),
                )
                .route("/GetGameCourseLevelApi", post(handle_game_course_level_api))
                .route("/GetGameUcConditionApi", post(handle_game_uc_condition_api))
                .route("/GetUserUcApi", post(handle_user_uc_api))
                .route("/GetUserRecMusicApi", post(handle_user_rec_music_api))
                .route("/GetUserRecRatingApi", post(handle_user_rec_rating_api))
                .route(
                    "/GetUserSymbolChatSettingApi",
                    post(handle_user_symbol_chat_setting_api),
                )
                .route("/GetUserMapAreaApi", post(handle_user_map_area_api))
                .route("/GetUserRegionApi", post(handle_user_region_api)),
        ),
    )
}
