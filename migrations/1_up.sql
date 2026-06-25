PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS sessions (
  id TEXT PRIMARY KEY,
  username TEXT NOT NULL,
  password_hash TEXT NOT NULL,
  debug_pipe_id TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  initial_at TEXT,
  expires_at TEXT
);

CREATE TABLE IF NOT EXISTS settings (
  name TEXT PRIMARY KEY,
  value TEXT
);

CREATE TABLE IF NOT EXISTS class (
    id TEXT PRIMARY KEY,
    class_id TEXT NOT NULL UNIQUE,

    name TEXT NOT NULL,
    base_class_id TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    menu_caption TEXT NOT NULL,
    class_interface TEXT NOT NULL,
    flags TEXT NOT NULL,

    is_kernel_type INTEGER NOT NULL DEFAULT 0 CHECK (is_kernel_type IN (0, 1)),
    is_accessible INTEGER NOT NULL DEFAULT 0 CHECK (is_accessible IN (0, 1)),

    pad_length INTEGER,
    data_size INTEGER,
    data_precision INTEGER,
    properties TEXT
);

CREATE TABLE IF NOT EXISTS view (
    id INTEGER PRIMARY KEY,
    class_id INTEGER NOT NULL,

    name TEXT NOT NULL,
    short_name TEXT NOT NULL,
    properties TEXT NOT NULL,
    distance INTEGER NOT NULL DEFAULT 0,
    object_rights INTEGER NOT NULL DEFAULT 0,

    is_default INTEGER NOT NULL DEFAULT 0 CHECK (is_default IN (0, 1)),
    to_printer INTEGER NOT NULL DEFAULT 0 CHECK (to_printer IN (0, 1)),
    to_file INTEGER NOT NULL DEFAULT 0 CHECK (to_file IN (0, 1)),

    cell_style_script TEXT,
    filter_method_short_name TEXT,
    filter_method_properties TEXT,
    hints TEXT,
    order_by TEXT,

    FOREIGN KEY(class_id) REFERENCES class(id)
);

CREATE TABLE IF NOT EXISTS column (
    id INTEGER PRIMARY KEY,
    view_id INTEGER NOT NULL,
    alias TEXT NOT NULL UNIQUE,

    name TEXT NOT NULL,
    width INTEGER NOT NULL,
    position INTEGER NOT NULL,
    qual TEXT NOT NULL,
    base TEXT NOT NULL,

    align INTEGER NOT NULL DEFAULT 0 CHECK (align IN (0, 1, 2)),
    is_invisible INTEGER NOT NULL DEFAULT 0 CHECK (is_invisible IN (0, 2)),

    is_sizeable INTEGER NOT NULL DEFAULT 0 CHECK (is_sizeable IN (0, 1)),
    is_cell_style INTEGER NOT NULL DEFAULT 0 CHECK (is_cell_style IN (0, 1)),

    reference_type INTEGER CHECK (reference_type IN (0, 1)),
    is_editable INTEGER CHECK (reference_type IN (0, 1)),
    ability_perform_operation INTEGER CHECK (reference_type IN (0, 1)),

    target_class_id TEXT,
    reference_id TEXT,
    logging TEXT CHECK (reference_type IN ("0", "D")),

    FOREIGN KEY(view_id) REFERENCES view(id)
);

CREATE TABLE IF NOT EXISTS row_item (
    id INTEGER PRIMARY KEY,
    row_id INTEGER NOT NULL,
    view_id INTEGER NOT NULL,

    name TEXT NOT NULL,
    value TEXT NOT NULL,

    FOREIGN KEY(view_id) REFERENCES view(id)
);

INSERT INTO settings
VALUES  ("test1", "test1"),
        ("test2", "test2");

INSERT INTO class(id, class_id, name, base_class_id, entity_id, menu_caption, class_interface, is_accessible, flags)
VALUES  (0, "USER",    "Пользователи",     "STRUCTURE", "USER",    "По&льзователи",  "Z#USER#INTERFACE.CLASS#USER",        1, "0100101110100000000000000"),
        (1, "CL_PRIV", "Физические лица",  "STRUCTURE", "CLIENT",  "&Клиенты",       "Z#CL_PRIV#INTERFACE.CLASS#CL_PRIV",  1, "0100110000100010000010000")
;

INSERT INTO view(id, class_id, name, short_name, properties, is_default, to_printer, to_file)
VALUES  -- ::[USER].[VW_CRIT_USER] "Полный список"
        (4384,  0, "Полный список",   "VW_CRIT_USER",   "|AllMethods Y|HasClass|NotObjects|PlPlus|USERCONTEXT 0|", 1, 1, 1),
        -- ::[USER].[VW_CRIT_USER_2] "Полный список 2"
        (1,     0, "Полный список 2", "VW_CRIT_USER_2", "|AllMethods Y|HasClass|NotObjects|PlPlus|USERCONTEXT 0|", 0, 1, 1)
;

INSERT INTO column(view_id, name, width, position, qual, alias, base, is_editable, is_sizeable, is_invisible, logging, ability_perform_operation, target_class_id, reference_type, reference_id)
VALUES  -- ::[USER].[VW_CRIT_USER] "Полный список"
        (4384, "Фамилия Имя Отчество",  "21", "1", "NAME",              "C_FIO",      "STRING", "0", "1", "0", "0", "1", NULL,      NULL, NULL               ),
        (4384, "Сетевое имя",           "9",  "2", "USERNAME",          "C_USERNAME", "STRING", "0", "1", "0", "0", "1", NULL,      NULL, NULL               ),
        (4384, "Физическое лицо",       "21", "3", "CL_PRIV_REF.NAME",  "C_NAME_1",   "STRING", "0", "1", "0", "0", "1", "CL_PRIV", 0,    "a1.C_CL_PRIV_REF" ),
        (4384, "id",                    "4",  "4", "ID",                "C_ID",       "NUMBER", "0", "1", "2", "0", "1", NULL,      NULL, NULL               )
;

INSERT INTO row_item(view_id, row_id, name, value)
VALUES  -- ::[USER].[VW_CRIT_USER] "Полный список"
        (4384, 0, "ID", "2350467263"), (4384, 0, "C_1", "Тест Тест Тестович"), (4384, 0, "C_2", "TEST"), (4384, 0, "C_3", "22738342"),
        (4384, 1, "ID", "2350467263"), (4384, 1, "C_1", "Тест Тест Тестович"), (4384, 1, "C_2", "TEST"), (4384, 1, "C_3", "22738342")
;
