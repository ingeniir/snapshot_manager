use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CellType {
    Code,
    Markdown,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cell {
    pub id: String,
    pub cell_type: CellType,
    pub source: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotebookMetadata {
    pub name: String,
    pub kernel: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotebookContent {
    pub metadata: NotebookMetadata,
    pub cells: Vec<Cell>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Snapshot {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub content: NotebookContent,
}

impl Snapshot {
    pub fn new(id: String, content: NotebookContent) -> Self {
        Snapshot {
            id,
            timestamp: Utc::now(),
            content,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn filename(&self) -> String {
        format!("snapshot_{}_{}.json", self.timestamp.timestamp_millis(), self.id)
    }
}

#[derive(Debug, Clone)]
pub struct SnapshotMeta {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub notebook_name: String,
    pub filename: String,
}

pub struct HistoryManager {
    save_folder: std::path::PathBuf,
}

impl HistoryManager {
    pub fn new(path: String) -> Self {
        HistoryManager {
            save_folder: std::path::PathBuf::from(path),
        }
    }

    pub fn create_folder(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.save_folder)
    }

    pub fn save_snapshot(&self, snapshot: &Snapshot, max_keep: usize) -> Result<(), String> {
        let filepath = self.save_folder.join(snapshot.filename());
        let json = snapshot.to_json().map_err(|e| e.to_string())?;
        std::fs::write(&filepath, json).map_err(|e| e.to_string())?;

        let mut snapshot_files: Vec<std::path::PathBuf> = std::fs::read_dir(&self.save_folder)
            .map_err(|e| e.to_string())?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| {
                path.is_file()
                    && path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|n| n.starts_with("snapshot_"))
                        .unwrap_or(false)
            })
            .collect();

        snapshot_files.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

        if snapshot_files.len() > max_keep {
            for old_file in &snapshot_files[max_keep..] {
                std::fs::remove_file(old_file).map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }

    pub fn load_snapshot(&self, id: &str) -> Result<Snapshot, String> {
        let entries = std::fs::read_dir(&self.save_folder).map_err(|e| e.to_string())?;

        for entry in entries {
            let path = entry.map_err(|e| e.to_string())?.path();
            let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            let expected_suffix = format!("_{}.json", id);
            if filename.ends_with(&expected_suffix) {
                let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
                return Snapshot::from_json(&content).map_err(|e| e.to_string());
            }
        }

        Err(format!("Snapshot '{}' introuvable", id))
    }

    pub fn list_snapshots(&self) -> Result<Vec<SnapshotMeta>, String> {
        let mut metas = Vec::new();
        let entries = std::fs::read_dir(&self.save_folder).map_err(|e| e.to_string())?;

        for entry in entries {
            let path = entry.map_err(|e| e.to_string())?.path();
            if !path.is_file() {
                continue;
            }

            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            if !filename.starts_with("snapshot_") {
                continue;
            }

            let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
            let snapshot = Snapshot::from_json(&content).map_err(|e| e.to_string())?;

            metas.push(SnapshotMeta {
                id: snapshot.id.clone(),
                timestamp: snapshot.timestamp,
                notebook_name: snapshot.content.metadata.name.clone(),
                filename,
            });
        }

        metas.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(metas)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    fn make_snapshot(id: &str, name: &str, source: &str) -> Snapshot {
        let metadata = NotebookMetadata {
            name: name.to_string(),
            kernel: "python3".to_string(),
            created_at: Utc::now(),
        };
        let cell = Cell {
            id: "cell1".to_string(),
            cell_type: CellType::Code,
            source: source.to_string(),
        };
        let content = NotebookContent {
            metadata,
            cells: vec![cell],
        };
        Snapshot::new(id.to_string(), content)
    }

    #[test]
    fn test_save_and_load_snapshot() {
        let path = "./.test_save_load".to_string();
        let manager = HistoryManager::new(path.clone());
        manager.create_folder().unwrap();

        let snap = make_snapshot("v1", "notebook_test.db", "print('hello')");
        manager.save_snapshot(&snap, 10).unwrap();

        let loaded = manager.load_snapshot("v1").unwrap();
        assert_eq!(loaded.id, "v1");
        assert_eq!(loaded.content.metadata.name, "notebook_test.db");

        let _ = std::fs::remove_dir_all(path);
    }

    #[test]
    fn test_retention_policy() {
        let path = "./.test_retention".to_string();
        let manager = HistoryManager::new(path.clone());
        manager.create_folder().unwrap();

        for i in 1..=5 {
            sleep(Duration::from_millis(10));
            let snap = make_snapshot(&format!("v{}", i), "notebook.db", "code");
            manager.save_snapshot(&snap, 3).unwrap();
        }

        let metas = manager.list_snapshots().unwrap();
        assert_eq!(metas.len(), 3);
        assert_eq!(metas[0].id, "v5");

        let _ = std::fs::remove_dir_all(path);
    }

    #[test]
    fn test_list_snapshots_order() {
        let path = "./.test_list_order".to_string();
        let manager = HistoryManager::new(path.clone());
        manager.create_folder().unwrap();

        let snap1 = make_snapshot("v1", "nb.db", "code v1");
        manager.save_snapshot(&snap1, 10).unwrap();
        sleep(Duration::from_millis(10));
        let snap2 = make_snapshot("v2", "nb.db", "code v2");
        manager.save_snapshot(&snap2, 10).unwrap();

        let metas = manager.list_snapshots().unwrap();
        assert_eq!(metas[0].id, "v2");
        assert_eq!(metas[1].id, "v1");

        let _ = std::fs::remove_dir_all(path);
    }
}