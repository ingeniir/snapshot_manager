use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ---- Snapshot Struct ----

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
pub struct NotebookContent {
    pub metadata: String,
    pub cells: Vec<Cell>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Snapshot {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub contenu: NotebookContent,
}

impl Snapshot {
    pub fn new(id: String, contenu: NotebookContent) -> Self {
        Snapshot {
            id,
            timestamp: Utc::now(),
            contenu,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

// ---- History Manager ----

pub struct HistoryManager {
    dossier_sauvegarde: std::path::PathBuf,
    
}

impl HistoryManager {
    pub fn new(path: String) -> Self {
        HistoryManager {
            dossier_sauvegarde: std::path::PathBuf::from(path),
        }
    }

    pub fn create_folder(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.dossier_sauvegarde)
    }

    pub fn save_snapshot(&self, snapshot: &Snapshot) -> Result<(), String> {
        let filename = format!("snapshot_{}.json", snapshot.id);
        let filepath = self.dossier_sauvegarde.join(filename);
        let json = snapshot.to_json().map_err(|e| e.to_string())?;
        std::fs::write(filepath, json).map_err(|e| e.to_string())
    }

    pub fn load_all_snapshots(&self) -> Result<Vec<Snapshot>, String> {
        let mut snapshots = Vec::new();
        let entries = std::fs::read_dir(&self.dossier_sauvegarde).map_err(|e| e.to_string())?;
        
        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if path.is_file() {
                let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
                let snapshot = Snapshot::from_json(&content).map_err(|e| e.to_string())?;
                snapshots.push(snapshot);
            }
        }
        snapshots.sort_by(|a, b| b.timestamp.cmp(&a.timestamp)); // Tri par ordre décroissant de timestamp
        Ok(snapshots)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_gestionnaire_historique_complet() {
        let chemin_test = "./.test_notebook_history".to_string();
        let manager = HistoryManager::new(chemin_test.clone());
        
        manager.create_folder().expect("Impossible de créer le dossier de test");

        let cell1 = Cell {
            id: "cell1".to_string(),
            cell_type: CellType::Code,
            source: "print('Première version')".to_string(),
        };
        let contenu1 = NotebookContent {
            metadata: "metadata1".to_string(),
            cells: vec![cell1],
        };
        let snap1 = Snapshot::new("v1".to_string(), contenu1);
        manager.save_snapshot(&snap1).expect("Échec sauvegarde v1");

        sleep(Duration::from_secs(1));

        let cell2 = Cell {
            id: "cell2".to_string(),
            cell_type: CellType::Code,
            source: "print('Deuxième version complétée')".to_string(),
        };
        let contenu2 = NotebookContent {
            metadata: "metadata2".to_string(),
            cells: vec![cell2],
        };
        let snap2 = Snapshot::new("v2".to_string(), contenu2);
        manager.save_snapshot(&snap2).expect("Échec sauvegarde v2");

        let liste_snapshots = manager.load_all_snapshots().expect("Échec du chargement");

        assert_eq!(liste_snapshots.len(), 2);
        assert_eq!(liste_snapshots[0].id, "v2");
        assert_eq!(liste_snapshots[1].id, "v1");

        println!("Bravo ! Le premier snapshot lu est bien le plus récent : {:?}", liste_snapshots[0].contenu);

        let _ = std::fs::remove_dir_all(chemin_test);
    }
}
