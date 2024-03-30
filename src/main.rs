use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};
use std::collections::HashMap;
use std::env;
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::error::Error;
use bincode;

// Adjust the main function's return type to use a more generic error type
fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("embeddings.db")?;

    // Create the table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS file_embeddings (
            id INTEGER PRIMARY KEY,
            file_hash TEXT NOT NULL UNIQUE,
            file_path TEXT NOT NULL,
            embedding BLOB NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS similarities (
            id INTEGER PRIMARY KEY,
            file_id1 INTEGER NOT NULL,
            file_id2 INTEGER NOT NULL,
            similarity REAL NOT NULL,
            FOREIGN KEY (file_id1) REFERENCES file_embeddings(id),
            FOREIGN KEY (file_id2) REFERENCES file_embeddings(id),
            UNIQUE(file_id1, file_id2)
        )",
        [],
    )?;

    // Example file path

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: program <file_path>".into());
    }
    let file_path = &args[1];


    //let file_path = "example.txt";
    let (file_hash, embedding) = process_file(file_path)?;


    // Check if an entry with the same file path exists
    let mut stmt = conn.prepare("SELECT id, file_hash FROM file_embeddings WHERE file_path = ?1")?;
    let mut rows = stmt.query(params![file_path])?;

    if let Some(row) = rows.next()? {
        let id: i32 = row.get(0)?;
        let existing_hash: String = row.get(1)?;

        if existing_hash == file_hash {
            // Hashes match, update the file path if different (this might be redundant in this case)
            conn.execute("UPDATE file_embeddings SET file_path = ?1 WHERE id = ?2", params![file_path, id])?;
        } else {
            // Hashes differ, update the hash and re-compute embeddings
            let serialized_embedding = bincode::serialize(&embedding).map_err(|e| Box::new(e) as Box<dyn Error>)?;
            conn.execute("UPDATE file_embeddings SET file_hash = ?1, embedding = ?2 WHERE id = ?3", params![file_hash, serialized_embedding, id])?;
        }
    } else {
        // No entry exists, insert new record
        let serialized_embedding = bincode::serialize(&embedding).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        conn.execute(
            "INSERT INTO file_embeddings (file_hash, file_path, embedding) VALUES (?1, ?2, ?3)",
            params![file_hash, file_path, serialized_embedding],
        )?;
    }

    // Compute and store similarities
    compute_and_store_similarities(&conn)?;

    Ok(())
}

fn process_file(file_path: &str) -> Result<(String, Vec<f32>), Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Hash the file content
    let mut hasher = Sha256::new();
    hasher.update(&contents);
    let hash_result = hasher.finalize();
    let file_hash = format!("{:x}", hash_result);

    // Initialize the embedding model
    let model = TextEmbedding::try_new(InitOptions {
        model_name: EmbeddingModel::AllMiniLML6V2,
        show_download_progress: true,
        ..Default::default()
    })?;

    // Treat the entire file content as a single document for embedding
    let documents = vec![contents];
    let embeddings = model.embed(documents, None)?;

    // Assuming we want the first (and only) embedding
    let embedding = embeddings.into_iter().next().ok_or("Failed to generate embedding")?;

    Ok((file_hash, embedding))
}

fn compute_and_store_similarities(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT id, embedding FROM file_embeddings")?;
    let embeddings_iter = stmt.query_map([], |row| {
        let id: i32 = row.get(0)?;
        let embedding_blob: Vec<u8> = row.get(1)?;
        let embedding: Vec<f32> = bincode::deserialize(&embedding_blob).expect("Failed to deserialize$1");
        Ok((id, embedding))
    })?;

    let embeddings: HashMap<i32, Vec<f32>> = embeddings_iter.into_iter().collect::<Result<_, _>>()?;

    for (&id1, embedding1) in &embeddings {
        for (&id2, embedding2) in &embeddings {
            if id1 >= id2 { continue; }

            let similarity = cosine_similarity(&embedding1, &embedding2);
            conn.execute(
                "INSERT INTO similarities (file_id1, file_id2, similarity) VALUES (?1, ?2, ?3)
                 ON CONFLICT(file_id1, file_id2) DO UPDATE SET similarity=excluded.similarity",
                params![id1, id2, similarity],
            )?;
        }
    }

    Ok(())
}
// The cosine_similarity function remains unchanged
fn cosine_similarity(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
    dot_product / (norm_a * norm_b)
}
