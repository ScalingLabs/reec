use libmdbx::{
    orm::{table, Database},
    table_info,
};

use std::path::Path;


table!(
    /// Example table
    /// TODO: remove when adding real tables
    ( Example ) String => String
);

/// Initializes a new database with provided path. If the path is 'None', the database will be temporary.
pub fn init_db(path: Option<impl AsRef<Path>>) -> Database {
    let tables = [table_info!(Example)].into_iter().collect();
    let path = path.map(|p| p.as_ref().to_path_buf());
    Database::create(path, &tables).unwrap()
} 
