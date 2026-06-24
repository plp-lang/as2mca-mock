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
INSERT INTO settings VALUES ("test1", "test1");
INSERT INTO settings VALUES ("test2", "test2");

CREATE TABLE IF NOT EXISTS view (
    id INTEGER PRIMARY KEY,

    class_id TEXT NOT NULL,

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
    order_by TEXT
);
INSERT INTO view(id, class_id, name, short_name, properties, is_default, to_printer, to_file) VALUES (1, "USER", "Полный список", "VW_CRIT_USER", "|AllMethods Y|HasClass|NotObjects|PlPlus|USERCONTEXT 0|", 1, 1, 1);
INSERT INTO view(id, class_id, name, short_name, properties, is_default, to_printer, to_file) VALUES (2, "USER", "Полный список 2", "VW_CRIT_USER_2", "|AllMethods Y|HasClass|NotObjects|PlPlus|USERCONTEXT 0|", 0, 1, 1);
