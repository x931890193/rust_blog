
-- ----------------------------
-- Table structure for article
-- ----------------------------
CREATE SEQUENCE IF NOT EXISTS article_id_seq;
CREATE SEQUENCE IF NOT EXISTS category_id_seq;
CREATE SEQUENCE IF NOT EXISTS collection_id_seq;
CREATE SEQUENCE IF NOT EXISTS comment_id_seq;
CREATE SEQUENCE IF NOT EXISTS like_id_seq;
CREATE SEQUENCE IF NOT EXISTS request_id_seq;
CREATE SEQUENCE IF NOT EXISTS link_id_seq;
CREATE SEQUENCE IF NOT EXISTS reward_id_seq;
CREATE SEQUENCE IF NOT EXISTS resource_id_seq;
CREATE SEQUENCE IF NOT EXISTS siteinfo_id_seq;
CREATE SEQUENCE IF NOT EXISTS tags_id_seq;
CREATE SEQUENCE IF NOT EXISTS user_id_seq;

DROP TABLE IF EXISTS "public"."article";
CREATE TABLE "public"."article" (
                                    "id" int8 NOT NULL DEFAULT nextval('article_id_seq'::regclass),
                                    "created_at" timestamptz(6),
                                    "updated_at" timestamptz(6),
                                    "is_delete" bool DEFAULT false,
                                    "category_id" int8,
                                    "tags" varchar(255) COLLATE "pg_catalog"."default",
                                    "user_id" int8 NOT NULL,
                                    "title" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
                                    "summary" char(255) COLLATE "pg_catalog"."default" NOT NULL,
                                    "content" text COLLATE "pg_catalog"."default" NOT NULL,
                                    "click_times" int8 NOT NULL DEFAULT 0,
                                    "like_count" int8 NOT NULL DEFAULT 0,
                                    "collect_count" int8 NOT NULL DEFAULT 0,
                                    "comment_count" int8 NOT NULL DEFAULT 0,
                                    "can_comment" bool NOT NULL DEFAULT true,
                                    "weight" int8 NOT NULL,
                                    "support" bool NOT NULL DEFAULT true,
                                    "header_img_type" int8 NOT NULL,
                                    "header_img" varchar(255) COLLATE "pg_catalog"."default"
)
;
ALTER TABLE "public"."article" OWNER TO "postgres";
COMMENT ON COLUMN "public"."article"."user_id" IS ' 用户ID';
COMMENT ON COLUMN "public"."article"."title" IS ' 标题';
COMMENT ON COLUMN "public"."article"."summary" IS ' 摘要';
COMMENT ON COLUMN "public"."article"."content" IS ' 文章内容';

-- ----------------------------
-- Table structure for category
-- ----------------------------
DROP TABLE IF EXISTS "public"."category";
CREATE TABLE "public"."category" (
                                     "id" int8 NOT NULL DEFAULT nextval('category_id_seq'::regclass),
                                     "created_at" timestamptz(6),
                                     "updated_at" timestamptz(6),
                                     "is_delete" bool DEFAULT false,
                                     "name" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
                                     "display_name" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
                                     "seo_desc" varchar(255) COLLATE "pg_catalog"."default",
                                     "support" bool NOT NULL,
                                     "parent_id" int8 NOT NULL
)
;
ALTER TABLE "public"."category" OWNER TO "postgres";
COMMENT ON COLUMN "public"."category"."name" IS '分类名';
COMMENT ON COLUMN "public"."category"."display_name" IS '分类别名';
COMMENT ON COLUMN "public"."category"."seo_desc" IS 'seo描述';
COMMENT ON COLUMN "public"."category"."parent_id" IS '父类ID';

-- ----------------------------
-- Table structure for collection
-- ----------------------------
DROP TABLE IF EXISTS "public"."collection";
CREATE TABLE "public"."collection" (
                                       "id" int8 NOT NULL DEFAULT nextval('collection_id_seq'::regclass),
                                       "created_at" timestamptz(6),
                                       "updated_at" timestamptz(6),
                                       "is_delete" bool DEFAULT false,
                                       "user_id" int8 NOT NULL,
                                       "article_id" int8 NOT NULL
)
;
ALTER TABLE "public"."collection" OWNER TO "postgres";

