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
  is_kernel_type INTEGER NOT NULL DEFAULT 0 CHECK (is_kernel_type IN (0, 1)),
  class_interface TEXT NOT NULL,
  flags TEXT NOT NULL,

  is_accessible INTEGER CHECK (is_accessible IN (0, 1)),
  pad_length INTEGER,
  data_size INTEGER,
  data_precision INTEGER,
  properties TEXT,
  group_id TEXT
);

CREATE TABLE IF NOT EXISTS method (
  id INTEGER PRIMARY KEY,
  class_id INTEGER NOT NULL,

  name TEXT NOT NULL,
  short_name TEXT NOT NULL,
  type TEXT NOT NULL,
  form_class_id TEXT NOT NULL,
  properties TEXT NOT NULL,
  distance INTEGER NOT NULL DEFAULT 0,
  callable_short_name TEXT NOT NULL,

  script_id TEXT,
  result_class_id TEXT,
  user_driven INTEGER,
  form_id INTEGER,
  report_type TEXT,
  report_template TEXT,

  FOREIGN KEY(class_id) REFERENCES class(id)
);

CREATE TABLE IF NOT EXISTS method_parameter (
  id INTEGER PRIMARY KEY,
  method_id INTEGER NOT NULL,

  short_name TEXT NOT NULL,
  class_id TEXT NOT NULL,
  position INTEGER NOT NULL,
  reference_type TEXT NOT NULL,
  direction TEXT NOT NULL,
  default_value TEXT,

  FOREIGN KEY(method_id) REFERENCES method(id)
);

CREATE TABLE IF NOT EXISTS method_variable (
  id INTEGER PRIMARY KEY,
  method_id INTEGER NOT NULL,

  short_name TEXT NOT NULL,
  class_id TEXT NOT NULL,
  position INTEGER NOT NULL,
  reference_type TEXT NOT NULL,

  FOREIGN KEY(method_id) REFERENCES method(id)
);

CREATE TABLE IF NOT EXISTS method_control (
  id INTEGER PRIMARY KEY,
  method_id INTEGER NOT NULL,

  control TEXT NOT NULL,
  caption TEXT NOT NULL,
  top INTEGER NOT NULL,
  left INTEGER NOT NULL,
  height INTEGER NOT NULL,
  width INTEGER NOT NULL,
  tab_index INTEGER NOT NULL,
  position INTEGER NOT NULL,
  validate_name TEXT NOT NULL,

  qualifier TEXT,
  parent_id INTEGER,
  class_id TEXT,
  depend INTEGER,
  properties TEXT,
  tips TEXT,

  FOREIGN KEY(method_id) REFERENCES method(id)
);

CREATE TABLE IF NOT EXISTS view (
  id INTEGER PRIMARY KEY,
  class_id INTEGER NOT NULL,

  name TEXT NOT NULL,
  short_name TEXT NOT NULL,
  is_default INTEGER NOT NULL DEFAULT 0 CHECK (is_default IN (0, 1)),
  properties TEXT NOT NULL,
  distance INTEGER NOT NULL DEFAULT 0,
  object_rights INTEGER NOT NULL DEFAULT 0,
  to_printer INTEGER NOT NULL DEFAULT 0 CHECK (to_printer IN (0, 1)),
  to_file INTEGER NOT NULL DEFAULT 0 CHECK (to_file IN (0, 1)),

  order_by TEXT,
  hints TEXT,
  cell_style_script TEXT,
  source_id INTEGER,
  extension_id INTEGER,
  filter_method_short_name TEXT,
  filter_method_properties TEXT,

  FOREIGN KEY(class_id) REFERENCES class(id)
  FOREIGN KEY(source_id) REFERENCES view(id)
  FOREIGN KEY(extension_id) REFERENCES view(id)
);

