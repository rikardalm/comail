-- connect_microsoft_calendar stores kind = 'microsoft', but 009's CHECK only
-- allowed 'google'/'generic', so connecting an Outlook calendar always failed
-- with a constraint error ("Database error"). Rebuild the table to allow it.
CREATE TABLE caldav_config_new (
  account_id    INTEGER PRIMARY KEY REFERENCES accounts(id) ON DELETE CASCADE,
  kind          TEXT NOT NULL CHECK (kind IN ('google','generic','microsoft')),
  base_url      TEXT NOT NULL,
  username      TEXT,
  principal_url TEXT,
  home_set_url  TEXT,
  enabled       INTEGER NOT NULL DEFAULT 1,
  last_error    TEXT
);
INSERT INTO caldav_config_new
  (account_id, kind, base_url, username, principal_url, home_set_url, enabled, last_error)
SELECT account_id, kind, base_url, username, principal_url, home_set_url, enabled, last_error
FROM caldav_config;
DROP TABLE caldav_config;
ALTER TABLE caldav_config_new RENAME TO caldav_config;
