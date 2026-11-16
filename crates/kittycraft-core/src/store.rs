use crate::error::KittyCraftError;
use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiaryEntry {
    pub id: i64,
    pub date: String,
    pub category: String,
    pub title: String,
    pub notes: String,
    pub weight_kg: Option<f64>,
    pub mood: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitterEvent {
    pub id: i64,
    pub start_date: String,
    pub end_date: String,
    pub sitter_name: String,
    pub sitter_relation: String,
    pub care_notes: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VetVisit {
    pub id: i64,
    pub date: String,
    pub clinic: String,
    pub reason: String,
    pub diagnosis: String,
    pub cost_eur: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoRecord {
    pub id: i64,
    pub cat_name: String,
    pub file_path: String,
    pub preset: String,
    pub caption: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct StoreStatus {
    pub db_path: String,
    pub diary_count: i64,
    pub calendar_count: i64,
    pub vet_visit_count: i64,
    pub photo_count: i64,
}

pub struct KittyStore {
    conn: Mutex<Connection>,
    path: String,
}

pub fn data_dir() -> PathBuf {
    let base = std::env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    base.join("kittycraft")
}

impl KittyStore {
    pub fn open_default() -> Result<Self, KittyCraftError> {
        let dir = data_dir();
        std::fs::create_dir_all(&dir)?;
        let db_path = dir.join("kittycraft.db");
        Self::open_path(&db_path)
    }

    pub fn open_path(path: &Path) -> Result<Self, KittyCraftError> {
        let conn = Connection::open(path)?;
        let store = Self {
            conn: Mutex::new(conn),
            path: path.display().to_string(),
        };
        store.init_schema()?;
        Ok(store)
    }

    pub fn open_in_memory() -> Result<Self, KittyCraftError> {
        let conn = Connection::open_in_memory()?;
        let store = Self {
            conn: Mutex::new(conn),
            path: ":memory:".into(),
        };
        store.init_schema()?;
        Ok(store)
    }

    fn init_schema(&self) -> Result<(), KittyCraftError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| KittyCraftError::InvalidInput("DB mutex poisoned".into()))?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS diary (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date TEXT NOT NULL,
                category TEXT NOT NULL,
                title TEXT NOT NULL,
                notes TEXT NOT NULL,
                weight_kg REAL,
                mood TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS sitter_calendar (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                start_date TEXT NOT NULL,
                end_date TEXT NOT NULL,
                sitter_name TEXT NOT NULL,
                sitter_relation TEXT NOT NULL,
                care_notes TEXT NOT NULL,
                status TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS vet_visits (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date TEXT NOT NULL,
                clinic TEXT NOT NULL,
                reason TEXT NOT NULL,
                diagnosis TEXT NOT NULL,
                cost_eur REAL
            );
            CREATE TABLE IF NOT EXISTS photos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                cat_name TEXT NOT NULL,
                file_path TEXT NOT NULL,
                preset TEXT NOT NULL,
                caption TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                val TEXT NOT NULL
            );",
        )?;
        Ok(())
    }

    pub fn status(&self) -> Result<StoreStatus, KittyCraftError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| KittyCraftError::InvalidInput("DB mutex poisoned".into()))?;
        let diary_count: i64 = conn.query_row("SELECT COUNT(*) FROM diary", [], |r| r.get(0))?;
        let calendar_count: i64 =
            conn.query_row("SELECT COUNT(*) FROM sitter_calendar", [], |r| r.get(0))?;
        let vet_visit_count: i64 =
            conn.query_row("SELECT COUNT(*) FROM vet_visits", [], |r| r.get(0))?;
        let photo_count: i64 = conn.query_row("SELECT COUNT(*) FROM photos", [], |r| r.get(0))?;
        Ok(StoreStatus {
            db_path: self.path.clone(),
            diary_count,
            calendar_count,
            vet_visit_count,
            photo_count,
        })
    }

    pub fn add_diary(
        &self,
        date: &str,
        category: &str,
        title: &str,
        notes: &str,
        weight_kg: Option<f64>,
        mood: &str,
    ) -> Result<i64, KittyCraftError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| KittyCraftError::InvalidInput("DB mutex poisoned".into()))?;
        conn.execute(
            "INSERT INTO diary (date, category, title, notes, weight_kg, mood) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![date, category, title, notes, weight_kg, mood],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn list_diary(&self, limit: usize) -> Result<Vec<DiaryEntry>, KittyCraftError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| KittyCraftError::InvalidInput("DB mutex poisoned".into()))?;
        let mut stmt = conn.prepare(
            "SELECT id, date, category, title, notes, weight_kg, mood FROM diary ORDER BY date DESC, id DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit as i64], |r| {
            Ok(DiaryEntry {
                id: r.get(0)?,
                date: r.get(1)?,
                category: r.get(2)?,
                title: r.get(3)?,
                notes: r.get(4)?,
                weight_kg: r.get(5)?,
                mood: r.get(6)?,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn add_sitter_event(
        &self,
        start_date: &str,
        end_date: &str,
        sitter_name: &str,
        sitter_relation: &str,
        care_notes: &str,
        status: &str,
    ) -> Result<i64, KittyCraftError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| KittyCraftError::InvalidInput("DB mutex poisoned".into()))?;
        conn.execute(
            "INSERT INTO sitter_calendar (start_date, end_date, sitter_name, sitter_relation, care_notes, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![start_date, end_date, sitter_name, sitter_relation, care_notes, status],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn list_sitter_events(&self) -> Result<Vec<SitterEvent>, KittyCraftError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| KittyCraftError::InvalidInput("DB mutex poisoned".into()))?;
        let mut stmt = conn.prepare(
            "SELECT id, start_date, end_date, sitter_name, sitter_relation, care_notes, status FROM sitter_calendar ORDER BY start_date ASC, id DESC",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(SitterEvent {
                id: r.get(0)?,
                start_date: r.get(1)?,
                end_date: r.get(2)?,
                sitter_name: r.get(3)?,
                sitter_relation: r.get(4)?,
                care_notes: r.get(5)?,
                status: r.get(6)?,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn add_vet_visit(
        &self,
        date: &str,
        clinic: &str,
        reason: &str,
        diagnosis: &str,
        cost_eur: Option<f64>,
    ) -> Result<i64, KittyCraftError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| KittyCraftError::InvalidInput("DB mutex poisoned".into()))?;
        conn.execute(
            "INSERT INTO vet_visits (date, clinic, reason, diagnosis, cost_eur) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![date, clinic, reason, diagnosis, cost_eur],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn list_vet_visits(&self) -> Result<Vec<VetVisit>, KittyCraftError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| KittyCraftError::InvalidInput("DB mutex poisoned".into()))?;
        let mut stmt = conn.prepare(
            "SELECT id, date, clinic, reason, diagnosis, cost_eur FROM vet_visits ORDER BY date DESC, id DESC",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(VetVisit {
                id: r.get(0)?,
                date: r.get(1)?,
                clinic: r.get(2)?,
                reason: r.get(3)?,
                diagnosis: r.get(4)?,
                cost_eur: r.get(5)?,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn add_photo(
        &self,
        cat_name: &str,
        file_path: &str,
        preset: &str,
        caption: &str,
    ) -> Result<i64, KittyCraftError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| KittyCraftError::InvalidInput("DB mutex poisoned".into()))?;
        let now = Utc::now().format("%Y-%m-%d").to_string();
        conn.execute(
            "INSERT INTO photos (cat_name, file_path, preset, caption, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![cat_name, file_path, preset, caption, now],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn list_photos(&self) -> Result<Vec<PhotoRecord>, KittyCraftError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| KittyCraftError::InvalidInput("DB mutex poisoned".into()))?;
        let mut stmt = conn.prepare(
            "SELECT id, cat_name, file_path, preset, caption, created_at FROM photos ORDER BY id DESC LIMIT 50",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(PhotoRecord {
                id: r.get(0)?,
                cat_name: r.get(1)?,
                file_path: r.get(2)?,
                preset: r.get(3)?,
                caption: r.get(4)?,
                created_at: r.get(5)?,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>, KittyCraftError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| KittyCraftError::InvalidInput("DB mutex poisoned".into()))?;
        let mut stmt = conn.prepare("SELECT val FROM settings WHERE key = ?1")?;
        let mut rows = stmt.query(params![key])?;
        if let Some(r) = rows.next()? {
            Ok(Some(r.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn set_setting(&self, key: &str, val: &str) -> Result<(), KittyCraftError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| KittyCraftError::InvalidInput("DB mutex poisoned".into()))?;
        conn.execute(
            "INSERT INTO settings (key, val) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET val = excluded.val",
            params![key, val],
        )?;
        Ok(())
    }
}