CREATE TABLE IF NOT EXISTS column (
  id INTEGER PRIMARY KEY,
  view_id INTEGER NOT NULL,

  name TEXT NOT NULL,
  width INTEGER NOT NULL,
  align INTEGER NOT NULL DEFAULT 0 CHECK (align IN (0, 1, 2)),
  position INTEGER NOT NULL,
  qual TEXT NOT NULL,
  alias TEXT NOT NULL,
  base TEXT NOT NULL,
  is_sizeable INTEGER NOT NULL DEFAULT 0 CHECK (is_sizeable IN (0, 1)),
  is_cell_style INTEGER NOT NULL DEFAULT 0 CHECK (is_cell_style IN (0, 1)),
  is_invisible INTEGER NOT NULL DEFAULT 0 CHECK (is_invisible IN (0, 2)),
  ability_perform_operation INTEGER NOT NULL DEFAULT 0 CHECK (reference_type IN (0, 1)),

  is_editable INTEGER CHECK (reference_type IN (0, 1)),
  reference_id TEXT,
  target_class_id TEXT,
  reference_type INTEGER CHECK (reference_type IN (0, 1)),
  logging TEXT CHECK (reference_type IN ("0", "D")),

  FOREIGN KEY(view_id) REFERENCES view(id)
);

CREATE TABLE IF NOT EXISTS row_item (
  id INTEGER PRIMARY KEY,
  object_id INTEGER NOT NULL,
  view_id INTEGER NOT NULL,

  name TEXT NOT NULL,
  value TEXT NOT NULL,

  FOREIGN KEY(view_id) REFERENCES view(id)
);

INSERT INTO settings
VALUES  ("test1", "test1"),
        ("test2", "test2");

INSERT INTO class(
        id, class_id,       name,               base_class_id,  entity_id, menu_caption,        class_interface,                        flags,                          is_accessible,  pad_length, data_size,  data_precision, properties, group_id)
VALUES  (0, "USER",         "Пользователи",     "STRUCTURE",    "USER",    "По&льзователи",     "Z#USER#INTERFACE.CLASS#USER",          "0100101110100000000000000",    1,              NULL,       NULL,       NULL,           NULL,       NULL),
        (1, "CL_PRIV",      "Физические лица",  "STRUCTURE",    "CLIENT",  "&Клиенты",          "Z#CL_PRIV#INTERFACE.CLASS#CL_PRIV",    "0100110000100010000010000",    1,              NULL,       NULL,       NULL,           NULL,       NULL),
        (2, "CL_ORG",       "Организации",      "STRUCTURE",    "CLIENT",  "&Клиенты",          "Z#CL_ORG#INTERFACE.CLASS#CL_ORG",      "0100110000100000000010000",    1,              NULL,       NULL,       NULL,           NULL,       NULL),
        (3, "NAME",         "Наименование",     "STRING",       "TYPE",    "Сп&равочники",      "VARCHAR2",                             "0000000000100000000000000",    1,              NULL,       "100",      NULL,           NULL,       NULL),
        (4, "STRING_30",    "STRING_30",        "STRING",       "TYPE",    "Сп&равочники",      "VARCHAR2",                             "0000000000100000000000000",    1,              NULL,       "30",       NULL,           NULL,       NULL),
        (5, "BOOLEAN",      "Логика",           "BOOLEAN",      "TYPE",    "Сп&равочники",      "BOOLEAN",                              "0000000000100000000000000",    1,              NULL,       "1",        NULL,           NULL,       NULL)
;

INSERT INTO method(
        id,     class_id,   name,                   short_name, type,   form_class_id,  callable_short_name,    script_id,  result_class_id,    user_driven, form_id,   report_type,    report_template,    properties)
