use chrono::{self, TimeZone, Utc};
use std::io;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn main() {
    let mut source = String::new();
    let mut dest = String::new();

    println!("Source Directory {}:", "(e.g. D:\\Photos\\Random 2023)");
    io::stdin().read_line(&mut source).expect("Failed to read line");

    println!("Destination Directory {}:",
        "(e.g. D:\\Photos\\Random 2023)");
    io::stdin().read_line(&mut dest).expect("Failed to read line");

    let source = source.trim();
    let dest = dest.trim();

    let source = Path::new(&source);
    let dest = Path::new(&dest);

    move_files(&source, &dest);

    println!("Press Enter to exit.");

    // Wait for user input (pressing Enter)
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("Exiting program.");
}

fn move_files(source: &Path, dest: &Path) -> std::io::Result<()> {
    if source.is_dir() {
        match fs::read_dir(source) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let photo_metadata = fs::metadata(entry.path())?;

                            let created = photo_metadata.created().expect("Creation Time unsupported");
                            let timestamp = created.duration_since(SystemTime::UNIX_EPOCH)
                                        .expect("File cannot be crated before Epoch")
                                        .as_nanos();
                            let datetime_utc = Utc.timestamp_nanos(timestamp.try_into().unwrap());
                            let destination_dir = format!("{}\\{}",dest.display().to_string(),datetime_utc.format("%Y-%m-%d"));
                            fs::create_dir_all(format!("{}\\{}",dest.display().to_string(),datetime_utc.format("%Y-%m-%d")))?;
                            let file_name = entry.file_name();

                            if let Some(file_name_str) = file_name.to_str() {
                                match fs::copy(entry.path(), format!("{}\\{}",&destination_dir,file_name_str)) {
                                    Ok(_) => println!("File copied successfully!"),
                                    Err(e) => println!("Error copying file: {}", e),
                                }
                            }


                            //println!("{}",datetime_utc.format("%Y-%m-%d"));
                        },
                        Err(e) => eprintln!("Error: {}", e)
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    Ok(())
}
