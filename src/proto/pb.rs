#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginAdminRequest {
    #[prost(string, tag="1")]
    pub username: std::string::String,
    #[prost(string, tag="2")]
    pub password: std::string::String,
    #[prost(string, tag="3")]
    pub code: std::string::String,
    #[prost(string, tag="4")]
    pub id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginAdminResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(string, tag="3")]
    pub token: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogoutAdminRequest {
    #[prost(string, tag="1")]
    pub token: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminInfoResp {
    #[prost(string, tag="1")]
    pub name: std::string::String,
    #[prost(string, tag="2")]
    pub avatar: std::string::String,
    #[prost(string, tag="3")]
    pub job: std::string::String,
    #[prost(string, tag="4")]
    pub organization: std::string::String,
    #[prost(string, tag="5")]
    pub location: std::string::String,
    #[prost(string, tag="6")]
    pub email: std::string::String,
    #[prost(string, tag="7")]
    pub introduction: std::string::String,
    #[prost(string, tag="8")]
    pub personal_website: std::string::String,
    #[prost(string, tag="9")]
    pub job_name: std::string::String,
    #[prost(string, tag="10")]
    pub organization_name: std::string::String,
    #[prost(string, tag="11")]
    pub location_name: std::string::String,
    #[prost(string, tag="12")]
    pub phone: std::string::String,
    #[prost(string, tag="13")]
    pub registration_date: std::string::String,
    #[prost(string, tag="14")]
    pub account_id: std::string::String,
    #[prost(string, tag="15")]
    pub certification: std::string::String,
    #[prost(string, tag="16")]
    pub role: std::string::String,
    #[prost(uint32, tag="17")]
    pub code: u32,
    #[prost(string, tag="18")]
    pub msg: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Comment {
    #[prost(string, tag="1")]
    pub id: std::string::String,
    #[prost(string, tag="2")]
    pub avatar: std::string::String,
    #[prost(string, tag="3")]
    pub username: std::string::String,
    #[prost(string, tag="4")]
    pub label: std::string::String,
    #[prost(string, tag="5")]
    pub create_date: std::string::String,
    #[prost(string, tag="6")]
    pub content: std::string::String,
    #[prost(message, repeated, tag="7")]
    pub children: ::std::vec::Vec<Comment>,
    #[prost(string, tag="8")]
    pub parent_username: std::string::String,
    #[prost(string, tag="9")]
    pub ip: std::string::String,
    #[prost(string, tag="10")]
    pub ua: std::string::String,
    #[prost(string, tag="11")]
    pub location: std::string::String,
    #[prost(string, tag="12")]
    pub os: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pagination {
    #[prost(uint32, tag="1")]
    pub count_total: u32,
    #[prost(uint32, tag="2")]
    pub total_page: u32,
    #[prost(uint32, tag="3")]
    pub current_page: u32,
    #[prost(uint32, tag="4")]
    pub page_size: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommentListResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(message, repeated, tag="4")]
    pub list: ::std::vec::Vec<Comment>,
    #[prost(message, optional, tag="5")]
    pub pagination: ::std::option::Option<Pagination>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommentAddRequest {
    #[prost(string, tag="1")]
    pub content: std::string::String,
    #[prost(string, tag="2")]
    pub article_id: std::string::String,
    #[prost(string, tag="3")]
    pub parent_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommentAddResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(message, optional, tag="3")]
    pub data: ::std::option::Option<Comment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CatchMe {
    #[prost(string, tag="1")]
    pub git: std::string::String,
    #[prost(string, tag="2")]
    pub job: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AboutResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(string, tag="3")]
    pub like_num: std::string::String,
    #[prost(message, optional, tag="4")]
    pub catch_me: ::std::option::Option<CatchMe>,
    #[prost(string, tag="5")]
    pub descriptions: std::string::String,
    #[prost(uint32, tag="6")]
    pub id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SiteInfoReq {
    #[prost(string, tag="1")]
    pub copyright: std::string::String,
    #[prost(string, tag="4")]
    pub descriptions: std::string::String,
    #[prost(string, tag="5")]
    pub beian: std::string::String,
    #[prost(string, tag="6")]
    pub title: std::string::String,
    #[prost(uint32, tag="7")]
    pub id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SiteInfoResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(string, tag="3")]
    pub author: std::string::String,
    #[prost(string, tag="4")]
    pub github: std::string::String,
    #[prost(string, tag="5")]
    pub beian: std::string::String,
    #[prost(string, tag="6")]
    pub descriptions: std::string::String,
    #[prost(string, tag="7")]
    pub self_descriptions: std::string::String,
    #[prost(uint32, tag="8")]
    pub id: u32,
    #[prost(string, tag="9")]
    pub title: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrowseList {
    #[prost(string, tag="1")]
    pub article_id: std::string::String,
    #[prost(string, tag="2")]
    pub title: std::string::String,
    #[prost(uint32, tag="3")]
    pub count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopCommentList {
    #[prost(string, tag="1")]
    pub article_id: std::string::String,
    #[prost(string, tag="2")]
    pub avatar: std::string::String,
    #[prost(string, tag="3")]
    pub title: std::string::String,
    #[prost(string, tag="4")]
    pub username: std::string::String,
    #[prost(string, tag="5")]
    pub content: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopCommentResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(string, tag="3")]
    pub love_count: std::string::String,
    #[prost(message, repeated, tag="4")]
    pub browse_list: ::std::vec::Vec<BrowseList>,
    #[prost(message, repeated, tag="5")]
    pub top_comment_list: ::std::vec::Vec<TopCommentList>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassList {
    #[prost(uint32, tag="1")]
    pub count: u32,
    #[prost(string, tag="2")]
    pub create_date: std::string::String,
    #[prost(string, tag="3")]
    pub last_modified_date: std::string::String,
    #[prost(string, tag="4")]
    pub name: std::string::String,
    #[prost(uint32, tag="5")]
    pub state: u32,
    #[prost(uint32, tag="6")]
    pub id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListYear {
    #[prost(message, repeated, tag="1")]
    pub list: ::std::vec::Vec<list_year::ListItem>,
    #[prost(string, tag="2")]
    pub year: std::string::String,
}
pub mod list_year {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ListItem {
        #[prost(string, tag="1")]
        pub create_date: std::string::String,
        #[prost(string, tag="2")]
        pub title: std::string::String,
        #[prost(uint32, tag="3")]
        pub id: u32,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListByClass {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(message, repeated, tag="3")]
    pub article_list: ::std::vec::Vec<ListYear>,
    #[prost(message, repeated, tag="4")]
    pub class_list: ::std::vec::Vec<ClassList>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Article {
    #[prost(uint32, tag="1")]
    pub browse_count: u32,
    #[prost(uint32, tag="2")]
    pub class_id: u32,
    #[prost(uint32, tag="3")]
    pub collect_count: u32,
    #[prost(uint32, tag="4")]
    pub comment_count: u32,
    #[prost(string, tag="5")]
    pub content: std::string::String,
    #[prost(string, tag="6")]
    pub create_date: std::string::String,
    #[prost(bool, tag="7")]
    pub is_hot: bool,
    #[prost(bool, tag="8")]
    pub is_recommend: bool,
    #[prost(string, tag="9")]
    pub last_modified_date: std::string::String,
    #[prost(uint32, tag="10")]
    pub like_count: u32,
    #[prost(uint32, tag="11")]
    pub state: u32,
    #[prost(uint32, tag="12")]
    pub id: u32,
    #[prost(uint32, tag="13")]
    pub v: u32,
    #[prost(uint32, repeated, tag="14")]
    pub tags: ::std::vec::Vec<u32>,
    #[prost(string, tag="15")]
    pub title: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArticleListResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(message, repeated, tag="3")]
    pub list: ::std::vec::Vec<Article>,
    #[prost(message, optional, tag="4")]
    pub pagination: ::std::option::Option<Pagination>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArticleOneResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(message, optional, tag="3")]
    pub obj: ::std::option::Option<Article>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaptchaResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(string, tag="3")]
    pub id: std::string::String,
    #[prost(string, tag="4")]
    pub img: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComponentMeta {
    #[prost(string, tag="1")]
    pub title: std::string::String,
    #[prost(string, tag="2")]
    pub name: std::string::String,
    #[prost(string, tag="3")]
    pub icon: std::string::String,
    #[prost(bool, tag="4")]
    pub no_cache: bool,
    #[prost(bool, tag="5")]
    pub affix: bool,
    #[prost(string, tag="6")]
    pub active_menu: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Component {
    #[prost(string, tag="1")]
    pub component: std::string::String,
    #[prost(string, tag="3")]
    pub path: std::string::String,
    #[prost(string, tag="4")]
    pub name: std::string::String,
    #[prost(message, optional, tag="5")]
    pub meta: ::std::option::Option<ComponentMeta>,
    #[prost(bool, tag="6")]
    pub hidden: bool,
    #[prost(message, repeated, tag="7")]
    pub children: ::std::vec::Vec<Component>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminRouterResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(message, repeated, tag="3")]
    pub data: ::std::vec::Vec<Component>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminCategoryAddRequest {
    #[prost(string, tag="1")]
    pub title: std::string::String,
    #[prost(string, tag="2")]
    pub description: std::string::String,
    #[prost(bool, tag="3")]
    pub support: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminCategoryListResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(uint32, tag="3")]
    pub total: u32,
    #[prost(message, repeated, tag="4")]
    pub rows: ::std::vec::Vec<admin_category_list_resp::CategoryBase>,
}
pub mod admin_category_list_resp {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BlogBase {
        #[prost(string, tag="1")]
        pub title: std::string::String,
        #[prost(string, tag="2")]
        pub summary: std::string::String,
        #[prost(string, tag="3")]
        pub header_img: std::string::String,
        #[prost(string, tag="4")]
        pub comment: std::string::String,
        #[prost(uint32, tag="5")]
        pub weight: u32,
        #[prost(bool, tag="6")]
        pub support: bool,
        #[prost(string, tag="7")]
        pub create_time: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CategoryBase {
        #[prost(string, tag="1")]
        pub title: std::string::String,
        #[prost(string, tag="2")]
        pub description: std::string::String,
        #[prost(string, tag="3")]
        pub create_time: std::string::String,
        #[prost(bool, tag="4")]
        pub support: bool,
        #[prost(message, repeated, tag="5")]
        pub blog_list: ::std::vec::Vec<BlogBase>,
        #[prost(uint32, tag="6")]
        pub id: u32,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminArticleAddRequest {
    #[prost(string, tag="1")]
    pub title: std::string::String,
    #[prost(string, tag="2")]
    pub summary: std::string::String,
    #[prost(uint32, tag="3")]
    pub category_id: u32,
    #[prost(bool, tag="4")]
    pub support: bool,
    #[prost(bool, tag="5")]
    pub comment: bool,
    #[prost(uint32, tag="6")]
    pub header_img_type: u32,
    #[prost(string, tag="7")]
    pub header_img: std::string::String,
    #[prost(uint32, tag="8")]
    pub weight: u32,
    #[prost(string, repeated, tag="9")]
    pub tag_title_list: ::std::vec::Vec<std::string::String>,
    #[prost(string, tag="10")]
    pub content: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminArticleListResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(uint32, tag="3")]
    pub total: u32,
    #[prost(message, repeated, tag="4")]
    pub rows: ::std::vec::Vec<admin_article_list_resp::AdminArticleListBase>,
}
pub mod admin_article_list_resp {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AdminArticleListCategory {
        #[prost(string, tag="1")]
        pub title: std::string::String,
        #[prost(string, tag="2")]
        pub description: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AdminArticleListBase {
        #[prost(string, tag="1")]
        pub title: std::string::String,
        #[prost(string, tag="2")]
        pub summary: std::string::String,
        #[prost(string, tag="3")]
        pub header_img: std::string::String,
        #[prost(bool, tag="4")]
        pub comment: bool,
        #[prost(uint32, tag="5")]
        pub weight: u32,
        #[prost(bool, tag="6")]
        pub support: bool,
        #[prost(string, tag="7")]
        pub create_time: std::string::String,
        #[prost(uint32, tag="8")]
        pub id: u32,
        #[prost(bool, tag="9")]
        pub status: bool,
        #[prost(message, optional, tag="10")]
        pub category: ::std::option::Option<AdminArticleListCategory>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminArticleOneResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(string, tag="3")]
    pub title: std::string::String,
    #[prost(string, tag="4")]
    pub summary: std::string::String,
    #[prost(uint32, tag="5")]
    pub category_id: u32,
    #[prost(bool, tag="6")]
    pub support: bool,
    #[prost(bool, tag="7")]
    pub comment: bool,
    #[prost(uint32, tag="8")]
    pub header_img_type: u32,
    #[prost(string, tag="9")]
    pub header_img: std::string::String,
    #[prost(uint32, tag="10")]
    pub weight: u32,
    #[prost(string, repeated, tag="11")]
    pub tag_title_list: ::std::vec::Vec<std::string::String>,
    #[prost(string, tag="12")]
    pub content: std::string::String,
    #[prost(uint32, tag="13")]
    pub id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tags {
    #[prost(uint32, tag="1")]
    pub id: u32,
    #[prost(string, tag="2")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListByClassResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(message, repeated, tag="3")]
    pub class_list: ::std::vec::Vec<list_by_class_resp::ClassList>,
    #[prost(message, repeated, tag="4")]
    pub article_list: ::std::vec::Vec<list_by_class_resp::ArticleList>,
    #[prost(message, repeated, tag="5")]
    pub tags: ::std::vec::Vec<Tags>,
}
pub mod list_by_class_resp {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct List {
        #[prost(uint32, tag="1")]
        pub id: u32,
        #[prost(string, tag="2")]
        pub create_date: std::string::String,
        #[prost(string, tag="3")]
        pub title: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClassList {
        #[prost(uint32, tag="1")]
        pub id: u32,
        #[prost(string, tag="2")]
        pub name: std::string::String,
        #[prost(uint32, tag="3")]
        pub count: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ArticleList {
        #[prost(uint32, tag="1")]
        pub year: u32,
        #[prost(message, repeated, tag="2")]
        pub list: ::std::vec::Vec<List>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminEditCategoryRequest {
    #[prost(uint32, tag="1")]
    pub id: u32,
    #[prost(string, tag="2")]
    pub title: std::string::String,
    #[prost(string, tag="3")]
    pub description: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkBase {
    #[prost(string, tag="1")]
    pub title: std::string::String,
    #[prost(string, tag="2")]
    pub description: std::string::String,
    #[prost(string, tag="3")]
    pub email: std::string::String,
    #[prost(string, tag="4")]
    pub url: std::string::String,
    #[prost(string, tag="5")]
    pub header_img: std::string::String,
    #[prost(bool, tag="6")]
    pub display: bool,
    #[prost(uint32, tag="7")]
    pub id: u32,
    #[prost(string, tag="8")]
    pub create_time: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkRequest {
    #[prost(string, tag="3")]
    pub title: std::string::String,
    #[prost(string, tag="4")]
    pub description: std::string::String,
    #[prost(string, tag="5")]
    pub email: std::string::String,
    #[prost(string, tag="6")]
    pub url: std::string::String,
    #[prost(string, tag="7")]
    pub header_img: std::string::String,
    #[prost(string, tag="8")]
    pub display: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkListResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(uint32, tag="3")]
    pub total: u32,
    #[prost(message, repeated, tag="4")]
    pub rows: ::std::vec::Vec<LinkBase>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfoResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(uint32, tag="3")]
    pub user_id: u32,
    #[prost(string, tag="4")]
    pub username: std::string::String,
    #[prost(uint32, tag="5")]
    pub status: u32,
    #[prost(string, tag="6")]
    pub avatar: std::string::String,
    #[prost(string, tag="7")]
    pub linkname: std::string::String,
    #[prost(string, tag="8")]
    pub link_url: std::string::String,
    #[prost(string, tag="9")]
    pub link_desc: std::string::String,
    #[prost(string, tag="10")]
    pub logo_url: std::string::String,
    #[prost(bool, tag="11")]
    pub state: bool,
    #[prost(uint32, tag="12")]
    pub label: u32,
    #[prost(bool, tag="13")]
    pub receive_update: bool,
    #[prost(string, tag="14")]
    pub token: std::string::String,
    #[prost(string, tag="15")]
    pub verify_status: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadFileResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(string, tag="3")]
    pub url: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditUserInfoRequest {
    #[prost(uint32, tag="1")]
    pub user_id: u32,
    #[prost(uint32, tag="2")]
    pub label: u32,
    #[prost(bool, tag="3")]
    pub state: bool,
    #[prost(string, tag="4")]
    pub link_url: std::string::String,
    #[prost(string, tag="5")]
    pub linkname: std::string::String,
    #[prost(string, tag="6")]
    pub link_desc: std::string::String,
    #[prost(bool, tag="7")]
    pub receive_update: bool,
    #[prost(string, tag="8")]
    pub logo_url: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAboutRequest {
    #[prost(string, tag="1")]
    pub content: std::string::String,
    #[prost(string, tag="2")]
    pub html_content: std::string::String,
    #[prost(uint32, tag="3")]
    pub id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PanelGroupResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(uint32, tag="3")]
    pub blog_count: u32,
    #[prost(uint32, tag="4")]
    pub visitor_count: u32,
    #[prost(uint32, tag="5")]
    pub user_count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LineChartDataResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(uint32, repeated, tag="3")]
    pub axis_data: ::std::vec::Vec<u32>,
    #[prost(uint32, repeated, tag="4")]
    pub expected_data: ::std::vec::Vec<u32>,
    #[prost(uint32, repeated, tag="5")]
    pub actual_data: ::std::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LikeOrCollectRequest {
    #[prost(uint32, tag="1")]
    pub id: u32,
    #[prost(bool, tag="2")]
    pub flag: bool,
    #[prost(bool, tag="3")]
    pub is_like: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsLikeOrCollectResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(bool, tag="3")]
    pub like: bool,
    #[prost(bool, tag="4")]
    pub collect: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(string, tag="3")]
    pub wechat_image: std::string::String,
    #[prost(string, tag="4")]
    pub ali_pay_image: std::string::String,
    #[prost(message, repeated, tag="5")]
    pub rewards: ::std::vec::Vec<reward_resp::Rewards>,
}
pub mod reward_resp {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Rewards {
        #[prost(string, tag="1")]
        pub pay_time: std::string::String,
        #[prost(string, tag="2")]
        pub name: std::string::String,
        #[prost(string, tag="3")]
        pub money: std::string::String,
        #[prost(string, tag="4")]
        pub source: std::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QiNiuListResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    ///  repeated categoryBase rows = 4;
    #[prost(uint32, tag="3")]
    pub total: u32,
}
pub mod qi_niu_list_resp {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QiNiuItem {
        #[prost(string, tag="1")]
        pub name: std::string::String,
        #[prost(string, tag="2")]
        pub url: std::string::String,
        #[prost(string, tag="3")]
        pub create_time: std::string::String,
        #[prost(string, tag="4")]
        pub id: std::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogResp {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub msg: std::string::String,
    #[prost(string, repeated, tag="3")]
    pub rows: ::std::vec::Vec<std::string::String>,
}