VALUES  -- ::[USER].[TEST] "Тестовая операция"
        (311,   0,          "Тестовая операция",    "TEST",     "M",    "USER",         "TEST",                 NULL,       NULL,               1,           NULL,      NULL,           NULL,               "|ARCHPACK 0|BUF |CMD Y|COMMIT N|COMPILER 01111011111111FF0000|CREATE N|CRITICAL N|DEFHOTKEY Y|EMPTY_ID N|FOCUS_VALIDATE 2|HOTKEY 368|IPIPE |LOAD_VALIDATE 2|OPIPE |PARAMS N|PRIORITY 100|REFRESH Y|RTLBASE 807001|RUN_MON N|SHARE N|STREAM N|")
;

INSERT INTO method_parameter(
        method_id, short_name,    class_id,       position, reference_type, direction, default_value)
VALUES  -- ::[USER].[TEST] "Тестовая операция"
        (311,      "P_NAME",      "NAME",         1,        "D",            "I",       "%THIS%.NAME"),
        (311,      "P_USERNAME",  "STRING_30",    2,        "D",            "I",       "%THIS%.USERNAME")
;

INSERT INTO method_control(
        id,         method_id,  qualifier,            control,    caption,                top,  left, height, width,  tab_index,  position, validate_name,  parent_id,    class_id,     depend, properties, tips)
VALUES  -- ::[USER].[TEST] "Тестовая операция"
        (14682335,  311,        NULL,                 "FORM",     "Тест",                 0,    0,    2415,   8700,   0,          0,        "Form1",        NULL,         NULL,         NULL,   NULL,       NULL),
        (14682336,  311,        "%PARAM%.P_NAME",     "LABEL",    "Фамилия Имя Отчество", 120,  120,  360,    2580,   2,          1,        "Label1",       14682335,     NULL,         NULL,   NULL,       "Поясняющая надпись (Фамилия Имя Отчество)"),
        (14682337,  311,        "%PARAM%.P_NAME",     "TEXT",     "",                     120,  2760, 360,    5700,   3,          2,        "Text1",        14682335,     "NAME",       NULL,   "|V|",      "Фамилия Имя Отчество"),
        (14682338,  311,        "%PARAM%.P_USERNAME", "LABEL",    "Сетевое имя",          540,  120,  360,    2580,   4,          3,        "Label2",       14682335,     NULL,         NULL,   NULL,       "Поясняющая надпись (Сетевое имя)"),
        (14682339,  311,        "%PARAM%.P_USERNAME", "TEXT",     "",                     540,  2760, 360,    5700,   5,          4,        "Text2",        14682335,     "STRING_30",  NULL,   "|V|",      "Сетевое имя"),
        (14682344,  311,        "OK",                 "BUTTON",   "OK",                   1740, 3780, 360,    2160,   6,          5,        "OK",           14682335,     NULL,         NULL,   NULL,       "Выполнить метод"),
        (14682345,  311,        "CANCEL",             "BUTTON",   "Отмена",               1740, 6060, 360,    2160,   7,          6,        "CANCEL",       14682335,     NULL,         NULL,   NULL,       "Отказ от выполнения")
;

INSERT INTO view(id, class_id, name, short_name, properties, is_default, to_printer, to_file, source_id, extension_id)
VALUES  -- ::[USER].[VW_CRIT_USER] "Полный список"
        (4384,          0, "Полный список",                 "VW_CRIT_USER",         "|AllMethods Y|HasClass|NotObjects|PlPlus|USERCONTEXT 0|",                      1, 1, 1, NULL, NULL),
        -- ::[CL_PRIV].[VW_CRIT_CL_PRIV] "Полный список"
        (3616,          1, "Полный список",                 "VW_CRIT_CL_PRIV",      "|AllMethods Y|HasClass|NotObjects|PlPlus|ShowChild|USERCONTEXT 1|",            1, 1, 1, NULL, 4172642368),
        (4172642368,    1, "Полный список (расширение)",    "VW_CRIT_CL_PRIV_EXT",  "|AllMethods Y|HasClass|NotObjects|PlPlus|ShowChild|USERCONTEXT 1|",            0, 1, 1, 3616, NULL),
        -- ::[CL_PRIV].[VW_CRIT_CL_PRIV] "Полный список"
        (4522,          2, "Полный список",                 "VW_CRIT_CL_ORG",       "|AllMethods Y|GENERATE_ANSI_JOINS|HasClass|NotObjects|PlPlus|USERCONTEXT 0|",  1, 1, 1, NULL, NULL)
