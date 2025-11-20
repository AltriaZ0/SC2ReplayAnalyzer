#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::thread;
use tauri::path::BaseDirectory;
use tauri::{command, AppHandle, Manager, Emitter};
use std::{
    fs,
    path::{Path, PathBuf},
};

// ---------- 数据结构 ----------
#[derive(Serialize, Deserialize)]
struct AnalyzeOptions {
    analyze_type: String,
    output_dir: String,
    fulltime: bool,
    cancel: bool,
    buildList: bool,
    upgradeList: bool,
    UnitList: bool,
    terranFly: bool,
    workerNumber: bool,
    exportXlsx: bool,
    exportTxt: bool,
    exportSummary: bool
    // tz: String,
    // lang: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzeResult {
    #[serde(rename = "map_name")]
    pub map_name: String,
    #[serde(rename = "ladder?", default)]
    pub ladder: bool,
    pub duration: u32,
    #[serde(rename = "playersInfo")]
    pub players_info: HashMap<String, PlayerInfo>,
    pub winner: String,
    pub region: String,
    #[serde(rename = "endTime")]
    end_time: String,
    #[serde(rename = "raceBattle")]
    pub race_battle: String,
    pub output: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub race: String,
    pub result: bool,
    pub buildOrder: Vec<BoStep>,
    pub outputPath: String,
    pub versus: String,
    pub note: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BoStep {
    pub t: String,
    pub action: String,
}

// 发送给前端的进度条结构
#[derive(Serialize, Clone)]
struct ProgressPayload {
    percentage: f64,
    message: String,
}

// ---------- Tauri 命令 ----------
#[command]
async fn analyze_replay(
    app: AppHandle,
    path: String,
    options: AnalyzeOptions,
) ->Result<Option<AnalyzeResult>, String> {
    #[cfg(target_os = "windows")]
    let rel = "bin/main.exe";
    #[cfg(not(target_os = "windows"))]
    let rel = "bin/main";

    let py_bin = app
        .path()
        .resolve(rel, BaseDirectory::Resource)
        .map_err(|e| format!("resolve py bin error: {e}"))?;

    // 启动子进程
    let mut child = Command::new(py_bin)
        .arg("json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())   // 可选：捕获错误日志
        .spawn()
        .map_err(|e| format!("无法启动 Python 进程: {}", e))?;

    // 写入请求 JSON
    let input = serde_json::json!({
      "path": path,
      "options": options
    })
    .to_string();

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(input.as_bytes())
            .map_err(|e| format!("stdin write error: {e}"))?;
    }


    // 3. 处理 stderr (开启独立线程读取，防止 stderr 缓冲区满导致死锁)
    let stderr = child.stderr.take().ok_or("failed to capture stderr")?;
    let stderr_thread = thread::spawn(move || {
        let reader = BufReader::new(stderr);
        let mut err_log = String::new();
        for line in reader.lines() {
            if let Ok(l) = line {
                eprintln!("Python Stderr: {}", l); // 在 Rust 控制台打印以便调试
                err_log.push_str(&l);
                err_log.push('\n');
            }
        }
        err_log
    });

    // 4. 处理 stdout (主线程逐行读取并分发)
    let stdout = child.stdout.take().ok_or("failed to capture stdout")?;
    let reader = BufReader::new(stdout);
    
    let mut final_result: Option<AnalyzeResult> = None;

    for line in reader.lines() {
        let line_str = line.map_err(|e| format!("Error reading line: {}", e))?;
        
        // 忽略空行
        if line_str.trim().is_empty() { continue; }

        // 尝试解析为通用 JSON
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&line_str) {
            // 根据 "type" 字段判断消息类型
            let msg_type = parsed.get("type").and_then(|t| t.as_str());
            
            match msg_type {
                Some("progress") => {
                    // ---> 发送事件给前端
                    if let Some(payload) = parsed.get("payload") {
                        // 这里不做严格的错误处理，发不出去就算了
                        let _ = app.emit("analyze_progress", payload);
                    }
                }
                Some("result") => {
                    // ---> 捕获最终结果
                    if let Some(payload) = parsed.get("payload") {
                        // 将 payload 也就是之前定义的 AnalyzeResult 结构转换出来
                        if let Ok(res) = serde_json::from_value::<AnalyzeResult>(payload.clone()) {
                            final_result = Some(res);
                        } else {
                            println!("Failed to deserialize result payload: {:?}", payload);
                        }
                    }
                }
                _ => {
                    // 未知类型的 JSON，可能是普通的 log
                    println!("Python Output (Unknown JSON): {}", line_str);
                }
            }
        } else {
            // 不是 JSON，直接打印
            println!("Python Output (Raw): {}", line_str);
        }
    }

    // 5. 等待进程结束
    let status = child.wait().map_err(|e| format!("wait error: {e}"))?;
    let stderr_output = stderr_thread.join().unwrap_or_else(|_| "Thread panic".to_string());

    if !status.success() {
        return Err(format!(
            "python exit code: {:?}\nstderr:\n{}",
            status, stderr_output
        ));
    }

    // 6. 返回结果
    match final_result {
        Some(res) => Ok(Some(res)),
        None => Ok(None)
        // None => Err(format!("Python finished but returned no result. Stderr:\n{}", stderr_output)),
    }


}

fn count_sc2_replays(dir: &Path) -> std::io::Result<u64> {
    let mut total = 0u64;

    if !dir.is_dir() {
        return Ok(0);
    }

    for entry_res in fs::read_dir(dir)? {
        let entry = entry_res?;
        let path = entry.path();

        if path.is_dir() {
            total += count_sc2_replays(&path)?;
        } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if ext.eq_ignore_ascii_case("sc2replay") {
                total += 1;
            }
        }
    }
    println!("total = {}", total);
    Ok(total)
}

#[tauri::command]
fn get_replay_meta(path: String) -> Result<u64, String> {
    let path = PathBuf::from(&path);


    if !path.is_dir() {
        return Err("path is not a directory".to_string());
    }

    let count = count_sc2_replays(&path).map_err(|e| e.to_string())?;

    Ok(count)
}



fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![analyze_replay, get_replay_meta])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
