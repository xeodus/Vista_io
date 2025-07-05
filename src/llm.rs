use std::{io::Write, process::{ Command as ProcessCommand, Stdio }};
use std::result::Result::Ok;

pub fn call_llm(prompt: &str, model: &str) -> Result<String, String> {
    let script_path = std::path::Path::new("script_manager/llm_call.py");

    if !script_path.exists() {
        eprintln!("Missing script_manager/llm_call.py path");
    }

    let mut child = ProcessCommand::new("python3")
        .arg(script_path)
        .arg(model)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start the subprocess: {}", e))?;

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(prompt.as_bytes())
        .map_err(|_| format!("Cannot write the prompt to stdin for our subprocess"));
    }

    let output = child
        .wait_with_output().map_err(|_| "Failed to fetch output for the subprocess")?;

    
    if output.status.success() {
        match String::from_utf8(output.stdout) {
            Ok(stdout) => Ok(stdout),
            _ => unreachable!("Programs only keeps valid outputs by its subprocesses..")
        }
    }
    else {
        let stderr = String::from_utf8(output.stderr).expect("Unreadable error received..");
        return Err(format!("LLM subprocess error: {}", stderr));
    }


}