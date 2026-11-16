#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use clap::{Parser, Subcommand};
use kittycraft_core::command::{dispatch_tool, tools_list};
use kittycraft_core::skills::{export_skills, skill_by_name, skill_by_uri, skills_list};
use kittycraft_core::store::KittyStore;
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "kittycraft-mcp",
    version,
    about = "KittyCraft CLI & MCP server"
)]
struct Cli {
    #[arg(long, global = true)]
    json: bool,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Status,
    Breeds {
        #[arg(long, default_value = "")]
        query: String,
    },
    Skills {
        #[command(subcommand)]
        op: SkillsOp,
    },
    Mcp,
}

#[derive(Subcommand)]
enum SkillsOp {
    List,
    Show { name: String },
    Export { dir: PathBuf },
}

fn handle_mcp_line(store: &KittyStore, line: &str) -> Option<Value> {
    let trimmed = line.trim_start_matches('\u{feff}').trim();
    if trimmed.is_empty() {
        return None;
    }
    let req: Value = match serde_json::from_str(trimmed) {
        Ok(v) => v,
        Err(_) => {
            return Some(json!({
                "jsonrpc": "2.0",
                "id": Value::Null,
                "error": { "code": -32700, "message": "parse error" }
            }))
        }
    };
    let id = req.get("id")?.clone();
    if id.is_null() {
        return None;
    }
    let method = req.get("method").and_then(Value::as_str).unwrap_or("");
    let params = req.get("params").cloned().unwrap_or(Value::Null);
    let ok = |res: Value| json!({ "jsonrpc": "2.0", "id": id, "result": res });

    Some(match method {
        "initialize" => ok(json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": { "listChanged": false },
                "resources": { "listChanged": false }
            },
            "serverInfo": { "name": "kittycraft-mcp", "version": env!("CARGO_PKG_VERSION") }
        })),
        "ping" => ok(json!({})),
        "tools/list" => {
            let tools: Vec<Value> = tools_list()
                .into_iter()
                .map(|t| {
                    json!({
                        "name": t.name,
                        "description": t.description,
                        "inputSchema": t.input_schema
                    })
                })
                .collect();
            ok(json!({ "tools": tools }))
        }
        "tools/call" => {
            let name = params.get("name").and_then(Value::as_str).unwrap_or("");
            let args = params.get("arguments").cloned().unwrap_or(json!({}));
            match dispatch_tool(store, name, &args) {
                Ok(v) => ok(json!({
                    "content": [{ "type": "text", "text": serde_json::to_string_pretty(&v).unwrap_or_default() }],
                    "isError": false
                })),
                Err(e) => ok(json!({
                    "content": [{ "type": "text", "text": json!({ "error": e.to_string() }).to_string() }],
                    "isError": true
                })),
            }
        }
        "resources/list" => {
            let res: Vec<Value> = skills_list()
                .iter()
                .map(|s| {
                    json!({
                        "uri": s.uri,
                        "name": s.name,
                        "description": s.description,
                        "mimeType": "text/markdown"
                    })
                })
                .collect();
            ok(json!({ "resources": res }))
        }
        "resources/read" => {
            let uri = params.get("uri").and_then(Value::as_str).unwrap_or("");
            if let Some(s) = skill_by_uri(uri) {
                ok(json!({
                    "contents": [{ "uri": s.uri, "mimeType": "text/markdown", "text": s.skill_md }]
                }))
            } else {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": { "code": -32002, "message": format!("Resource not found: {uri}") }
                })
            }
        }
        other => json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": { "code": -32601, "message": format!("Method not found: {other}") }
        }),
    })
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    if let Cmd::Skills { op } = &cli.cmd {
        match op {
            SkillsOp::List => {
                let list = skills_list();
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(list).unwrap_or_default());
                } else {
                    for s in list {
                        println!("{}\t{}\t({} B)", s.name, s.uri, s.skill_md.len());
                    }
                }
                return ExitCode::SUCCESS;
            }
            SkillsOp::Show { name } => match skill_by_name(name) {
                Ok(s) => {
                    if cli.json {
                        println!("{}", serde_json::to_string_pretty(s).unwrap_or_default());
                    } else {
                        print!("{}", s.skill_md);
                    }
                    return ExitCode::SUCCESS;
                }
                Err(e) => {
                    eprintln!("kittycraft: {e}");
                    return ExitCode::FAILURE;
                }
            },
            SkillsOp::Export { dir } => match export_skills(dir) {
                Ok(paths) => {
                    for p in paths {
                        println!("exported {p}");
                    }
                    return ExitCode::SUCCESS;
                }
                Err(e) => {
                    eprintln!("kittycraft: {e}");
                    return ExitCode::FAILURE;
                }
            },
        }
    }

    let store = match KittyStore::open_default() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("kittycraft: cannot open store: {e}");
            return ExitCode::FAILURE;
        }
    };

    match cli.cmd {
        Cmd::Status => match dispatch_tool(&store, "kitty_status", &json!({})) {
            Ok(v) => {
                println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("kittycraft: {e}");
                ExitCode::FAILURE
            }
        },
        Cmd::Breeds { query } => {
            match dispatch_tool(&store, "kitty_breeds_list", &json!({ "query": query })) {
                Ok(v) => {
                    println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("kittycraft: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        Cmd::Mcp => {
            eprintln!("[kittycraft-mcp] MCP 2024-11-05 stdio ready");
            let stdin = io::stdin();
            let mut stdout = io::stdout().lock();
            for line in stdin.lock().lines().map_while(Result::ok) {
                if let Some(resp) = handle_mcp_line(&store, &line) {
                    let _ = writeln!(stdout, "{resp}");
                    let _ = stdout.flush();
                }
            }
            ExitCode::SUCCESS
        }
        Cmd::Skills { .. } => ExitCode::SUCCESS,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mcp_resources_and_tools_work() -> Result<(), Box<dyn std::error::Error>> {
        let store = KittyStore::open_in_memory()?;
        let r_list = handle_mcp_line(
            &store,
            r#"{"jsonrpc":"2.0","id":1,"method":"resources/list"}"#,
        )
        .ok_or("no reply")?;
        let res = r_list["result"]["resources"]
            .as_array()
            .ok_or("missing resources")?;
        assert_eq!(res.len(), 2);

        let r_read = handle_mcp_line(
            &store,
            r#"{"jsonrpc":"2.0","id":2,"method":"resources/read","params":{"uri":"skill://kittycraft-advisor/SKILL.md"}}"#,
        )
        .ok_or("no reply")?;
        assert!(r_read["result"]["contents"][0]["text"]
            .as_str()
            .unwrap_or("")
            .contains("KittyCraft"));

        let r_err = handle_mcp_line(
            &store,
            r#"{"jsonrpc":"2.0","id":3,"method":"resources/read","params":{"uri":"skill://missing/SKILL.md"}}"#,
        )
        .ok_or("no reply")?;
        assert_eq!(r_err["error"]["code"], -32002);
        Ok(())
    }
}
