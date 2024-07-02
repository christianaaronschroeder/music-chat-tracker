use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Chat {
    id: i32,
}

#[derive(Debug)]
struct Message {
    id: i32,
    text: Option<String>,
    attributed_body: String,
}

fn get_chat_id(conn: &Connection, display_name: &str) -> Result<i32> {
    let mut stmt = conn
        .prepare("select * from chat where display_name=? and service_name='iMessage' limit 1")?;
    let chat_iter = stmt.query_map([display_name], |row| Ok(Chat { id: row.get(0)? }))?;

    for chat in chat_iter {
        return Ok(chat.unwrap().id);
    }
    Ok(-1)
}

fn get_messages(conn: &Connection, chat_id: &i32) -> Result<Vec<Message>, rusqlite::Error> {
    let query = format!(
        r#"
        SELECT
            message.ROWID,
            message.text,
            quote(message.attributedBody) AS text_data
           
        FROM message LEFT JOIN handle ON message.handle_id=handle.ROWID
        WHERE message.ROWID IN (SELECT message_id FROM chat_message_join WHERE chat_id={})
        "#,
        chat_id
    );
    let mut stmt = conn.prepare(&query)?;
    let message_iter = stmt.query_map([], |row| {
        Ok(Message {
            id: row.get(0)?,
            text: row.get(1)?,
            attributed_body: row.get(2)?,
        })
    })?;

    let messages: Result<Vec<Message>, rusqlite::Error> = message_iter.collect();
    messages
}

fn main() -> Result<()> {
    let conn = Connection::open("../Library/Messages/chat.db")?;

    let chat_id = get_chat_id(&conn, "Music (A Little Spam)")?;
    println!("Chat ID: {}", chat_id);
    let messages = get_messages(&conn, &chat_id)?;
    println!("Messages: {:?}", messages);
    Ok(())
}
