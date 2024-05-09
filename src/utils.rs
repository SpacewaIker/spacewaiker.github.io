use toml::{value::Datetime, Table};

/// Formats a date in the format "Month Year".
///
/// # Panics
/// Panics if the datetime has no date
#[must_use]
pub fn format_date(datetime: &Datetime, lang: &str) -> String {
    let date = datetime.date.unwrap();

    let month = if lang == "fr" {
        match date.month {
            1 => "janvier",
            2 => "février",
            3 => "mars",
            4 => "avril",
            5 => "mai",
            6 => "juin",
            7 => "juillet",
            8 => "août",
            9 => "septembre",
            10 => "octobre",
            11 => "novembre",
            12 => "décembre",
            _ => "inconnu",
        }
    } else {
        match date.month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "unknown",
        }
    };

    format!("{} {}", month, date.year)
}

/// Reroots the table to bring the language keys to the top level
pub fn reroot(table: Table) -> Table {
    let mut table_en = Table::new();
    let mut table_fr = Table::new();

    for (key, value) in table {
        if let toml::Value::Table(table) = value {
            if table.contains_key("en") && table.contains_key("fr") {
                table_en.insert(key.clone(), table.get("en").unwrap().clone());
                table_fr.insert(key.clone(), table.get("fr").unwrap().clone());
                continue;
            }
            table_en.insert(key.clone(), toml::Value::Table(table.clone()));
            table_fr.insert(key.clone(), toml::Value::Table(table.clone()));
        } else {
            table_en.insert(key.clone(), value.clone());
            table_fr.insert(key.clone(), value.clone());
        }
    }

    let mut new_table = Table::new();
    new_table.insert("en".to_string(), toml::Value::Table(table_en));
    new_table.insert("fr".to_string(), toml::Value::Table(table_fr));
    new_table
}
