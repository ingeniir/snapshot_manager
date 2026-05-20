# Snapshot Manager (`snapshot_manager`)

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Un moteur de sauvegarde automatique (*autosave*) et de gestion d'historique de révisions développé en Rust. Ce module a été conçu pour être intégré comme backend haute performance au sein d'une application de notebook scientifique (similaire à Jupyter) développée avec le framework **Tauri + React**.

## 📌 Contexte du Projet

En tant qu'étudiant en **L2 MIASHS** (Mathématiques et Informatique Appliquées aux Sciences Humaines et Sociales) et passionné de Data Science, je développe actuellement une application Desktop de prise de notes scientifiques et d'analyse de données. 

Pour garantir la sécurité des blocs de code (Python) et des cellules Markdown saisis par l'analyste, j'ai développé ce cœur de système en **Rust**. L'objectif est d'isoler la logique métier lourde (manipulation du système de fichiers, sérialisation et gestion de la mémoire) dans une bibliothèque isolée, performante et robuste.

## 🛠️ Fonctionnalités

- **Modélisation typée d'un Notebook :** Gestion de structures de données imbriquées représentant le notebook, ses métadonnées (kernels) et ses cellules de manière polymorphe (Code vs Markdown).
- **Sérialisation JSON ultra-rapide :** Utilisation de `Serde` pour encoder et décoder l'état de l'application afin de faciliter la communication asynchrone avec le frontend React.
- **Gestion des révisions chronologiques :** Tri et indexation automatique des sauvegardes (`snapshots`) de la plus récente à la plus ancienne à l'aide de l'API temporelle de `Chrono`.
- **Tolérance aux pannes (I/O) :** Gestion stricte des erreurs et des permissions système à l'aide des types d'exécution natifs de Rust (`Result` / `Option`), empêchant tout crash de l'application principale.

## 🏗️ Architecture et Notions Rust Apprises

Ce projet m'a permis de consolider des concepts avancés de programmation système avec Rust :
- **Ownership & Borrowing :** Gestion fine de la mémoire sur le tas (*Heap*) via l'utilisation de structures et de pointeurs intelligents sans avoir recours à un Garbage Collector.
- **Polymorphisme statique et dynamique :** Implémentation de *Traits* pour définir des contrats comportementaux clairs entre mes objets.
- **Gestion idiomatique des erreurs :** Remplacement des exceptions traditionnelles (types Python) par l'utilisation intensive du pattern matching et de l'opérateur `?`.

## 📦 Installation et Utilisation

### Prérequis
- [Rust & Cargo](https://www.rust-lang.org/tools/install) (version stable)

### Intégration locale
Pour l'utiliser dans un projet exécutable ou un projet Tauri existant, ajoutez la dépendance locale dans votre fichier `Cargo.toml` :

```toml
[dependencies]
snapshot_manager = { path = "chemin/vers/snapshot_manager" }