-- ----------------------------
-- Table structure for comment
-- ----------------------------
DROP TABLE IF EXISTS "public"."comment";
CREATE TABLE "public"."comment" (
                                    "id" int8 NOT NULL DEFAULT nextval('comment_id_seq'::regclass),
                                    "created_at" timestamptz(6),
                                    "updated_at" timestamptz(6),
                                    "is_delete" bool DEFAULT false,
                                    "user_id" int8 NOT NULL,
                                    "article_id" int8 NOT NULL,
                                    "content" text COLLATE "pg_catalog"."default" NOT NULL,
                                    "parent_id" int8,
                                    "ip" text COLLATE "pg_catalog"."default",
                                    "ua" text COLLATE "pg_catalog"."default",
                                    "location" text COLLATE "pg_catalog"."default",
                                    "os" text COLLATE "pg_catalog"."default"
)
;
ALTER TABLE "public"."comment" OWNER TO "postgres";
COMMENT ON COLUMN "public"."comment"."user_id" IS ' 用户ID';
COMMENT ON COLUMN "public"."comment"."article_id" IS ' 文章ID';
COMMENT ON COLUMN "public"."comment"."content" IS ' 评论内容';
COMMENT ON COLUMN "public"."comment"."parent_id" IS ' 父评论ID';

-- ----------------------------
-- Table structure for like
-- ----------------------------
DROP TABLE IF EXISTS "public"."like";
CREATE TABLE "public"."like" (
                                 "id" int8 NOT NULL DEFAULT nextval('like_id_seq'::regclass),
                                 "created_at" timestamptz(6),
                                 "updated_at" timestamptz(6),
                                 "is_delete" bool DEFAULT false,
                                 "user_id" int8 NOT NULL,
                                 "article_id" int8 NOT NULL
)
;
ALTER TABLE "public"."like" OWNER TO "postgres";

-- ----------------------------
-- Table structure for link
-- ----------------------------
DROP TABLE IF EXISTS "public"."link";
CREATE TABLE "public"."link" (
                                 "id" int8 NOT NULL DEFAULT nextval('link_id_seq'::regclass),
                                 "created_at" timestamptz(6),
                                 "updated_at" timestamptz(6),
                                 "is_delete" bool DEFAULT false,
                                 "user_id" int8,
                                 "title" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
                                 "description" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
                                 "email" text COLLATE "pg_catalog"."default" NOT NULL,
                                 "url" text COLLATE "pg_catalog"."default" NOT NULL,
                                 "header_img" varchar(255) COLLATE "pg_catalog"."default",
                                 "show_link" bool DEFAULT true,
                                 "verify_status" int8 DEFAULT 0
)
;
ALTER TABLE "public"."link" OWNER TO "postgres";
COMMENT ON COLUMN "public"."link"."title" IS ' 标题';
COMMENT ON COLUMN "public"."link"."description" IS ' 网站描述';
COMMENT ON COLUMN "public"."link"."email" IS ' 邮箱';

-- ----------------------------
-- Table structure for request
-- ----------------------------
DROP TABLE IF EXISTS "public"."request";
CREATE TABLE "public"."request" (
                                    "id" int8 NOT NULL DEFAULT nextval('request_id_seq'::regclass),
                                    "created_at" timestamptz(6),
                                    "updated_at" timestamptz(6),
                                    "is_delete" bool DEFAULT false,
                                    "ip" text COLLATE "pg_catalog"."default" NOT NULL DEFAULT '127.0.0.1'::text,
                                    "referer" text COLLATE "pg_catalog"."default",
                                    "url" text COLLATE "pg_catalog"."default" NOT NULL,
                                    "major" int8,
                                    "remote_addr" text COLLATE "pg_catalog"."default" NOT NULL,
                                    "user_agent" text COLLATE "pg_catalog"."default",
                                    "op_type" text COLLATE "pg_catalog"."default",
                                    "method" text COLLATE "pg_catalog"."default",
                                    "is_login" bool,
                                    "request_time" int8
)
;
ALTER TABLE "public"."request" OWNER TO "postgres";

-- ----------------------------
-- Table structure for resource
-- ----------------------------
DROP TABLE IF EXISTS "public"."resource";
CREATE TABLE "public"."resource" (
                                     "id" int8 NOT NULL DEFAULT nextval('resource_id_seq'::regclass),
                                     "created_at" timestamptz(6),
                                     "updated_at" timestamptz(6),
                                     "is_delete" bool DEFAULT false,
                                     "uuid" text COLLATE "pg_catalog"."default" NOT NULL,
                                     "key" text COLLATE "pg_catalog"."default" NOT NULL,
                                     "type" int8 NOT NULL
)
;
ALTER TABLE "public"."resource" OWNER TO "postgres";
COMMENT ON COLUMN "public"."resource"."type" IS ' 资源类型';

