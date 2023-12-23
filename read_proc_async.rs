use std::path::PathBuf;
use tokio::fs;
#[derive(Debug)]
struct ProcessRawInfo {
    pid: u32,
    stat: String,
}

enum FileSystemError {
    Error1
}

async fn read_process_info(proc_path: &str, pid: u32) -> Option<ProcessRawInfo> {
    // Construct the path to the 'stat' file for the process
    let stat_path = PathBuf::from(proc_path).join(&pid.to_string()).join("stat");
    // Read the 'stat' file
    if let Ok(stat) = fs::read_to_string(&stat_path).await {
        return Some(ProcessRawInfo {
            pid,
            stat
        })    
    }
    None
}

async fn read_process_raw_info_async(proc_path: &str) -> Result<Vec<ProcessRawInfo>, FileSystemError> {
    let mut tasks = Vec::new();
    let paths = std::fs::read_dir(proc_path).expect("Failed to read /proc directory");

    for entry in paths {
        let entry = entry.expect("Failed to read directory entry");
        if let Ok(pid) = entry.file_name().to_string_lossy().parse::<u32>() {
            let s = proc_path.to_string();
            // Spawn a new task for each PID and store the future
            tasks.push(tokio::spawn(async move {
                read_process_info(&s, pid).await
            }));
        }
    }

    // Collect all results
    let mut process_infos = Vec::new();
    for task in tasks {
        if let Some(info) = task.await.expect("The task being joined has panicked") {
            process_infos.push(info);
        }
    }

    Ok(process_infos)
}

#[tokio::main]
async fn main() {
    // Read the /proc directory
    if let Ok(res) = read_process_raw_info_async("/proc").await {
        for r in &res {
            println!("Process: {r:?}");
        }
    } else {
        println!("Nothing?");
    }

}
