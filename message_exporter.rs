use rusqlite::{Connection, params};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("path/to/database.sqlite")?;

    let mut chats = conn.prepare("SELECT * FROM chats")?;
    let mut chat_iter = chats.query_map(params![], |row| {
        let guid: String = row.get("guid")?;
        let chat_id: i64 = row.get("ROWID")?;

        // Process other fields and perform string operations

        Ok(())
    })?;

    // Iterate over chat_iter and perform message retrieval and processing

    Ok(())
}