;

INSERT INTO column(view_id, name, width, position, qual, alias, base, is_sizeable, is_invisible, logging, ability_perform_operation, target_class_id, reference_type, reference_id)
VALUES  -- ::[USER].[VW_CRIT_USER] "Полный список"
        (4384,          "Фамилия Имя Отчество", 21, 1, "NAME",              "C_FIO",        "STRING", 1, 0, "0",     1, NULL,        NULL, NULL                  ),
        (4384,          "Сетевое имя",          9,  2, "USERNAME",          "C_USERNAME",   "STRING", 1, 0, "0",     1, NULL,        NULL, NULL                  ),
        (4384,          "Физическое лицо",      21, 3, "CL_PRIV_REF.NAME",  "C_NAME_1",     "STRING", 1, 0, "0",     1, "CL_PRIV",   0,    "a1.C_CL_PRIV_REF"    ),
        (4384,          "id",                   4,  4, "ID",                "C_ID",         "NUMBER", 1, 2, "0",     1, NULL,        NULL, NULL                  ),
        -- ::[CL_PRIV].[VW_CRIT_CL_PRIV] "Полный список"
        (3616,          "ID",                   17, 1, "ID",                "C_ID",         "NUMBER", 1, 2, "D",     1, NULL,        NULL, NULL                  ),
        (3616,          "Ф.И.О.",               25, 2, "NAME",              "C_NAME",       "STRING", 1, 0, "D",     1, NULL,        NULL, NULL                  ),
        -- ::[CL_PRIV].[VW_CRIT_CL_PRIV_EXT] "Полный список (расширение)"
        (4172642368,    "ID",                   17, 1, "ID",                "C_ID",         "NUMBER", 1, 2, "0",     1, NULL,        NULL, NULL                  ),
        (4172642368,    "Ф.И.О.",               25, 2, "NAME",              "C_NAME",       "STRING", 1, 0, "0",     1, NULL,        NULL, NULL                  ),
        (4172642368,    "Расширение",           25, 3, "EXT",               "C_EXT",        "STRING", 1, 0, "0",     1, NULL,        NULL, NULL                  ),
        -- ::[CL_ORG].[VW_CRIT_CL_ORG] "Полный список"
        (4522,          "ID",                   17, 1, "ID",                "C_ID",         "NUMBER", 1, 2, "D",     1, NULL,        NULL, NULL                  ),
        (4522,          "Наименование",         25, 2, "NAME",              "C_NAME",       "STRING", 1, 0, "D",     1, NULL,        NULL, NULL                  )
;

INSERT INTO row_item(view_id, object_id, name, value)
VALUES  -- ::[USER].[VW_CRIT_USER] "Полный список"
        (4384, 22738256, "ID", "0"), (4384, 22738256, "C_1", "Тест Тест Тестович"), (4384, 22738256, "C_2", "TEST"), (4384, 22738256, "REF3", "1"),
        -- ::[CL_PRIV].[VW_CRIT_CL_PRIV] "Полный список"
        (3616, 1, "ID", "1"), (3616, 1, "C_1", "Тест Тест Тестович"),
        -- ::[CL_PRIV].[VW_CRIT_CL_PRIV_EXT] "Полный список (расширение)"
        (4172642368, 1, "ID", "1"), (4172642368, 1, "C_1", "Тест Тест Тестович"), (4172642368, 1, "C_2", "Расширение"),
        -- ::[CL_ORG].[VW_CRIT_CL_ORG] "Полный список"
        (4522, 2, "ID", "1"), (4522, 2, "C_1", 'ООО "Тест"')
;
