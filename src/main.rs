#[macro_use]
extern crate prettytable;
extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
use chrono::NaiveDateTime;
use prettytable::{cell, color, format, Attr, Row, Table};
use serde::Deserialize;

const GITHUB_RATE_URL: &str = "https://api.github.com/rate_limit";

#[derive(Deserialize)]
struct GithubResponse {
    resources: Resources,
    rate: Rate,
}

#[derive(Deserialize)]
struct Resources {
    core: Rate,
    search: Rate,
    graphql: Rate,
    integration_manifest: Rate,
    source_import: Rate,
    code_scanning_upload: Rate,
    actions_runner_registration: Rate,
    scim: Rate,
    dependency_snapshots: Rate,
    audit_log: Rate,
    code_search: Rate,
}

#[derive(Deserialize)]
struct Rate {
    limit: u32,
    used: u32,
    remaining: u32,
    reset: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = option_env!("GH_RATE_LIMIT").map_or_else(
        || {
            if std::env::args().len() < 2 {
                eprintln!("Usage: github_rate_checker <GitHub-Token>");
                std::process::exit(1);
            }
            std::env::args().nth(1).unwrap()
        },
        |token| token.to_string(),
    );

    let client = reqwest::blocking::Client::new();
    let response: GithubResponse = client
        .get(GITHUB_RATE_URL)
        .bearer_auth(token)
        .header(reqwest::header::USER_AGENT, "github_rate_checker")
        .send()?
        .json()?;

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.add_row(Row::new(vec![
        cell!("Resource")
            .with_style(Attr::ForegroundColor(color::BLUE))
            .with_style(Attr::Bold),
        cell!("Limit")
            .with_style(Attr::ForegroundColor(color::BLUE))
            .with_style(Attr::Bold),
        cell!("Used")
            .with_style(Attr::ForegroundColor(color::BLUE))
            .with_style(Attr::Bold),
        cell!("Remaining")
            .with_style(Attr::ForegroundColor(color::BLUE))
            .with_style(Attr::Bold),
        cell!("Reset")
            .with_style(Attr::ForegroundColor(color::BLUE))
            .with_style(Attr::Bold),
    ]));
    add_rate_to_table(&mut table, "core", &response.resources.core);
    add_rate_to_table(&mut table, "search", &response.resources.search);
    add_rate_to_table(&mut table, "graphql", &response.resources.graphql);
    add_rate_to_table(
        &mut table,
        "integration_manifest",
        &response.resources.integration_manifest,
    );
    add_rate_to_table(
        &mut table,
        "source_import",
        &response.resources.source_import,
    );
    add_rate_to_table(
        &mut table,
        "code_scanning_upload",
        &response.resources.code_scanning_upload,
    );
    add_rate_to_table(
        &mut table,
        "actions_runner_registration",
        &response.resources.actions_runner_registration,
    );
    add_rate_to_table(&mut table, "scim", &response.resources.scim);
    add_rate_to_table(
        &mut table,
        "dependency_snapshots",
        &response.resources.dependency_snapshots,
    );
    add_rate_to_table(&mut table, "audit_log", &response.resources.audit_log);
    add_rate_to_table(&mut table, "code_search", &response.resources.code_search);
    add_rate_to_table(&mut table, "Overall", &response.rate);

    table.printstd();

    Ok(())
}

fn add_rate_to_table(table: &mut Table, resource_name: &str, rate: &Rate) {
    let formatted_datetime = match NaiveDateTime::from_timestamp_opt(rate.reset as i64, 0) {
        Some(datetime) => datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
        None => "Invalid Timestamp".to_string(),
    };
    table.add_row(row![
        resource_name,
        rate.limit,
        rate.used,
        rate.remaining,
        formatted_datetime
    ]);
}
