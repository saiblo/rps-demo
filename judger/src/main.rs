#![feature(ascii_char, byte_slice_trim_ascii)]

use std::process::Stdio;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::{Child, Command},
};

const BUFFER_SIZE: usize = 500;
const MAX_ROUND: usize = 512;

async fn read_child_stdout(child: &mut Child) -> tokio::io::Result<String> {
    let stdout = child.stdout.as_mut().unwrap();
    let mut output = vec![0; BUFFER_SIZE];
    stdout.read(&mut output).await?;
    unsafe {
        Ok(output
            .as_ascii_unchecked()
            .as_str()
            .trim_end_matches(|c| c == '\0' || c == '\n')
            .to_owned())
    }
}

async fn write_child_stdin(child: &mut Child, content: &str) -> tokio::io::Result<()> {
    let stdin = child.stdin.as_mut().unwrap();
    stdin.write(content.as_bytes()).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let ai_path = "../ai/target/debug/ai";
    let logic_path = "../target/debug/rps-demo";

    let mut logic = Command::new(logic_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut ais = [
        Command::new(ai_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?,
        Command::new(ai_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?,
        Command::new(ai_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?,
    ];
    let init = r#"{"initdata": {"use_combo_scoring": true}}
"#;
    write_child_stdin(&mut logic, init).await?;

    for round in 0..=MAX_ROUND {
        // Read logic input
        let logic_output = read_child_stdout(&mut logic).await?;
        println!("Round {}: Output {}", round, logic_output);

        // Write to ai
        for ai in &mut ais {
            write_child_stdin(ai, "unnecessary\n").await?;
        }

        // Read from ai
        let mut ai_output = Vec::new();
        for ai in &mut ais {
            ai_output.push(read_child_stdout(ai).await?);
        }

        // Prepare output
        let logic_input = format!(
            "{{\"log\": {{\"0\": {{ \"verdict\": \"OK\", \"response\": {} }}, \"1\": {{ \"verdict\": \"OK\", \"response\": {} }}, \"2\": {{ \"verdict\": \"OK\", \"response\": {} }}}}}}\n",
            ai_output[0], ai_output[1], ai_output[2]
        );
        println!("{}", logic_input);

        write_child_stdin(&mut logic, &logic_input).await?;
    }

    // Read logic input
    let logic_output = read_child_stdout(&mut logic).await?;
    println!("Game Over!!!");
    println!("Result {}", logic_output);

    // Kill child process
    logic.kill().await?;
    for ai in &mut ais {
        ai.kill().await?;
    }

    Ok(())
}
