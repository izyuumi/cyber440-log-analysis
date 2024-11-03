use chrono::NaiveDateTime;

use std::{collections::HashMap, io::Write};

const H_MAIL_SERVER_LOG_FILE: &str = "hmailserver.log";

#[derive(Debug, PartialEq, Eq, Hash)]
enum LogType {
    DEBUG,
    TCPIP,
    IMAPD,
    SMTPD,
    APPLICATION,
}

#[derive(Debug)]
struct Line {
    date: NaiveDateTime,
    raw: String,
}

fn main() {
    // loop through each log file
    let file = std::fs::read_to_string(H_MAIL_SERVER_LOG_FILE).unwrap();

    let lines: HashMap<LogType, Vec<Line>> =
        file.split("\n").fold(HashMap::new(), |mut map, line| {
            if line.is_empty() {
                return map;
            }

            let second_quotation_index = line[1..].find("\"").unwrap() + 1;
            let log_type = match &line[1..second_quotation_index] {
                "DEBUG" => LogType::DEBUG,
                "TCPIP" => LogType::TCPIP,
                "IMAPD" => LogType::IMAPD,
                "SMTPD" => LogType::SMTPD,
                "APPLICATION" => LogType::APPLICATION,
                _ => unreachable!("Unknown log type: {}", line),
            };

            let split = line.split("\t").collect::<Vec<&str>>();

            // 2020-02-28 15:06:25.816
            let date = match log_type {
                LogType::DEBUG | LogType::TCPIP | LogType::APPLICATION => {
                    NaiveDateTime::parse_from_str(&split[2][1..22], "%Y-%m-%d %H:%M:%S%.3f")
                        .unwrap()
                }
                LogType::IMAPD | LogType::SMTPD => {
                    NaiveDateTime::parse_from_str(&split[3][1..22], "%Y-%m-%d %H:%M:%S%.3f")
                        .unwrap()
                }
            };

            map.entry(log_type).or_default().push(Line {
                date,
                raw: line.to_string(),
            });

            map
        });

    login(lines.get(&LogType::IMAPD).unwrap());
}

/// check for login logs
fn login(lines: &Vec<Line>) {
    // create a file to write the output
    let mut file = std::fs::File::create("login.csv").unwrap();

    // write the header
    writeln!(file, "Date,IP Address,Info").unwrap();

    // loop through each line
    for line in lines {
        let date = line.date;
        let line = line.raw.clone();

        // ignore if the line doesn't contains "login"
        if !line.to_lowercase().contains("login") {
            continue;
        }

        // split the line into parts by tab, trim quotation marks
        let parts: Vec<&str> = line.split("\t").map(|p| p.trim_matches('\"')).collect();

        // write the output
        writeln!(file, "{},{},{}", date, parts[4], parts[5]).unwrap();
    }

    println!("login.csv created");

    // close the file
    file.flush().unwrap();
}