-- ----------------------------
-- Table structure for reward
-- ----------------------------
DROP TABLE IF EXISTS "public"."reward";
CREATE TABLE "public"."reward" (
                                   "id" int8 NOT NULL DEFAULT nextval('reward_id_seq'::regclass),
                                   "created_at" timestamptz(6),
                                   "updated_at" timestamptz(6),
                                   "is_delete" bool DEFAULT false,
                                   "order_id" text COLLATE "pg_catalog"."default" NOT NULL,
                                   "who" text COLLATE "pg_catalog"."default" NOT NULL,
                                   "amount" numeric NOT NULL,
                                   "payment_method" int8 NOT NULL
)
;
ALTER TABLE "public"."reward" OWNER TO "postgres";
COMMENT ON COLUMN "public"."reward"."order_id" IS ' 订单号';
COMMENT ON COLUMN "public"."reward"."who" IS ' 赞赏人';
COMMENT ON COLUMN "public"."reward"."amount" IS ' 金额';
COMMENT ON COLUMN "public"."reward"."payment_method" IS ' 支付方式';

-- ----------------------------
-- Table structure for siteinfo
-- ----------------------------
DROP TABLE IF EXISTS "public"."siteinfo";
CREATE TABLE "public"."siteinfo" (
                                     "id" int8 NOT NULL DEFAULT nextval('siteinfo_id_seq'::regclass),
                                     "created_at" timestamptz(6),
                                     "updated_at" timestamptz(6),
                                     "is_delete" bool DEFAULT false,
                                     "author" text COLLATE "pg_catalog"."default",
                                     "title" varchar(255) COLLATE "pg_catalog"."default",
                                     "keywords" varchar(255) COLLATE "pg_catalog"."default",
                                     "description" varchar(255) COLLATE "pg_catalog"."default",
                                     "record_number" varchar(255) COLLATE "pg_catalog"."default",
                                     "ali_pay_image" varchar(255) COLLATE "pg_catalog"."default",
                                     "we_chat_pay_image" varchar(255) COLLATE "pg_catalog"."default",
                                     "self_description" text COLLATE "pg_catalog"."default",
                                     "self_description_html" text COLLATE "pg_catalog"."default",
                                     "git" char(255) COLLATE "pg_catalog"."default",
                                     "job" char(255) COLLATE "pg_catalog"."default"
)
;
ALTER TABLE "public"."siteinfo" OWNER TO "postgres";
COMMENT ON COLUMN "public"."siteinfo"."title" IS ' 网站title';
COMMENT ON COLUMN "public"."siteinfo"."keywords" IS ' 网站关键字';
COMMENT ON COLUMN "public"."siteinfo"."description" IS ' 网站描述';
COMMENT ON COLUMN "public"."siteinfo"."record_number" IS ' 备案号';
COMMENT ON COLUMN "public"."siteinfo"."ali_pay_image" IS ' 支付宝收款图片';
COMMENT ON COLUMN "public"."siteinfo"."we_chat_pay_image" IS ' 微信收款图片';
COMMENT ON COLUMN "public"."siteinfo"."self_description" IS ' 个人介绍';
COMMENT ON COLUMN "public"."siteinfo"."self_description_html" IS ' 个人介绍';

-- ----------------------------
-- Table structure for tags
-- ----------------------------
DROP TABLE IF EXISTS "public"."tags";
CREATE TABLE "public"."tags" (
                                 "id" int8 NOT NULL DEFAULT nextval('tags_id_seq'::regclass),
                                 "created_at" timestamptz(6),
                                 "updated_at" timestamptz(6),
                                 "is_delete" bool DEFAULT false,
                                 "name" text COLLATE "pg_catalog"."default" NOT NULL,
                                 "tag_type" varchar(255) COLLATE "pg_catalog"."default" NOT NULL
)
;
ALTER TABLE "public"."tags" OWNER TO "postgres";
COMMENT ON COLUMN "public"."tags"."name" IS 'tag name';
COMMENT ON COLUMN "public"."tags"."tag_type" IS '标签类型';

