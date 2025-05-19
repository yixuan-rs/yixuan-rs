CREATE TABLE t_account_uid (
        uid int primary key generated always as identity,
        account_uid varchar(64) NOT NULL,
        UNIQUE(account_uid)
);

CREATE TABLE t_basic_data (
        uid INT PRIMARY KEY NOT NULL,
        nick_name varchar(32) NOT NULL,
        name_change_times INT NOT NULL,
        level INT NOT NULL,
        exp INT NOT NULL,
        create_time BIGINT NOT NULL,
        avatar_id INT NOT NULL,
        control_avatar_id INT NOT NULL,
        control_guise_avatar_id INT NOT NULL,
        portrait_id INT NOT NULL
);

CREATE TABLE t_avatar_data (
        uid INT PRIMARY KEY NOT NULL,
        data BYTEA NOT NULL
);

CREATE TABLE t_item_data (
        uid INT PRIMARY KEY NOT NULL,
        data BYTEA NOT NULL
);

CREATE TABLE t_quest_data (
        uid INT PRIMARY KEY NOT NULL,
        data BYTEA NOT NULL
);

CREATE TABLE t_archive_data (
        uid INT PRIMARY KEY NOT NULL,
        data BYTEA NOT NULL
);

CREATE TABLE t_hollow_data (
        uid INT PRIMARY KEY NOT NULL,
        data BYTEA NOT NULL
);

CREATE TABLE t_abyss_data (
        uid INT PRIMARY KEY NOT NULL,
        data BYTEA NOT NULL
);

CREATE TABLE t_buddy_data (
        uid INT PRIMARY KEY NOT NULL,
        data BYTEA NOT NULL
);

CREATE TABLE t_misc_data (
        uid INT PRIMARY KEY NOT NULL,
        data BYTEA NOT NULL
);

CREATE TABLE t_main_city_data (
        uid INT PRIMARY KEY NOT NULL,
        data BYTEA NOT NULL
);

CREATE TABLE t_scene_data (
        uid INT PRIMARY KEY NOT NULL,
        data BYTEA NOT NULL
);

CREATE TABLE t_gacha_data (
        uid INT PRIMARY KEY NOT NULL,
        data BYTEA NOT NULL
);
