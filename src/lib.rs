/// Core library
use rusqlite::Connection;
use textwrap::{indent, Options};
// use serde::Serialize;
use anyhow::Result as AResult;

/// Returns a query depending on examples (bool value)
fn make_query(examples: bool) -> &'static str {
    match examples {
        false => {
            r#"
        SELECT ld.posid,ss.definition
        FROM words w
        JOIN senses s ON w.wordid = s.wordid
        JOIN synsets ss ON s.synsetid = ss.synsetid
        JOIN domains ld ON ss.domainid = ld.domainid
        WHERE w.word = ?
        GROUP BY ss.synsetid
        ORDER BY ld.posid, s.senseid;
        "#
        }
        true => {
            r#"
        SELECT ld.posid,
               ss.definition,
               GROUP_CONCAT(sm.sample, '; ') as examples
        FROM words w
        JOIN senses s ON w.wordid = s.wordid
        JOIN synsets ss ON s.synsetid = ss.synsetid
        JOIN domains ld ON ss.domainid = ld.domainid
        LEFT JOIN samples sm ON ss.synsetid = sm.synsetid
        WHERE w.word = ?
        GROUP BY ss.synsetid
        ORDER BY ld.posid, s.senseid;
        "#
        }
    }
}

/// Display meanings without examples
pub fn display_meanings(conn: &Connection, word: &str) -> AResult<()> {
    // Modified query to include part of speech
    let query = make_query(false);
    let mut stmt = conn.prepare(query)?;

    println!("\nMeanings of '{}':", word);

    let rows = stmt.query_map([word], |row| {
        Ok((
            row.get::<_, String>(0)?, // pos
            row.get::<_, String>(1)?, // definition
        ))
    })?;

    // Configure text wrapping
    let wrap_options = Options::new(70)
        .initial_indent("      ")
        .subsequent_indent("      ");

    for row in rows {
        let (pos, definition) = row?;

        // Print part of speech and definition
        let pos_symbol = match pos.as_str() {
            "n" => "[n]",
            "v" => "[v]",
            "a" | "s" => "[adj]", // 'a' for adjective, 's' for satellite adjective
            "r" => "[adv]",
            _ => "[?]",
        };
        println!("{} {}", pos_symbol, definition);
    }

    Ok(())
}

pub fn display_meanings_with_examples(conn: &Connection, word: &str) -> AResult<()> {
    // Modified query to include part of speech
    let query = make_query(true);
    let mut stmt = conn.prepare(query)?;

    println!("\nMeanings of '{}':", word);

    let rows = stmt.query_map([word], |row| {
        Ok((
            row.get::<_, String>(0)?,         // pos
            row.get::<_, String>(1)?,         // definition
            row.get::<_, Option<String>>(2)?, // examples
        ))
    })?;

    // Configure text wrapping
    let wrap_options = Options::new(70)
        .initial_indent("      ")
        .subsequent_indent("      ");

    for row in rows {
        let (pos, definition, examples) = row?;

        // Print part of speech and definition
        let pos_symbol = match pos.as_str() {
            "n" => "[n]",
            "v" => "[v]",
            "a" | "s" => "[adj]", // 'a' for adjective, 's' for satellite adjective
            "r" => "[adv]",
            _ => "[?]",
        };
        println!("{} {}", pos_symbol, definition);

        // dbg!(Print) examples if they exist
        /*
            if let Some(examples_str) = examples {
                let examples: Vec<&str> = examples_str.split("; ").collect();
                for example in examples {
                    if !example.is_empty() {
                        let cleaned_example = example.trim_matches('"');
                        println!("      \"{}\"", cleaned_example);
                    }
                }
            }
        */

        // dbg!(Print) examples if they exist
        if let Some(examples_str) = examples {
            examples_str
                .split("; ")
                .filter(|e| !e.is_empty())
                .map(|e| e.trim_matches('"'))
                .for_each(|cleaned| println!("      \"{}\"", cleaned));
        }

        // Print synonyms if they exist
        /*
        if let Some(syns) = synonyms.filter(|s| !s.is_empty()) {
                println!("      Synonyms: {}", syns);
        }
        */
    }

    Ok(())
}