-- ----------------------------
-- Table structure for user
-- ----------------------------
DROP TABLE IF EXISTS "public"."user";
CREATE TABLE "public"."user" (
                                 "id" int8 NOT NULL DEFAULT nextval('user_id_seq'::regclass),
                                 "created_at" timestamptz(6),
                                 "updated_at" timestamptz(6),
                                 "is_delete" bool DEFAULT false,
                                 "user_name" text COLLATE "pg_catalog"."default" NOT NULL,
                                 "password" text COLLATE "pg_catalog"."default" NOT NULL,
                                 "avatar" text COLLATE "pg_catalog"."default" NOT NULL,
                                 "label" int8 NOT NULL DEFAULT 0,
                                 "email" text COLLATE "pg_catalog"."default" NOT NULL,
                                 "github_id" int8 NOT NULL,
                                 "github_url" text COLLATE "pg_catalog"."default" NOT NULL,
                                 "is_admin" bool DEFAULT false,
                                 "receive_update" bool DEFAULT true,
                                 "last_login" timestamptz(6)
)
;
ALTER TABLE "public"."user" OWNER TO "postgres";
COMMENT ON COLUMN "public"."user"."user_name" IS ' 用户名';
COMMENT ON COLUMN "public"."user"."password" IS ' 密码';
COMMENT ON COLUMN "public"."user"."avatar" IS ' 头像';
COMMENT ON COLUMN "public"."user"."label" IS ' 标签';
COMMENT ON COLUMN "public"."user"."email" IS ' 邮箱';
COMMENT ON COLUMN "public"."user"."github_id" IS ' githubID';
COMMENT ON COLUMN "public"."user"."github_url" IS ' github地址';

-- ----------------------------
-- Indexes structure for table article
-- ----------------------------
CREATE INDEX "idx_article_category_id" ON "public"."article" USING btree (
    "category_id" "pg_catalog"."int8_ops" ASC NULLS LAST
    );
CREATE INDEX "idx_article_click_times" ON "public"."article" USING btree (
    "click_times" "pg_catalog"."int8_ops" ASC NULLS LAST
    );
CREATE INDEX "idx_article_collect_count" ON "public"."article" USING btree (
    "collect_count" "pg_catalog"."int8_ops" ASC NULLS LAST
    );
CREATE INDEX "idx_article_comment_count" ON "public"."article" USING btree (
    "comment_count" "pg_catalog"."int8_ops" ASC NULLS LAST
    );
CREATE INDEX "idx_article_id" ON "public"."article" USING btree (
    "id" "pg_catalog"."int8_ops" ASC NULLS LAST
    );
CREATE INDEX "idx_article_like_count" ON "public"."article" USING btree (
    "like_count" "pg_catalog"."int8_ops" ASC NULLS LAST
    );
CREATE INDEX "idx_article_title" ON "public"."article" USING btree (
    "title" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST
    );
CREATE INDEX "idx_article_user_id" ON "public"."article" USING btree (
    "user_id" "pg_catalog"."int8_ops" ASC NULLS LAST
    );

-- ----------------------------
-- Primary Key structure for table article
-- ----------------------------
ALTER TABLE "public"."article" ADD CONSTRAINT "article_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Indexes structure for table category
-- ----------------------------
CREATE INDEX "idx_category_display_name" ON "public"."category" USING btree (
    "display_name" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST
    );
CREATE INDEX "idx_category_id" ON "public"."category" USING btree (
    "id" "pg_catalog"."int8_ops" ASC NULLS LAST
    );
CREATE INDEX "idx_category_parent_id" ON "public"."category" USING btree (
    "parent_id" "pg_catalog"."int8_ops" ASC NULLS LAST
    );

-- ----------------------------
-- Primary Key structure for table category
-- ----------------------------
ALTER TABLE "public"."category" ADD CONSTRAINT "category_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Indexes structure for table collection
-- ----------------------------
CREATE INDEX "idx_collection_id" ON "public"."collection" USING btree (
    "id" "pg_catalog"."int8_ops" ASC NULLS LAST
    );
CREATE INDEX "user_article_id" ON "public"."collection" USING btree (
    "user_id" "pg_catalog"."int8_ops" ASC NULLS LAST,
    "article_id" "pg_catalog"."int8_ops" ASC NULLS LAST
    );

-- ----------------------------
-- Primary Key structure for table collection
-- ----------------------------
ALTER TABLE "public"."collection" ADD CONSTRAINT "collection_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Indexes structure for table comment
-- ----------------------------
CREATE INDEX "idx_comment_article_id" ON "public"."comment" USING btree (
    "article_id" "pg_catalog"."int8_ops" ASC NULLS LAST
    );
CREATE INDEX "idx_comment_id" ON "public"."comment" USING btree (
    "id" "pg_catalog"."int8_ops" ASC NULLS LAST
    );
CREATE INDEX "idx_comment_parent_id" ON "public"."comment" USING btree (
    "parent_id" "pg_catalog"."int8_ops" ASC NULLS LAST
    );
