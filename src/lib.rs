use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Snapshot {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub contenu: String,
}

impl Snapshot {
    pub fn new(id: String, contenu: String) -> Self {
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

struct HistoryManager {
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
    use super::*; // Permet d'importer ta structure Snapshot dans le module de test

    #[test]
    fn test_serialization_json() {
        // 1. On crée un snapshot de test
        let snapshot_original =
            Snapshot::new("version_1".to_string(), "print('Hello World')".to_string());

        // 2. On teste la conversion en JSON
        let json_genere = snapshot_original
            .to_json()
            .expect("La sérialisation a échoué");
        println!("JSON généré : {}", json_genere); // S'affichera si le test échoue ou avec 'cargo test -- --nocapture'

        // 3. On teste la reconversion du JSON vers une structure Rust
        let snapshot_recupere =
            Snapshot::from_json(&json_genere).expect("La désérialisation a échoué");

        // 4. On vérifie que les données n'ont pas changé en cours de route
        assert_eq!(snapshot_original.id, snapshot_recupere.id);
        assert_eq!(snapshot_original.contenu, snapshot_recupere.contenu);
    }
}
