use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;

// =============================================================================
// EDUCATIONAL COMMENTS: Common Rust Concepts
// =============================================================================
//
// Arc<T> (Atomic Reference Counted):
//   A smart pointer that allows MULTIPLE owners of the same data across threads.
//   Normal variables have ONE owner. Arc lets you share ownership safely.
//   Example: let data = Arc::new(vec![1, 2, 3]);
//            let data_clone = Arc::clone(&data); // Both point to same vec
//            // Now you can send data_clone to another thread
//
// Mutex<T> (Mutual Exclusion):
//   A lock that ensures only ONE thread can access the data at a time.
//   Prevents race conditions (two threads writing at the same time).
//   Example: let counter = Mutex::new(0);
//            let mut num = counter.lock().unwrap(); // Lock it, get access
//            *num += 1; // Modify the value
//            // Lock is automatically released when `num` goes out of scope
//
// Arc<Mutex<T>> (Combined):
//   Used together to share mutable data across threads safely.
//   Arc = multiple threads can own it, Mutex = only one can modify at a time.
//   Example: let shared_counter = Arc::new(Mutex::new(0));
//            let counter_clone = Arc::clone(&shared_counter);
//            // Send counter_clone to another thread, both can safely increment
//
// .unwrap():
//   Extracts the value from Option<T> or Result<T, E>.
//   If Option is None or Result is Err, it PANICS (crashes the program).
//   Use only when you're CERTAIN the value exists, or in quick prototypes.
//   Example: let x: Option<i32> = Some(5);
//            let value = x.unwrap(); // value = 5
//            let y: Option<i32> = None;
//            let crash = y.unwrap(); // PANIC! Program crashes here
//   Safer alternatives: .unwrap_or(default), .expect("error msg"), or match/if let
//
// .clone():
//   Creates a deep copy of a value. The new copy is independent of the original.
//   Example: let s1 = String::from("hello");
//            let s2 = s1.clone(); // s2 is a separate copy
//            // Both s1 and s2 are valid and independent
//   Note: Arc::clone() is cheap (just increments a counter), but cloning
//         large data structures can be expensive.
//
// =============================================================================

// This is the function we will call from main.rs
pub fn process_directory(target_path: &str, dry_run: bool) -> Result<(), ()> {
    let root = Path::new(target_path);

    println!("Scanning directory: {:?}", root);

    let entries: Vec<PathBuf> = WalkDir::new(root)
        .into_iter()
        .filter_map(|x| x.ok()) // Ignore errors (like permission denied)
        .filter(|x| x.path().is_file()) // Ignore folders, only look at files
        .map(|x| x.path().to_owned()) // Convert to PathBuf (owns the data)
        .collect();

    println!("Found {} files", entries.len());

    // Arc<Mutex<HashMap>> explained:
    // - HashMap tracks how many files of each extension we moved
    // - Mutex ensures only one thread updates the map at a time (prevents data corruption)
    // - Arc allows multiple threads to share ownership of the Mutex<HashMap>
    let stats: Arc<Mutex<HashMap<String, i32>>> = Arc::new(Mutex::new(HashMap::new()));

    // .par_iter() distributes the work across all your CPU cores automatically (parallel processing of files)
    entries.par_iter().for_each(|file_path| {
        // Arc::clone() creates another pointer to the SAME data (cheap, just increments counter)
        // We need this because each thread needs its own Arc handle to access the shared stats
        let stats_clone = Arc::clone(&stats);

        // organize_file returns Option<String> - the extension if file was moved, None otherwise
        if let Some(ext) = organize_file(file_path, root, dry_run) {
            // .lock() acquires the mutex lock - blocks until we get exclusive access
            // .unwrap() extracts the MutexGuard or panics if the lock is poisoned
            let mut map = stats_clone.lock().unwrap();
            *map.entry(ext).or_insert(0) += 1;
        }
    });

    // .lock().unwrap() - acquire the lock to read the final stats
    let final_stats = stats.lock().unwrap();
    println!("--- Organization Complete ---");
    for (ext, count) in final_stats.iter() {
        println!("[.{}] : {} files", ext, count);
    }

    Ok(())
}

// Logic for a single file
// Returns Some(extension) if file was moved, None if skipped
pub fn organize_file(file_path: &Path, root: &Path, dry_run: bool) -> Option<String> {
    // 1. Get the file extension
    // If no extension -> We just skip it (return None)
    let extension = match file_path.extension() {
        Some(ext) => ext.to_string_lossy().to_lowercase(),
        None => return None,
    };

    // 2. Create the destination folder (e.g. "./Downloads/pdf")
    let dest_folder = root.join(&extension);

    // 3. Create the full destination file path (e.g. "./Downloads/pdf/document.pdf")
    // .file_name() returns Option<&OsStr>, we use ? to return None if it fails
    let file_name = file_path.file_name()?;
    let dest_path = dest_folder.join(file_name);

    // 4. Don't move the file if it's already in the right place
    if dest_path == file_path {
        return None;
    }

    // 5. The Moving logic
    if dry_run {
        println!("[DRY RUN] Would move {:?} -> {:?}", file_path, dest_path);
    } else {
        // A. Create the directory if it doesn't exist
        if let Err(e) = fs::create_dir_all(&dest_folder) {
            eprintln!("Failed to create directory {:?}: {}", dest_folder, e);
            return None;
        }

        // B. Move the file (fixed: was using dest_folder instead of dest_path)
        if let Err(e) = fs::rename(file_path, &dest_path) {
            eprintln!("Failed to move {:?}: {}", file_path, e);
            return None;
        }

        println!("Moved {:?} -> {:?}", file_path, dest_path);
    }

    // .clone() creates a copy of the extension String so we can return it
    // (the original `extension` would be dropped at end of function)
    Some(extension.clone())
}
