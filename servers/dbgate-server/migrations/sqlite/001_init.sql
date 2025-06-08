CREATE TABLE t_account_uid (
        uid INTEGER PRIMARY KEY,
        account_uid varchar(64) NOT NULL,
        UNIQUE(account_uid)
);

CREATE TABLE t_basic_data (
        uid INTEGER PRIMARY KEY NOT NULL,
        nick_name varchar(32) NOT NULL,
        name_change_times INTEGER NOT NULL,
        level INTEGER NOT NULL,
        exp INTEGER NOT NULL,
        create_time BIGINTEGER NOT NULL,
        avatar_id INTEGER NOT NULL,
        control_avatar_id INTEGER NOT NULL,
        control_guise_avatar_id INTEGER NOT NULL,
        portrait_id INTEGER NOT NULL
);

CREATE TABLE t_avatar_data (
        uid INTEGER PRIMARY KEY NOT NULL,
        data BLOB NOT NULL
);

CREATE TABLE t_item_data (
        uid INTEGER PRIMARY KEY NOT NULL,
        data BLOB NOT NULL
);

CREATE TABLE t_quest_data (
        uid INTEGER PRIMARY KEY NOT NULL,
        data BLOB NOT NULL
);

CREATE TABLE t_archive_data (
        uid INTEGER PRIMARY KEY NOT NULL,
        data BLOB NOT NULL
);

CREATE TABLE t_hollow_data (
        uid INTEGER PRIMARY KEY NOT NULL,
        data BLOB NOT NULL
);

CREATE TABLE t_abyss_data (
        uid INTEGER PRIMARY KEY NOT NULL,
        data BLOB NOT NULL
);

CREATE TABLE t_buddy_data (
        uid INTEGER PRIMARY KEY NOT NULL,
        data BLOB NOT NULL
);

CREATE TABLE t_misc_data (
        uid INTEGER PRIMARY KEY NOT NULL,
        data BLOB NOT NULL
);

CREATE TABLE t_main_city_data (
        uid INTEGER PRIMARY KEY NOT NULL,
        data BLOB NOT NULL
);

CREATE TABLE t_scene_data (
        uid INTEGER PRIMARY KEY NOT NULL,
        data BLOB NOT NULL
);

CREATE TABLE t_gacha_data (
        uid INTEGER PRIMARY KEY NOT NULL,
        data BLOB NOT NULL
);

CREATE TABLE t_map_data (
        uid INTEGER PRIMARY KEY NOT NULL,
        data BLOB NOT NULL
);

CREATE TABLE t_big_scene_data (
        uid INTEGER PRIMARY KEY NOT NULL,
        data BLOB NOT NULL
);
