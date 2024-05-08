use toml::value::Datetime;

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
