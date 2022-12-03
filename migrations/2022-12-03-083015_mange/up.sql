-- 会话表 start
CREATE TABLE "ci_sessions" (
    "id" varchar(128) NOT NULL PRIMARY KEY,
    "ip_address" inet NOT NULL,
    "timestamp" timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "data" bytea DEFAULT '' NOT NULL
);

CREATE INDEX "ci_sessions_timestamp" ON "ci_sessions" ("timestamp");
CREATE INDEX "ci_sessions_id_ip_address" ON "ci_sessions" (id, ip_address);
-- 当 sessionMatchIP = TRUE 时
-- ALTER TABLE ci_sessions ADD PRIMARY KEY (id, ip_address);
-- 当 sessionMatchIP = FALSE 时
-- ALTER TABLE ci_sessions ADD PRIMARY KEY (id);
-- 删除先前创建的主键（在更改设置时使用）
-- ALTER TABLE ci_sessions DROP PRIMARY KEY;

-- 会话表 end

-- 后台管理用户表 start
CREATE TABLE admins (
    id serial primary key,
    username CHARACTER VARYING(16) NOT NULL,
    "password" CHARACTER VARYING(40) NOT NULL,
    salt CHARACTER(10) NOT NULL,
    "email" CHARACTER VARYING(100) DEFAULT NULL,
    "mobile" CHARACTER(11) DEFAULT NULL,
    "role" smallint DEFAULT NULL,
    "status" bigint DEFAULT 0,
    "create_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp(),
    --不带时区
    -- create_time TIMESTAMP(6) WITH TIME ZONE DEFAULT clock_timestamp(), --带时区
    "last_login" TIMESTAMP WITHOUT time ZONE DEFAULT NULL
);

CREATE INDEX idx_admins_username ON admins (username);

CREATE INDEX idx_admins_email ON admins (email);

CREATE INDEX idx_admins_role ON admins (role);

COMMENT ON TABLE admins IS '后台管理用户表';

COMMENT ON COLUMN admins.id IS '自增主键ID';

COMMENT ON COLUMN admins.username IS '登录名';

COMMENT ON COLUMN admins.password IS '登录密码';

COMMENT ON COLUMN admins.salt IS '混淆码';

COMMENT ON COLUMN admins.email IS '邮箱';

COMMENT ON COLUMN admins.mobile IS '电话';

COMMENT ON COLUMN admins.role IS '角色组ID';

COMMENT ON COLUMN admins.status IS '是否冻结：0=正常，1=永久冻结，冻结时间';

COMMENT ON COLUMN admins.create_time IS '创建时间(不带时区)';

COMMENT ON COLUMN admins.last_login IS '最后登录时间(不带时区)';

INSERT INTO
    admins (
        "id",
        "username",
        "password",
        "salt",
        "email",
        "mobile",
        "role",
        "status",
        "create_time",
        "last_login"
    )
VALUES
    (
        1,
        'luck',
        'c2a3b691ee173bbaee19a5d6aae8c995507fa706',
        '25ee364a54',
        'luck@fmail.pro',
        '13428122341',
        1,
        0,
        '2008-08-18 18:58:13',
        '2018-08-18 18:58:18'
    );
-- 后台管理用户表 end

-- 后台角色表 start
CREATE TABLE roles (
    id serial primary key,
    name CHARACTER VARYING(20) NOT NULL,
    rights CHARACTER VARYING(255) DEFAULT NULL,
    "default" CHARACTER VARYING(50) DEFAULT NULL
);

-- 添加name索引
CREATE INDEX idx_roles_name ON roles (name);

COMMENT ON TABLE roles IS '后台角色表';

COMMENT ON COLUMN roles.id IS '自增主键';

COMMENT ON COLUMN roles.name IS '角色组名称';

COMMENT ON COLUMN roles.rights IS '角色组权限';

COMMENT ON COLUMN roles.default IS '角色组默认登录页';

INSERT INTO
    roles (id, name, rights, "default")
VALUES
    (1, 'root=>超级管理组', '', 'luck/index');
-- 后台角色表 end

-- 菜单表 start
CREATE TABLE menus (
    id serial primary key,
    order_by smallint NOT NULL,
    class character varying(20) DEFAULT NULL,
    method character varying(20) DEFAULT NULL,
    name character varying(20) NOT NULL,
    level smallint DEFAULT 0,
    parent SMALLINT DEFAULT 0,
    icon CHARACTER VARYING(50) DEFAULT NULL,
    department CHARACTER VARYING(20) DEFAULT NULL,
    is_show boolean DEFAULT TRUE
);

