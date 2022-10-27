// @generated automatically by Diesel CLI.

diesel::table! {
    article (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        is_delete -> Nullable<Bool>,
        category_id -> Nullable<Int8>,
        tags -> Nullable<Varchar>,
        user_id -> Int8,
        title -> Varchar,
        summary -> Bpchar,
        content -> Text,
        click_times -> Int8,
        like_count -> Int8,
        collect_count -> Int8,
        comment_count -> Int8,
        can_comment -> Bool,
        weight -> Int8,
        support -> Bool,
        header_img_type -> Int8,
        header_img -> Nullable<Varchar>,
    }
}

diesel::table! {
    category (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        is_delete -> Nullable<Bool>,
        name -> Varchar,
        display_name -> Varchar,
        seo_desc -> Nullable<Varchar>,
        support -> Bool,
        parent_id -> Int8,
    }
}

diesel::table! {
    collection (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        is_delete -> Nullable<Bool>,
        user_id -> Int8,
        article_id -> Int8,
    }
}

diesel::table! {
    comment (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        is_delete -> Nullable<Bool>,
        user_id -> Int8,
        article_id -> Int8,
        content -> Text,
        parent_id -> Nullable<Int8>,
        ip -> Nullable<Text>,
        ua -> Nullable<Text>,
        location -> Nullable<Text>,
        os -> Nullable<Text>,
    }
}

diesel::table! {
    like (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        is_delete -> Nullable<Bool>,
        user_id -> Int8,
        article_id -> Int8,
    }
}

diesel::table! {
    link (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        is_delete -> Nullable<Bool>,
        user_id -> Nullable<Int8>,
        title -> Varchar,
        description -> Varchar,
        email -> Text,
        url -> Text,
        header_img -> Nullable<Varchar>,
        show_link -> Nullable<Bool>,
        verify_status -> Nullable<Int8>,
    }
}

diesel::table! {
    request (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        is_delete -> Nullable<Bool>,
        ip -> Text,
        referer -> Nullable<Text>,
        url -> Text,
        major -> Nullable<Int8>,
        remote_addr -> Text,
        user_agent -> Nullable<Text>,
        op_type -> Nullable<Text>,
        method -> Nullable<Text>,
        is_login -> Nullable<Bool>,
        request_time -> Nullable<Int8>,
    }
}

diesel::table! {
    resource (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        is_delete -> Nullable<Bool>,
        uuid -> Text,
        key -> Text,
        #[sql_name = "type"]
        type_ -> Int8,
    }
}

diesel::table! {
    reward (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        is_delete -> Nullable<Bool>,
        order_id -> Text,
        who -> Text,
        amount -> Numeric,
        payment_method -> Int8,
    }
}

diesel::table! {
    siteinfo (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        is_delete -> Nullable<Bool>,
        author -> Nullable<Text>,
        title -> Nullable<Varchar>,
        keywords -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        record_number -> Nullable<Varchar>,
        ali_pay_image -> Nullable<Varchar>,
        we_chat_pay_image -> Nullable<Varchar>,
        self_description -> Nullable<Text>,
        self_description_html -> Nullable<Text>,
        git -> Nullable<Bpchar>,
        job -> Nullable<Bpchar>,
    }
}

diesel::table! {
    tags (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        is_delete -> Nullable<Bool>,
        name -> Text,
        tag_type -> Varchar,
    }
}

diesel::table! {
    user (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        is_delete -> Nullable<Bool>,
        user_name -> Text,
        password -> Text,
        avatar -> Text,
        label -> Int8,
        email -> Text,
        github_id -> Int8,
        github_url -> Text,
        is_admin -> Nullable<Bool>,
        receive_update -> Nullable<Bool>,
        last_login -> Nullable<Timestamptz>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    article,
    category,
    collection,
    comment,
    like,
    link,
    request,
    resource,
    reward,
    siteinfo,
    tags,
    user,
);
