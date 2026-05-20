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
