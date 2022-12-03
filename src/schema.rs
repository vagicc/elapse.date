table! {
    admins (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        salt -> Bpchar,
        email -> Nullable<Varchar>,
        mobile -> Nullable<Bpchar>,
        role -> Nullable<Int2>,
        status -> Nullable<Int8>,
        create_time -> Nullable<Timestamp>,
        last_login -> Nullable<Timestamp>,
    }
}

table! {
    article (id) {
        id -> Int4,
        title -> Varchar,
        cover -> Nullable<Varchar>,
        summary -> Nullable<Varchar>,
        seo_title -> Nullable<Varchar>,
        seo_keywords -> Nullable<Varchar>,
        seo_description -> Nullable<Varchar>,
        content_id -> Int4,
        category_id -> Nullable<Int4>,
        category -> Nullable<Varchar>,
        columns_id -> Int4,
        available -> Nullable<Int2>,
        nav_id -> Nullable<Int4>,
        visit -> Int8,
        collect -> Int8,
        share -> Int8,
        user_id -> Nullable<Int4>,
        username -> Nullable<Varchar>,
        create -> Nullable<Int8>,
        last_time -> Nullable<Timestamp>,
    }
}

table! {
    article_category (id) {
        id -> Int4,
        category -> Varchar,
        seo_title -> Nullable<Varchar>,
        seo_keywords -> Nullable<Varchar>,
        seo_description -> Nullable<Varchar>,
        show -> Int2,
        order_by -> Nullable<Int2>,
        modify_id -> Nullable<Int4>,
        modify_time -> Nullable<Timestamp>,
        create_id -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
    }
}

table! {
    article_content (id) {
        id -> Int4,
        content -> Nullable<Text>,
    }
}

table! {
    ci_sessions (id) {
        id -> Varchar,
        ip_address -> Inet,
        timestamp -> Timestamptz,
        data -> Bytea,
    }
}

table! {
    column (id) {
        id -> Int4,
        title -> Varchar,
        subhead -> Varchar,
        surface_plot -> Nullable<Varchar>,
        author -> Nullable<Varchar>,
        excerpt -> Nullable<Text>,
        price -> Nullable<Money>,
        visit -> Int8,
        collect -> Int8,
        amount -> Nullable<Int4>,
        complete -> Int4,
        seo_title -> Nullable<Varchar>,
        seo_keywords -> Nullable<Varchar>,
        seo_description -> Nullable<Varchar>,
        create_id -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
    }
}

table! {
    menus (id) {
        id -> Int4,
        order_by -> Int2,
        class -> Nullable<Varchar>,
        method -> Nullable<Varchar>,
        name -> Varchar,
        level -> Nullable<Int2>,
        parent -> Nullable<Int2>,
        icon -> Nullable<Varchar>,
        department -> Nullable<Varchar>,
        is_show -> Nullable<Bool>,
    }
}

table! {
    record (record_time) {
        id -> Int4,
        table_id -> Int4,
        table_name -> Varchar,
        user_id -> Int4,
        username -> Varchar,
        action -> Varchar,
        ip -> Inet,
        record_time -> Timestamp,
    }
}

table! {
    record_2022 (record_time) {
        id -> Int4,
        table_id -> Int4,
        table_name -> Varchar,
        user_id -> Int4,
        username -> Varchar,
        action -> Varchar,
        ip -> Inet,
        record_time -> Timestamp,
    }
}

table! {
    rights (right_id) {
        right_id -> Int4,
        right_name -> Nullable<Varchar>,
        right_class -> Nullable<Varchar>,
        right_method -> Nullable<Varchar>,
        right_detail -> Nullable<Varchar>,
    }
}

table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
        rights -> Nullable<Varchar>,
        default -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    admins,
    article,
    article_category,
    article_content,
    ci_sessions,
    column,
    menus,
    record,
    record_2022,
    rights,
    roles,
);
