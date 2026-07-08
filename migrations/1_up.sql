PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS users (
  id INTEGER PRIMARY KEY,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  username TEXT NOT NULL UNIQUE,
  fullname TEXT NOT NULL,
  properties TEXT
);

CREATE TABLE IF NOT EXISTS groups (
  id INTEGER PRIMARY KEY,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  name TEXT NOT NULL,

  user_id INTEGER NOT NULL,
  FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS profiles (
  id INTEGER PRIMARY KEY,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  name TEXT NOT NULL,
  property TEXT NOT NULL,
  value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
  id TEXT PRIMARY KEY,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  debug_pipe_id TEXT,
  initial_at TEXT,
  expires_at TEXT,

  user_id INTEGER NOT NULL,
  FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS settings (
  id INTEGER PRIMARY KEY,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  name TEXT NOT NULL UNIQUE,
  value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS options (
  id INTEGER PRIMARY KEY,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  name TEXT NOT NULL UNIQUE,
  value INTEGER NOT NULL DEFAULT 0 CHECK (value IN (0, 1))
);

CREATE TABLE IF NOT EXISTS class (
  id TEXT PRIMARY KEY,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
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
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
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
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  method_id INTEGER NOT NULL,

  short_name TEXT NOT NULL,
  class_id TEXT NOT NULL,
  position INTEGER NOT NULL,
  reference_type TEXT NOT NULL,
  direction TEXT NOT NULL,

  view_id INTEGER,
  view_class_id TEXT,
  view_filter TEXT,
  default_value TEXT,

  FOREIGN KEY(method_id) REFERENCES method(id)
);

CREATE TABLE IF NOT EXISTS method_variable (
  id INTEGER PRIMARY KEY,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  method_id INTEGER NOT NULL,

  short_name TEXT NOT NULL,
  class_id TEXT NOT NULL,
  position INTEGER NOT NULL,
  reference_type TEXT NOT NULL,

  FOREIGN KEY(method_id) REFERENCES method(id)
);

CREATE TABLE IF NOT EXISTS method_control (
  id INTEGER PRIMARY KEY,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
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
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
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
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
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
  ability_perform_operation INTEGER NOT NULL DEFAULT 0 CHECK (ability_perform_operation IN (0, 1)),

  is_editable INTEGER CHECK (is_editable IN (0, 1)),
  reference_id TEXT,
  target_class_id TEXT,
  reference_type INTEGER CHECK (reference_type IN (0, 1)),
  logging TEXT CHECK (logging IN ("0", "D")),

  FOREIGN KEY(view_id) REFERENCES view(id)
);

CREATE TABLE IF NOT EXISTS row_item (
  id INTEGER PRIMARY KEY,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  object_id INTEGER NOT NULL,
  view_id INTEGER NOT NULL,

  name TEXT NOT NULL,
  value TEXT NOT NULL,

  FOREIGN KEY(view_id) REFERENCES view(id)
);

INSERT INTO users(
        id, username, fullname,             properties)
VALUES (0,  "TEST",   "Тест Тест Тестович", "|ADMIN|CONTEXT|PICKER|PROFILE DEFAULT|SESSION|")
;

INSERT INTO groups(
        name,         user_id)
VALUES ("ADMIN_GRP",  0)
;

INSERT INTO profiles(
        name,       property,             value)
VALUES ("DEFAULT",  "SESSIONS_PER_USER",  "UNLIMITED")
;

INSERT INTO settings(
        name,                 value)
VALUES  ("SHOW_SYSTEM_MENU", "YES")
;

INSERT INTO options(
        name,                 value)
VALUES  ("NAV_SKIN_INTERFACE", 1)
;

INSERT INTO class(
        id, class_id,       name,               base_class_id,  entity_id, menu_caption,        class_interface,                        flags,                          is_accessible,  pad_length, data_size,  data_precision, properties, group_id)
VALUES  (0, "USER",         "Пользователи",     "STRUCTURE",    "USER",    "По&льзователи",     "Z#USER#INTERFACE.CLASS#USER",          "0100101110100000000000000",    1,              NULL,       NULL,       NULL,           NULL,       NULL),
        (1, "CL_PRIV",      "Физические лица",  "STRUCTURE",    "CLIENT",  "&Клиенты",          "Z#CL_PRIV#INTERFACE.CLASS#CL_PRIV",    "0100110000100010000010000",    1,              NULL,       NULL,       NULL,           NULL,       NULL),
        (2, "CL_ORG",       "Организации",      "STRUCTURE",    "CLIENT",  "&Клиенты",          "Z#CL_ORG#INTERFACE.CLASS#CL_ORG",      "0100110000100000000010000",    1,              NULL,       NULL,       NULL,           NULL,       NULL),
        (3, "NAME",         "Наименование",     "STRING",       "TYPE",    "Сп&равочники",      "VARCHAR2",                             "0000000000100000000000000",    1,              NULL,       "100",      NULL,           NULL,       NULL),
        (4, "STRING_30",    "STRING_30",        "STRING",       "TYPE",    "Сп&равочники",      "VARCHAR2",                             "0000000000100000000000000",    1,              NULL,       "30",       NULL,           NULL,       NULL),
        (5, "BOOLEAN",      "Логика",           "BOOLEAN",      "TYPE",    "Сп&равочники",      "BOOLEAN",                              "0000000000100000000000000",    1,              NULL,       "1",        NULL,           NULL,       NULL),
        (6, "FP_TUNE",      "Настройки.",       "STRUCTURE",    "TYPE",    "Сп&равочники",      "Z#FP_TUNE#INTERFACE.CLASS#FP_TUNE",    "0100000000100000000000000",    1,              NULL,       NULL,       NULL,           NULL,       NULL),
        (7, "STRING_100",   "STRING_100",       "STRING",       "TYPE",    "Сп&равочники",      "VARCHAR2",                             "0000000000100000000000000",    1,              NULL,       "100",      NULL,           NULL,       NULL)
;

INSERT INTO method(
        id,         class_id,   name,                   short_name, type,   form_class_id,  callable_short_name,    script_id,  result_class_id,    user_driven, form_id,       report_type,    report_template,    properties)
VALUES  -- ::[USER].[TEST] "Тестовая операция"
        (311,       0,          "Тестовая операция",    "NEW#AUTO", "M",    "USER",         "TEST",                 NULL,       NULL,               1,           NULL,          NULL,           NULL,               "|ARCHPACK 0|BUF |CMD Y|COMMIT N|COMPILER 01111011111111FF0000|CREATE N|CRITICAL N|DEFHOTKEY Y|EMPTY_ID N|FOCUS_VALIDATE 2|HOTKEY 368|IPIPE |LOAD_VALIDATE 2|OPIPE |PARAMS N|PRIORITY 100|REFRESH Y|RTLBASE 807001|RUN_MON N|SHARE N|STREAM N|"),
        (6503609,   6,          "Добавить",             "NEW#AUTO", "C",    "FP_TUNE",      "NEW#AUTO",             NULL,       "FP_TUNE",          1,           "6503609",     NULL,           NULL,               "|ARCHPACK 0|BUF |CMD N|COMMIT N|COMPILER 01011011110111FF0010|CREATE Y|CRITICAL N|DEFHOTKEY N|EMPTY_ID N|FOCUS_VALIDATE 2|HOTKEY 00|IPIPE |LOAD_VALIDATE 2|OPIPE |PARAMS N|PRIORITY 100|REFRESH N|RTLBASE 212801|RUN_MON N|SHARE N|STREAM N|")
;

INSERT INTO method_parameter(
        method_id, short_name,      class_id,           position,   reference_type, direction, default_value,       view_id,    view_class_id,  view_filter)
VALUES  -- ::[USER].[NEW#AUTO] "Тестовая операция"
        (311,      "P_NAME",        "NAME",             1,          "D",            "I",       "%THIS%.NAME",       NULL,       NULL,           NULL                                        ),
        (311,      "P_USERNAME",    "STRING_30",        2,          "D",            "I",       "%THIS%.USERNAME",   NULL,       NULL,           NULL                                        ),
        -- ::[FP_TUNE].[NEW#AUTO] "Добавить"
        (6503609,   "P_CODE",       "STRING_30",        1,          "D" ,           "I",       NULL,                NULL,       NULL,           NULL                                        ),
        (6503609,   "P_NAME",       "STRING_100",       2,          "D" ,           "I",       NULL,                NULL,       NULL,           NULL                                        ),
        (6503609,   "P_GROUP_ID",   "STRING_30",        3,          "D" ,           "I",       NULL,                NULL,       NULL,           NULL                                        ),
        (6503609,   "P_VAL_TYPE",   "STRING_16",        4,          "D" ,           "I",       NULL,                4513,       "METACLASS",    "{BASE_CLASS_ID} = &apos;STRUCTURE&apos;"   ),
        (6503609,   "P_VALUES",     "FP_TUNE_VAL_ARR",  5,          "D" ,           "I",       NULL,                NULL,       NULL,           NULL                                        ),
        (6503609,   "P_NOTE",       "MEMO",             6,          "D" ,           "I",       NULL,                NULL,       NULL,           NULL                                        )
;

INSERT INTO method_control(
        id,         method_id,  qualifier,                      control,    top,  left, height, width,  tab_index,  position, validate_name,        parent_id,  class_id,           depend,     properties,         caption,                                                                tips)
VALUES  -- ::[USER].[NEW#AUTO] "Тестовая операция"
        (14682335,  311,        NULL,                           "FORM",     0,    0,    2415,   8700,   0,          0,        "Form1",              NULL,       NULL,               NULL,       NULL,               "Тест",                                                                 NULL),
        (14682336,  311,        "%PARAM%.P_NAME",               "LABEL",    120,  120,  360,    2580,   2,          1,        "Label1",             14682335,   NULL,               NULL,       NULL,               "Фамилия Имя Отчество",                                                 "Поясняющая надпись (Фамилия Имя Отчество)"),
        (14682337,  311,        "%PARAM%.P_NAME",               "TEXT",     120,  2760, 360,    5700,   3,          2,        "Text1",              14682335,   "NAME",             NULL,       "|V|",              "",                                                                     "Фамилия Имя Отчество"),
        (14682338,  311,        "%PARAM%.P_USERNAME",           "LABEL",    540,  120,  360,    2580,   4,          3,        "Label2",             14682335,   NULL,               NULL,       NULL,               "Сетевое имя",                                                          "Поясняющая надпись (Сетевое имя)"),
        (14682339,  311,        "%PARAM%.P_USERNAME",           "TEXT",     540,  2760, 360,    5700,   5,          4,        "Text2",              14682335,   "STRING_30",        NULL,       "|V|",              "",                                                                     "Сетевое имя"),
        (14682344,  311,        "OK",                           "BUTTON",   1740, 3780, 360,    2160,   6,          5,        "OK",                 14682335,   NULL,               NULL,       NULL,               "OK",                                                                   "Выполнить метод"),
        (14682345,  311,        "CANCEL",                       "BUTTON",   1740, 6060, 360,    2160,   7,          6,        "CANCEL",             14682335,   NULL,               NULL,       NULL,               "Отмена",                                                               "Отказ от выполнения"),
        -- ::[FP_TUNE].[NEW#AUTO] "Изменить"
        (17007816,  6503609,    NULL,                           "FORM",     0,    0,    7703,   12214,  0,          0,        "Form1",              NULL,       NULL,               NULL,       NULL,               "Изменить",                                                             NULL),
        (17007817,  6503609,    "%PARAM%.P_CODE",               "LABEL",    900,  60,   360,    1680,   0,          1,        "Label1",             17007816,   NULL,               NULL,       NULL,               "Код",                                                                  "Поясняющая надпись (Код)"),
        (17007818,  6503609,    "%PARAM%.P_CODE",               "TEXT",     900,  1740, 360,    3720,   2,          2,        "Text1",              17007816,   "STRING_30",        NULL,       "|V|",              "",                                                                     "Код"),
        (17007819,  6503609,    "%PARAM%.P_NAME",               "LABEL",    480,  60,   360,    1680,   0,          3,        "Label2",             17007816,   NULL,               NULL,       NULL,               "Наименование",                                                         "Поясняющая надпись (Наименование)"),
        (17007820,  6503609,    "%PARAM%.P_NAME",               "TEXT",     480,  1740, 360,    10380,  1,          4,        "Text2",              17007816,   "STRING_100",       NULL,       NULL,               "",                                                                     "Наименование"),
        (17007821,  6503609,    "OK",                           "BUTTON",   7125, 7680, 360,    2160,   25,         5,        "OK",                 17007816,   NULL,               NULL,       "|V|",              "OK",                                                                   "Выполнить метод"),
        (17007822,  6503609,    "CANCEL",                       "BUTTON",   7125, 9960, 360,    2160,   26,         6,        "CANCEL",             17007816,   NULL,               NULL,       "|V|",              "Отмена",                                                               "Отказ от выполнения"),
        (17007823,  6503609,    NULL,                           "SUBFORM",  345,  0,    7671,   12336,  0,          7,        "Form2",              17007816,   NULL,               NULL,       NULL,               "Описание",                                                             NULL),
        (17007824,  6503609,    "%PARAM%. ",                    "LINE",     555,  1080, 570,    1080,   0,          8,        "Line1",              17007823,   NULL,               NULL,       NULL,               "",                                                                     "Линия"),
        (17007825,  6503609,    "%PARAM%.P_NOTE",               "MEMO",     60,   60,   7500,   12120,  32,         9,        "Memo3",              17007823,   "MEMO",             NULL,       NULL,               "",                                                                     NULL),
        (17007826,  6503609,    "%VAR%.V_VAL_TYPE_ENABLED",     "CHECK",    7380, 2280, 240,    240,    31,         10,       "VAL_TYPE_ENABLED",   17007816,   "BOOLEAN",          NULL,       "#@#|Vb 0|",        "",                                                                     "Логика(Разрешить изменение типа зн.)"),
        (17007827,  6503609,    "%PARAM%. ",                    "FRAME",    6180, 5460, 660,    2448,   0,          11,       "frame_find_by",      17007816,   NULL,               NULL,       NULL,               "",                                                                     NULL),
        (17007828,  6503609,    "%PARAM%. ",                    "FRAME",    3480, 60,   2760,   12060,  0,          12,       "values_frame",       17007816,   NULL,               NULL,       NULL,               "",                                                                     NULL),
        (17007829,  6503609,    "%VAR%.V_VAL_ENABLED",          "CHECK",    7380, 1800, 240,    240,    30,         13,       "VALUE_ENABLED",      17007816,   "BOOLEAN",          NULL,       "#@#|Vb 0|",        "",                                                                     "Логика(Найдено общее значение)"),
        (17007830,  6503609,    "%VAR%.V_VAL",                  "LABEL",    180,  60,   360,    1260,   0,          14,       "Label3",             17007828,   NULL,               NULL,       NULL,               "Значение",                                                             "Поясняющая надпись-(Значение: строка)"),
        (17007831,  6503609,    "%VAR%.V_VAL",                  "TEXT",     180,  1320, 360,    10560,  14,         15,       "value",              17007828,   "STRING_2000",      NULL,       "|V|",              "",                                                                     "Значение: строка"),
        (17007832,  6503609,    "%VAR%.V_VAL_DATE",             "DATE",     180,  1320, 360,    1130,   15,         16,       "val_date",           17007828,   "DATE",             NULL,       "|V|",              "",                                                                     "Значение: дата"),
        (17007833,  6503609,    "%VAR%.V_VAL_REF",              "OBJECT",   180,  60,   780,    2880,   13,         17,       "val_ref",            17007828,   "REFERENCE",        NULL,       "|V|#@#|Vb 0|",     "Ссылка",                                                               "Объект:Значение: ссылка"),
        (17007834,  6503609,    "%VAR%.V_VAL_REF_TXT",          "TEXT",     420,  2940, 360,    8940,   19,         18,       "VAL_REF_TXT",        17007828,   "STRING_2000",      NULL,       "#@#|Lc 1|",        "",                                                                     "Информационное поле для ссылки"),
        (17007835,  6503609,    "%VAR%.V_VAL_BOOL.0",           "VARIANT",  240,  3120, 240,    2100,   18,         19,       "val_bool_null",      17007828,   "NUMBER",           NULL,       "3|V|",             "Не определено",                                                        "Вариант(Значение 3)"),
        (17007836,  6503609,    "%VAR%.V_VAL_BOOL.0",           "VARIANT",  240,  2220, 240,    840,    17,         20,       "val_bool_no",        17007828,   NULL,               NULL,       "2|V|",             "Нет",                                                                  "Вариант(Значение 2)"),
        (17007837,  6503609,    "%VAR%.V_VAL_BOOL.0",           "VARIANT",  240,  1380, 240,    780,    16,         21,       "val_bool_yes",       17007828,   NULL,               NULL,       "1|V|",             "Да",                                                                   "Вариант(Значение 1)"),
        (17007838,  6503609,    "%PARAM%.P_GROUP_ID",           "LABEL",    60,   60,   360,    1680,   0,          22,       "Label4",             17007816,   NULL,               NULL,       NULL,               "Группа",                                                               "Поясняющая надпись-(Идентификатор группы)"),
        (17007839,  6503609,    "%PARAM%.P_GROUP_ID",           "TEXT",     60,   1740, 360,    5400,   0,          23,       "Text7",              17007816,   "STRING_30",        NULL,       NULL,               "",                                                                     "Идентификатор группы"),
        (17007840,  6503609,    "%VAR%.V_RESET_ARR",            "CHECK",    7380, 1440, 240,    240,    29,         24,       "V_RESET_ARR",        17007816,   "BOOLEAN",          NULL,       "|D|#@#|Vb 0|",     "",                                                                     "Логика(Корректировать массив значений при выходе)"),
        (17007841,  6503609,    "%PARAM%.P_CANONS",             "ARRAY",    195,  165,  375,    1875,   22,         25,       "Array1",             17007827,   "FP_TUNE_CN_ARR",   NULL,       NULL,               "Критерии",                                                             "Массив(Критерии)"),
        (17007842,  6503609,    "%PARAM%.P_USE_CANONS",         "CHECK",    6420, 60,   240,    5328,   23,         26,       "find_by",            17007816,   "BOOLEAN",          NULL,       "|V|",              "Использовать критерии выбора значения",                                "Логика(Использовать критерии поиска)"),
        (17007843,  6503609,    "%VAR%.V_VAL_TYPE",             "FRAME",    1320, 60,   2220,   12060,  0,          27,       "val_type_frame",     17007816,   "EIGHT_CHOICE",     NULL,       "|V|",              "Тип значения",                                                         "Вариант(Вариант: тип значения)"),
        (17007844,  6503609,    "%VAR%.V_VAL_TYPE.0",           "VARIANT",  240,  120,  240,    1320,   5,          28,       "Variant3",           17007843,   "EIGHT_CHOICE",     NULL,       "1|V|",             "Строка",                                                               "Вариант(Значение 1)"),
        (17007845,  6503609,    "%PARAM%. ",                    "PANEL",    240,  1740, 1620,   10140,  0,          29,       "Panel8",             17007843,   NULL,               17007844,   NULL,               "",                                                                     "Контейнер элементов варианта(Значение 1)"),
        (17007846,  6503609,    "%VAR%.V_VAL_TYPE.0",           "VARIANT",  480,  120,  240,    1260,   6,          30,       "Variant4",           17007843,   NULL,               NULL,       "2|V|",             "Число",                                                                "Вариант(Значение 2)"),
        (17007847,  6503609,    "%PARAM%. ",                    "PANEL",    240,  1740, 1620,   10140,  0,          31,       "Panel9",             17007843,   NULL,               17007846,   NULL,               "",                                                                     "Контейнер элементов варианта(Значение 2)"),
        (17007848,  6503609,    "%VAR%.V_VAL_TYPE.0",           "VARIANT",  720,  120,  240,    1260,   7,          32,       "Variant5",           17007843,   NULL,               NULL,       "3|V|",             "Дата",                                                                 "Вариант(Значение 3)"),
        (17007849,  6503609,    "%PARAM%. ",                    "PANEL",    240,  1740, 1620,   10140,  0,          33,       "Panel10",            17007843,   NULL,               17007848,   NULL,               "",                                                                     "Контейнер элементов варианта(Значение 3)"),
        (17007850,  6503609,    "%VAR%.V_VAL_TYPE.0",           "VARIANT",  960,  120,  240,    1440,   8,          34,       "Variant6",           17007843,   NULL,               NULL,       "4|V|",             "Логика",                                                               "Вариант(Значение 4)"),
        (17007851,  6503609,    "%PARAM%. ",                    "PANEL",    240,  1740, 1620,   10140,  0,          35,       "Panel11",            17007843,   NULL,               17007850,   NULL,               "",                                                                     "Контейнер элементов варианта(Значение 4)"),
        (17007852,  6503609,    "%VAR%.V_VAL_TYPE.0",           "VARIANT",  1200, 120,  240,    1380,   9,          36,       "Variant7",           17007843,   NULL,               NULL,       "5|V|",             "Ссылка",                                                               "Вариант(Значение 5)"),
        (17007853,  6503609,    "%PARAM%. ",                    "PANEL",    240,  1740, 1620,   10140,  0,          37,       "Panel12",            17007843,   NULL,               17007852,   NULL,               "",                                                                     "Контейнер элементов варианта(Значение 5)"),
        (17007854,  6503609,    "%VAR%.V_VAL_TYPE.0",           "VARIANT",  1440, 120,  240,    1320,   10,         38,       "Variant15",          17007843,   NULL,               NULL,       "6|V|",             "Мемо",                                                                 "Вариант(Значение 6)"),
        (17007855,  6503609,    "%PARAM%. ",                    "PANEL",    240,  1740, 1620,   10140,  0,          39,       "Panel13",            17007843,   NULL,               17007854,   NULL,               "",                                                                     "Контейнер элементов варианта(Значение 6)"),
        (17007856,  6503609,    "%VAR%.V_VAL_TYPE.0",           "VARIANT",  1920, 120,  240,    1440,   12,         40,       "Variant16",          17007843,   NULL,               NULL,       "7|V|",             "Функция",                                                              "Вариант(Значение 7)"),
        (17007857,  6503609,    "%PARAM%. ",                    "PANEL",    240,  1740, 1620,   10140,  0,          41,       "Panel14",            17007843,   NULL,               17007856,   NULL,               "",                                                                     "Контейнер элементов варианта(Значение 7)"),
        (17007858,  6503609,    "%VAR%.V_VAL_TYPE.0",           "VARIANT",  1680, 120,  240,    1560,   11,         42,       "Variant9",           17007843,   NULL,               NULL,       "8|V|",             "CLOB",                                                                 "Вариант(Значение 8)"),
        (17007859,  6503609,    "%PARAM%. ",                    "PANEL",    240,  1740, 1620,   10140,  0,          43,       "Panel15",            17007843,   NULL,               17007858,   NULL,               "",                                                                     "Контейнер элементов варианта(Значение 8)"),
        (17007860,  6503609,    "%VAR%.V_MEMO",                 "MEMO",     600,  60,   1800,   11820,  20,         44,       "val_memo",           17007828,   "MEMO",             NULL,       "|V|",              "",                                                                     NULL),
        (17007861,  6503609,    "%VAR%.V_VAL_TYPE_REF",         "OBJECT",   60,   60,   360,    1860,   3,          45,       "Object1",            17007853,   "METACLASS_REF",    NULL,       "|V|",              "Тип значения",                                                         "Объект:Тип значения"),
        (17007862,  6503609,    "%VAR%.V_VAL_TYPE_REF.NAME",    "TEXT",     60,   1920, 360,    8100,   4,          46,       "val_type_name",      17007853,   "STRING",           17007861,   "|V|",              "",                                                                     "Class Name"),
        (17007863,  6503609,    "%PARAM%. ",                    "LABEL",    2400, 60,   300,    7860,   0,          47,       "memo_note",          17007828,   NULL,               NULL,       "#@#|Be 0|",        "При получении значения символ с кодом 13 удаляется из мемо-поля",      NULL),
        (17007864,  6503609,    "%PARAM%.P_VAL_TYPE",           "TEXT",     7260, 2580, 360,    1860,   27,         48,       "val_type",           17007816,   "STRING_16",        NULL,       "#@#|Vb 0|",        "",                                                                     "Тип значения"),
        (17007865,  6503609,    "%VAR%.V_VAL_REF_TYPE",         "TEXT",     7260, 4440, 360,    1980,   28,         49,       "val_ref_type",       17007816,   "STRING_100",       NULL,       "|V|#@#|Vb 0|",     "",                                                                     "Тип ссылки на значение"),
        (17007866,  6503609,    "%PARAM%.P_CACHED",             "CHECK",    6648, 60,   495,    5100,   24,         50,       "Check5",             17007816,   "BOOLEAN",          NULL,       "|V|",              "Кэшировать получение значений",                                        "Логика(Кэшировать получение значений)"),
        (17007867,  6503609,    "%PARAM%. ",                    "LABEL",    7005, 60,   315,    7080,   0,          51,       "Label6",             17007816,   NULL,               NULL,       "#@#|Be 0|FS 8|",   "Кэширование рекомендуется использовать для редко изменяемых настроек", "Надпись"),
        (17007868,  6503609,    "Кнопка1",                      "BUTTON",   600,  60,   360,    3600,   21,         52,       "val_clob_btn",       17007828,   NULL,               NULL,       "|V|",              "Просмотр / Изменение",                                                 "Кнопка1")
;

INSERT INTO view(id, class_id, name, short_name, properties, is_default, to_printer, to_file, source_id, extension_id)
VALUES  -- ::[USER].[VW_CRIT_USER] "Полный список"
        (4384,          0, "Полный список",                 "VW_CRIT_USER",         "|AllMethods Y|HasClass|NotObjects|PlPlus|USERCONTEXT 0|",                      1, 1, 1, NULL, NULL),
        -- ::[CL_PRIV].[VW_CRIT_CL_PRIV] "Полный список"
        (3616,          1, "Полный список",                 "VW_CRIT_CL_PRIV",      "|AllMethods Y|HasClass|NotObjects|PlPlus|ShowChild|USERCONTEXT 1|",            1, 1, 1, NULL, 4172642368),
        (4172642368,    1, "Полный список (расширение)",    "VW_CRIT_CL_PRIV_EXT",  "|AllMethods Y|HasClass|NotObjects|PlPlus|ShowChild|USERCONTEXT 1|",            0, 1, 1, 3616, NULL),
        -- ::[CL_ORG].[VW_CRIT_CL_PRIV] "Полный список"
        (4522,          2, "Полный список",                 "VW_CRIT_CL_ORG",       "|AllMethods Y|GENERATE_ANSI_JOINS|HasClass|NotObjects|PlPlus|USERCONTEXT 0|",  1, 1, 1, NULL, NULL),
        -- ::[FP_TUNE].[VW_CRIT_FP_TUNE_ALL] "Полный список"
        (8057380,       6, "Полный список",                 "VW_CRIT_FP_TUNE_ALL",  "|AllMethods Y|HasClass|NotObjects|PlPlus|USERCONTEXT 0|",                      1, 1, 1, NULL, NULL)
;

INSERT INTO column(
        view_id,        name,                               width,  position,   qual,               alias,          base,           is_sizeable,    is_invisible,   logging,    ability_perform_operation,  target_class_id,        reference_type,     reference_id        )
VALUES  -- ::[USER].[VW_CRIT_USER] "Полный список"
        (4384,          "Фамилия Имя Отчество",             21,     1,          "NAME",             "C_FIO",        "STRING",       1,              0,              "0",        1,                          NULL,                   NULL,               NULL                ),
        (4384,          "Сетевое имя",                      9,      2,          "USERNAME",         "C_USERNAME",   "STRING",       1,              0,              "0",        1,                          NULL,                   NULL,               NULL                ),
        (4384,          "Физическое лицо",                  21,     3,          "CL_PRIV_REF.NAME", "C_NAME_1",     "STRING",       1,              0,              "0",        1,                          "CL_PRIV",              0,                  "a1.C_CL_PRIV_REF"  ),
        (4384,          "id",                               4,      4,          "ID",               "C_ID",         "NUMBER",       1,              2,              "0",        1,                          NULL,                   NULL,               NULL                ),
        -- ::[CL_PRIV].[VW_CRIT_CL_PRIV] "Полный список"
        (3616,          "ID",                               17,     1,          "ID",               "C_ID",         "NUMBER",       1,              2,              "D",        1,                          NULL,                   NULL,               NULL                ),
        (3616,          "Ф.И.О.",                           25,     2,          "NAME",             "C_NAME",       "STRING",       1,              0,              "D",        1,                          NULL,                   NULL,               NULL                ),
        -- ::[CL_PRIV].[VW_CRIT_CL_PRIV_EXT] "Полный список (расширение)"
        (4172642368,    "ID",                               17,     1,          "ID",               "C_ID",         "NUMBER",       1,              2,              "0",        1,                          NULL,                   NULL,               NULL                ),
        (4172642368,    "Ф.И.О.",                           25,     2,          "NAME",             "C_NAME",       "STRING",       1,              0,              "0",        1,                          NULL,                   NULL,               NULL                ),
        (4172642368,    "Расширение",                       25,     3,          "EXT",              "C_EXT",        "STRING",       1,              0,              "0",        1,                          NULL,                   NULL,               NULL                ),
        -- ::[CL_ORG].[VW_CRIT_CL_ORG] "Полный список"
        (4522,          "ID",                               17,     1,          "ID",               "C_ID",         "NUMBER",       1,              2,              "D",        1,                          NULL,                   NULL,               NULL                ),
        (4522,          "Наименование",                     25,     2,          "NAME",             "C_NAME",       "STRING",       1,              0,              "D",        1,                          NULL,                   NULL,               NULL                ),
        -- ::[FP_TUNE].[VW_CRIT_FP_TUNE_ALL] "Полный список"
        (8057380,       "Группа",                           8,      1,          "GROUP_ID",         "GROUP_ID",     "STRING",       1,              0,              "0",        1,                          NULL,                   NULL,               NULL                ),
        (8057380,       "Код",                              9,      2,          "CODE",             "CODE",         "STRING",       1,              0,              "0",        1,                          NULL,                   NULL,               NULL                ),
        (8057380,       "Наименование",                     19,     3,          "NAME",             "NAME",         "STRING",       1,              0,              "0",        1,                          NULL,                   NULL,               NULL                ),
        (8057380,       "Тип значения",                     12,     4,          "VAL_TYPE",         "VAL_TYPE",     "STRING",       1,              0,              "0",        0,                          NULL,                   NULL,               NULL                ),
        (8057380,       "Значения настройки",               8,      5,          "VALUES",           "VALUES",       "COLLECTION",   1,              0,              "0",        1,                          "FP_TUNE_VAL",          1,                  NULL                ),
        (8057380,       "Общее значение",                   25,     6,          "VAL_TYPE",         "VALUE_COMMON", "STRING",       1,              0,              "0",        0,                          NULL,                   NULL,               NULL                ),
        (8057380,       "Использовать критерии выбора",     11,     7,          "USE_CANONS",       "USE_CANONS",   "BOOLEAN",      1,              0,              "0",        1,                          NULL,                   NULL,               NULL                ),
        (8057380,       "Критерии выбора",                  8,      8,          "CANONS",           "CANONS",       "COLLECTION",   1,              0,              "0",        1,                          "FP_TUNE_CN",           1,                  NULL                ),
        (8057380,       "Критерии выбора",                  8,      9,          "CANONS",           "CANONS_STR",   "STRING",       1,              0,              "0",        0,                          NULL,                   NULL,               NULL                ),
        (8057380,       "Описание",                         30,     10,         "NOTE",             "NOTE",         "MEMO",         1,              0,              "0",        1,                          NULL,                   NULL,               NULL                ),
        (8057380,       "Кэшировать получение значений",    8,      11,         "CACHED",           "CACHED",       "BOOLEAN",      1,              0,              "0",        1,                          NULL,                   NULL,               NULL                )
;

INSERT INTO row_item(view_id, object_id, name, value)
VALUES  -- ::[USER].[VW_CRIT_USER] "Полный список"
        (4384, 22738256, "ID", "0"), (4384, 22738256, "C_1", "Тест Тест Тестович"), (4384, 22738256, "C_2", "TEST"), (4384, 22738256, "REF3", "1"),
        -- ::[CL_PRIV].[VW_CRIT_CL_PRIV] "Полный список"
        (3616, 1, "ID", "1"), (3616, 1, "C_1", "Тест Тест Тестович"),
        -- ::[CL_PRIV].[VW_CRIT_CL_PRIV_EXT] "Полный список (расширение)"
        (4172642368, 1, "ID", "1"), (4172642368, 1, "C_1", "Тест Тест Тестович"), (4172642368, 1, "C_2", "Расширение"),
        -- ::[CL_ORG].[VW_CRIT_CL_ORG] "Полный список"
        (4522, 2, "ID", "1"), (4522, 2, "C_1", 'ООО "Тест"'),
        -- ::[FP_TUNE].[VW_CRIT_FP_TUNE_ALL] "Полный список"
        (8057380, 1, "ID",       "9611172"),
        (8057380, 1, "CLASS_ID", "FP_TUNE"),
        (8057380, 1, "C_1",      "ПЛАНИРОВАНИЕ"),
        (8057380, 1, "C_2",      "SHIFT_PAY_DAY"),
        (8057380, 1, "C_3",      "Сдвиг начала операций гашения в случае отсутствия нужного дня месяца на 1 день вперед"),
        (8057380, 1, "C_4",      "Логика"),
        (8057380, 1, "C_5",      "{***}"),
        (8057380, 1, "REF5",     "9611173"),
        (8057380, 1, "C_6",      "Нет"),
        (8057380, 1, "C_7",      "1"),
        (8057380, 1, "C_8",      "{***}"),
        (8057380, 1, "REF8",     "9611174"),
        (8057380, 1, "C_9",      "Виды операций по договору, MetaClass, Виды кредитов"),
        (8057380, 1, "C_10",     "При включенной настройке при расчёте плановой даты операции происходит сдвиг на 1-ое число следующего месяца, если указанный день отсутствует в текущем месяце (например, день = 31, а месяц = 'февраль'). При выключенной настройке дата устанавливается на последний день текущего месяца. Используется в продуктах: Взаимодействие кредитов с фронт-офисом, Гарантии, Заявки на получение кредита/гарантии, Кредиты, Привлечение/размещение, Факторинг"),
        (8057380, 1, "C_11",     "1")
;