COMMENT ON TABLE menus IS '后台菜单表';

comment on column menus.id is 'ID自增主键';

comment on column menus.order_by is '排序';

comment on column menus.class is '类';

comment on column menus.method is '方法';

comment on column menus.name is '菜单名字';

COMMENT ON COLUMN menus.level IS '菜单层级';

COMMENT ON COLUMN menus.parent IS '菜单父级';

COMMENT ON COLUMN menus.icon IS '菜单左侧小图标';

COMMENT ON COLUMN menus.department IS '菜单所属顶级';

COMMENT ON COLUMN menus.is_show IS '菜单是否显示：默认true显示，false不显示';
 
INSERT INTO
    menus (
        id,
        order_by,
        class,
        method,
        name,
        level,
        parent,
        icon,
        department,
        is_show
    )
VALUES
    (
        1,
        1,
        'luck',
        'index',
        '后台首页',
        1,
        0,
        'fa-desktop',
        '1',
        TRUE
    ),
    (
        2,
        8,
        '',
        '',
        '系统设置',
        1,
        0,
        'fa-cogs',
        '请选择',
        TRUE
    ),
    (
        3,
        1,
        'menus',
        'index',
        '菜单管理',
        2,
        2,
        'fa-folder',
        '2',
        TRUE
    ),
    (
        4,
        4,
        'record',
        'index',
        '操作日志',
        2,
        2,
        NULL,
        '2',
        TRUE
    ),
    (
        5,
        3,
        'roles',
        'index',
        '角色管理',
        2,
        2,
        'fa-key',
        '2',
        TRUE
    ),
    (
        6,
        2,
        'admins',
        'index',
        '后台用户',
        2,
        2,
        'ssssss',
        '2',
        TRUE
    ),
    (
        7,
        2,
        'nav',
        'index',
        '导航菜单',
        1,
        0,
        'fa-fire',
        '请选择',
        TRUE
    ),
    (
        8,
        2,
        'article',
        'index',
        '文章管理',
        1,
        0,
        'fa-leanpub',
        '请选择',
        TRUE
    );
-- 菜单表 end

-- 权限表 start
CREATE TABLE rights (
    "right_id" SERIAL PRIMARY KEY,
    "right_name" CHARACTER VARYING(30) DEFAULT NULL,
    "right_class" CHARACTER VARYING(30) DEFAULT NULL,
    "right_method" CHARACTER VARYING(30) DEFAULT NULL,
    "right_detail" CHARACTER VARYING(30) DEFAULT NULL
);

COMMENT ON TABLE rights IS '权限表';
-- 权限表 end

-- 日志表 start
-- 内置分区表 (record) ,每月要新添加分区表。（或者每年也是行的。）
CREATE TABLE record(
    "id" SERIAL,
    "table_id" integer NOT NULL,
    "table_name" CHARACTER VARYING(180) NOT NULL,
    "user_id" integer NOT NULL,
    "username" CHARACTER VARYING(18) NOT NULL,
    "action" CHARACTER VARYING(180) NOT NULL,
    "ip" inet NOT NULL,
    "record_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp() PRIMARY KEY
) PARTITION BY RANGE(record_time);

COMMENT ON TABLE record IS '操作记录表-采用了内置分区表，按月分表';

COMMENT ON COLUMN record.table_id IS '操作表ID';

COMMENT ON COLUMN record.table_name IS '操作表名';

COMMENT ON COLUMN record.user_id IS '操作用户ID';

COMMENT ON COLUMN record.username IS '操作用户名';

COMMENT ON COLUMN record.action IS '操作动作';

COMMENT ON COLUMN record.ip IS '操作IP';

COMMENT ON COLUMN record.record_time IS '操作时间';

-- 创建当月日志分区表
CREATE TABLE record_2022 PARTITION OF record FOR
VALUES
FROM
    ('2022-01-01') TO ('2023-01-01');

-- 创建索引
CREATE INDEX idx_record_2022_rtime ON record_2022 (record_time);

CREATE INDEX idx_record_2022_user_id ON record_2022 (user_id);
-- 日志表 end
