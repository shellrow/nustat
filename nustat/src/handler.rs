use inquire::Confirm;
use nustat_core::db::ip::{AS_BIN_NAME, COUNTRY_BIN_NAME, IPV4_INFO_BIN_NAME, IPV6_INFO_BIN_NAME};
use nustat_core::net::http::DownloadProgress;
use indicatif::ProgressBar;

pub fn check_db_files() -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = if let Some(path) = nustat_core::sys::get_config_dir_path() {
        path
    } else {
        return Err("Could not get config directory path".into());
    };
    let db_files_exists: bool = match nustat_core::db::ip::IpDatabase::check_db_files() {
        Ok(_) => {
            true
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            false
        }
    };
    if db_files_exists {
        return Ok(());
    }
    // DB files not found or corrupted
    let ans: bool = Confirm::new("DB files not found or corrupted. Do you want to download them now?")
        .prompt()
        .unwrap();
    if ans == false {
        println!("Aborted. You can download the DB files later by running the following command: nustat update --db");
        println!("or you can download the DB files manually from the GitHub repository, and place them in the following directory: {}", config_dir.to_string_lossy());
        println!("Exiting...");
        std::process::exit(0);
    }
    // Download the DB files from the GitHub repository
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // Check latest commit info
        let mut commit_hash = String::new();
        match nustat_core::net::http::get_commit_info().await {
            Ok(commit_info) => {
                commit_hash = commit_info.commit.sha;
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
        // Save file path is config_dir + DB file name
        // Download the latest ipv4 file
        let ipv4_db_url = nustat_core::db::ip::DbIpv4Info::get_github_url(&commit_hash);
        println!("Downloading ipv4 db file from: {}", ipv4_db_url);
        // create a channel for progress
        let (progress_tx, mut progress_rx) = tokio::sync::mpsc::channel(100);
        let save_file_path = config_dir.join(IPV4_INFO_BIN_NAME);
        // spawn a task to handle the progress
        tokio::spawn(async move {
            let _ = nustat_core::net::http::download_file_with_progress(ipv4_db_url, save_file_path, progress_tx).await;
        });
        // Display progress with indicatif
        let bar = ProgressBar::new(1000);
        bar.set_style(indicatif::ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})").progress_chars("#>-"));
        while let Some(progress) = progress_rx.recv().await {
            match progress {
                DownloadProgress::ContentLength(content_length) => {
                    println!("Content-Length: {}", content_length);
                    bar.set_length(content_length);
                }
                DownloadProgress::Downloaded(downloaded) => {
                    bar.set_position(downloaded);
                }
            }
        }
        bar.finish();

        // Download the latest ipv6 file
        let ipv6_db_url = nustat_core::db::ip::DbIpv6Info::get_github_url(&commit_hash);
        println!("Downloading ipv6 db file from: {}", ipv6_db_url);
        let save_file_path = config_dir.join(IPV6_INFO_BIN_NAME);
        // create a channel for progress
        let (progress_tx, mut progress_rx) = tokio::sync::mpsc::channel(100);
        // spawn a task to handle the progress
        tokio::spawn(async move {
            let _ = nustat_core::net::http::download_file_with_progress(ipv6_db_url, save_file_path, progress_tx).await;
        });
        // Display progress with indicatif
        let bar = ProgressBar::new(1000);
        bar.set_style(indicatif::ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})").progress_chars("#>-"));
        while let Some(progress) = progress_rx.recv().await {
            match progress {
                DownloadProgress::ContentLength(content_length) => {
                    println!("Content-Length: {}", content_length);
                    bar.set_length(content_length);
                }
                DownloadProgress::Downloaded(downloaded) => {
                    bar.set_position(downloaded);
                }
            }
        }
        bar.finish();

        // Download the latest country file
        let country_db_url = nustat_core::db::ip::Country::get_github_url(&commit_hash);
        println!("Downloading country db file from: {}", country_db_url);
        let save_file_path = config_dir.join(COUNTRY_BIN_NAME);
        // create a channel for progress
        let (progress_tx, mut progress_rx) = tokio::sync::mpsc::channel(100);
        // spawn a task to handle the progress
        tokio::spawn(async move {
            let _ = nustat_core::net::http::download_file_with_progress(country_db_url, save_file_path, progress_tx).await;
        });
        // Display progress with indicatif
        let bar = ProgressBar::new(1000);
        bar.set_style(indicatif::ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})").progress_chars("#>-"));
        while let Some(progress) = progress_rx.recv().await {
            match progress {
                DownloadProgress::ContentLength(content_length) => {
                    println!("Content-Length: {}", content_length);
                    bar.set_length(content_length);
                }
                DownloadProgress::Downloaded(downloaded) => {
                    bar.set_position(downloaded);
                }
            }
        }
        bar.finish();

        // Download the latest AS file
        let as_db_url = nustat_core::db::ip::AutonomousSystem::get_github_url(&commit_hash);
        println!("Downloading AS db file from: {}", as_db_url);
        let save_file_path = config_dir.join(AS_BIN_NAME);
        // create a channel for progress
        let (progress_tx, mut progress_rx) = tokio::sync::mpsc::channel(100);
        // spawn a task to handle the progress
        tokio::spawn(async move {
            let _ = nustat_core::net::http::download_file_with_progress(as_db_url, save_file_path, progress_tx).await;
        });
        // Display progress with indicatif
        let bar = ProgressBar::new(1000);
        bar.set_style(indicatif::ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})").progress_chars("#>-"));
        while let Some(progress) = progress_rx.recv().await {
            match progress {
                DownloadProgress::ContentLength(content_length) => {
                    println!("Content-Length: {}", content_length);
                    bar.set_length(content_length);
                }
                DownloadProgress::Downloaded(downloaded) => {
                    bar.set_position(downloaded);
                }
            }
        }
        bar.finish();
        println!("DB files downloaded successfully.")
    });
    Ok(())
}
