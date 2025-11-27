// @generated automatically by Diesel CLI.

diesel::table! {
    alert_logs (id) {
        id -> Integer,
        alert_rules_id -> Integer,
        monitor_id -> Integer,
        alert_type -> Text,
        status -> Integer,
        message -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    alert_rules (id) {
        id -> Integer,
        monitor_id -> Integer,
        alert_type -> Text,
        config_json -> Nullable<Text>,
        enabled -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    check_result (id) {
        id -> Integer,
        monitor_id -> Integer,
        monitor_type -> Text,
        status -> Integer,
        response_time -> Integer,
        metadata_json -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    monitor_config (id) {
        id -> Integer,
        name -> Nullable<Text>,
        target -> Text,
        method -> Nullable<Text>,
        monitor_type -> Text,
        interval_ms -> Nullable<Integer>,
        timeout_ms -> Integer,
        config_json -> Nullable<Text>,
        enabled -> Integer,
        tag -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    notification_channels (id) {
        id -> Integer,
        name -> Nullable<Text>,
        channel_type -> Text,
        config_json -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(alert_logs -> alert_rules (alert_rules_id));
diesel::joinable!(alert_logs -> monitor_config (monitor_id));
diesel::joinable!(alert_rules -> monitor_config (monitor_id));
diesel::joinable!(check_result -> monitor_config (monitor_id));

diesel::allow_tables_to_appear_in_same_query!(
    alert_logs,
    alert_rules,
    check_result,
    monitor_config,
    notification_channels,
);