CREATE INDEX "idx_comment_user_id" ON "public"."comment" USING btree (
    "user_id" "pg_catalog"."int8_ops" ASC NULLS LAST
    );

-- ----------------------------
-- Primary Key structure for table comment
-- ----------------------------
ALTER TABLE "public"."comment" ADD CONSTRAINT "comment_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Indexes structure for table like
-- ----------------------------
CREATE INDEX "idx_like_id" ON "public"."like" USING btree (
    "id" "pg_catalog"."int8_ops" ASC NULLS LAST
    );

-- ----------------------------
-- Primary Key structure for table like
-- ----------------------------
ALTER TABLE "public"."like" ADD CONSTRAINT "like_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Indexes structure for table link
-- ----------------------------
CREATE INDEX "idx_link_id" ON "public"."link" USING btree (
    "id" "pg_catalog"."int8_ops" ASC NULLS LAST
    );
CREATE INDEX "idx_link_title" ON "public"."link" USING btree (
    "title" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST
    );

-- ----------------------------
-- Uniques structure for table link
-- ----------------------------
ALTER TABLE "public"."link" ADD CONSTRAINT "link_email_key" UNIQUE ("email");

-- ----------------------------
-- Primary Key structure for table link
-- ----------------------------
ALTER TABLE "public"."link" ADD CONSTRAINT "link_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Indexes structure for table request
-- ----------------------------
CREATE INDEX "idx_request_id" ON "public"."request" USING btree (
    "id" "pg_catalog"."int8_ops" ASC NULLS LAST
    );

-- ----------------------------
-- Primary Key structure for table request
-- ----------------------------
ALTER TABLE "public"."request" ADD CONSTRAINT "request_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Indexes structure for table resource
-- ----------------------------
CREATE INDEX "idx_resource_id" ON "public"."resource" USING btree (
    "id" "pg_catalog"."int8_ops" ASC NULLS LAST
    );

-- ----------------------------
-- Primary Key structure for table resource
-- ----------------------------
ALTER TABLE "public"."resource" ADD CONSTRAINT "resource_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Indexes structure for table reward
-- ----------------------------
CREATE INDEX "idx_reward_id" ON "public"."reward" USING btree (
    "id" "pg_catalog"."int8_ops" ASC NULLS LAST
    );
CREATE INDEX "idx_reward_payment_method" ON "public"."reward" USING btree (
    "payment_method" "pg_catalog"."int8_ops" ASC NULLS LAST
    );

-- ----------------------------
-- Uniques structure for table reward
-- ----------------------------
ALTER TABLE "public"."reward" ADD CONSTRAINT "reward_order_id_key" UNIQUE ("order_id");

-- ----------------------------
-- Primary Key structure for table reward
-- ----------------------------
ALTER TABLE "public"."reward" ADD CONSTRAINT "reward_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Indexes structure for table siteinfo
-- ----------------------------
CREATE INDEX "idx_siteinfo_id" ON "public"."siteinfo" USING btree (
    "id" "pg_catalog"."int8_ops" ASC NULLS LAST
    );

-- ----------------------------
-- Primary Key structure for table siteinfo
-- ----------------------------
ALTER TABLE "public"."siteinfo" ADD CONSTRAINT "siteinfo_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Indexes structure for table tags
-- ----------------------------
CREATE INDEX "idx_tags_id" ON "public"."tags" USING btree (
    "id" "pg_catalog"."int8_ops" ASC NULLS LAST
    );

-- ----------------------------
-- Uniques structure for table tags
-- ----------------------------
ALTER TABLE "public"."tags" ADD CONSTRAINT "tags_name_key" UNIQUE ("name");

-- ----------------------------
-- Primary Key structure for table tags
-- ----------------------------
ALTER TABLE "public"."tags" ADD CONSTRAINT "tags_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Indexes structure for table user
-- ----------------------------
CREATE INDEX "idx_user_id" ON "public"."user" USING btree (
    "id" "pg_catalog"."int8_ops" ASC NULLS LAST
    );

-- ----------------------------
-- Uniques structure for table user
-- ----------------------------
ALTER TABLE "public"."user" ADD CONSTRAINT "user_user_name_key" UNIQUE ("user_name");
ALTER TABLE "public"."user" ADD CONSTRAINT "user_email_key" UNIQUE ("email");

-- ----------------------------
-- Primary Key structure for table user
-- ----------------------------
ALTER TABLE "public"."user" ADD CONSTRAINT "user_pkey" PRIMARY KEY ("id");
