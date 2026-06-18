CREATE TABLE sessions (
  id TEXT PRIMARY KEY,
  username TEXT NOT NULL,
  password_hash TEXT NOT NULL,
  debug_pipe_id TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  initial_at TEXT,
  expires_at TEXT
);

CREATE TABLE settings (
  name TEXT PRIMARY KEY,
  value TEXT
);

INSERT INTO settings VALUES ("test1", "test1");
INSERT INTO settings VALUES ("test2", "test2");
