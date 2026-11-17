use std::path::PathBuf;

/// Optional `COMAIL_PROFILE` (ASCII letters, digits, `-`, `_`): runs a
/// separate install side by side, e.g. a dev build next to the released app.
/// It namespaces the data dir and the keyring service as `comail-<profile>`.
/// Unset or invalid means the default `comail`.
pub fn namespace() -> String {
    match std::env::var("COMAIL_PROFILE") {
        Ok(p)
            if !p.is_empty()
                && p.len() <= 32
                && p.chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') =>
        {
            format!("comail-{p}")
        }
        _ => "comail".to_string(),
    }
}

#[derive(Debug, Clone)]
pub struct Paths {
    pub data_dir: PathBuf,
}

impl Paths {
    pub fn default_dirs() -> Self {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(namespace());
        Paths { data_dir }
    }

    pub fn for_tests(root: &std::path::Path) -> Self {
        Paths {
            data_dir: root.to_path_buf(),
        }
    }

    pub fn db_file(&self) -> PathBuf {
        self.data_dir.join("comail.db")
    }

    /// Directory for raw .eml files of one account.
    pub fn mail_dir(&self, account_id: i64) -> PathBuf {
        self.data_dir.join("mail").join(account_id.to_string())
    }

    /// Directory for extracted attachment files.
    pub fn attachments_dir(&self, account_id: i64) -> PathBuf {
        self.data_dir
            .join("attachments")
            .join(account_id.to_string())
    }

    /// Root for app-managed copies of outgoing (draft) attachments. Files
    /// picked in the composer are staged here so dispatch only ever reads
    /// files the app itself wrote, never an arbitrary frontend-supplied path.
    pub fn draft_attachments_dir(&self) -> PathBuf {
        self.data_dir.join("draft_attachments")
    }

    /// Directory holding local embedding-model weights (`<models_dir>/<key>`).
    pub fn models_dir(&self) -> PathBuf {
        self.data_dir.join("models")
    }

    pub fn ensure(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.data_dir)
    }
}
