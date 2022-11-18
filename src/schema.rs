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

allow_tables_to_appear_in_same_query!(
    article,
    article_category,
    article_content,
    column,
);
