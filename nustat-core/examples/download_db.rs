// Download bin DB files from the internet and save them to the local file system
fn main() {
    // tokio runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // Check latest commit info
        let mut commit_hash = String::new();
        match nustat_core::net::http::get_commit_info().await {
            Ok(commit_info) => {
                println!("Latest commit info: {:?}", commit_info);
                commit_hash = commit_info.commit.sha;
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }

        // Save file path is ./temp
        let temp_dir = "./temp";

        // Download the latest ipv4 file
        let ipv4_db_url = nustat_core::db::ip::DbIpv4Info::get_github_url(&commit_hash);
        println!("Downloading ipv4 db file from: {}", ipv4_db_url);
        // create a channel for progress
        let (progress_tx, mut progress_rx) = tokio::sync::mpsc::channel(100);
        // spawn a task to handle the progress
        let save_file_path = format!("{}/{}", temp_dir, "ipv4.bin");
        tokio::spawn(async move {
            let _ = nustat_core::net::http::download_file_with_progress(ipv4_db_url, save_file_path, progress_tx).await;
        });
        while let Some(progress) = progress_rx.recv().await {
            println!("Downloaded: {:?}", progress);
        }

        // Download the latest ipv6 file
        let ipv6_db_url = nustat_core::db::ip::DbIpv6Info::get_github_url(&commit_hash);
        println!("Downloading ipv6 db file from: {}", ipv6_db_url);
        let (progress_tx, mut progress_rx) = tokio::sync::mpsc::channel(100);
        let save_file_path = format!("{}/{}", temp_dir, "ipv6.bin");
        tokio::spawn(async move {
            let _ = nustat_core::net::http::download_file_with_progress(ipv6_db_url, save_file_path, progress_tx).await;
        });
        while let Some(progress) = progress_rx.recv().await {
            println!("Downloaded: {:?}", progress);
        }

        // Download the latest asn file
        let asn_db_url = nustat_core::db::ip::AutonomousSystem::get_github_url(&commit_hash);
        println!("Downloading asn db file from: {}", asn_db_url);
        let (progress_tx, mut progress_rx) = tokio::sync::mpsc::channel(100);
        let save_file_path = format!("{}/{}", temp_dir, "as.bin");
        tokio::spawn(async move {
            let _ = nustat_core::net::http::download_file_with_progress(asn_db_url, save_file_path, progress_tx).await;
        });
        while let Some(progress) = progress_rx.recv().await {
            println!("Downloaded: {:?}", progress);
        }

        // Download the latest country file      
        let country_db_url = nustat_core::db::ip::Country::get_github_url(&commit_hash);
        println!("Downloading country db file from: {}", country_db_url);
        let (progress_tx, mut progress_rx) = tokio::sync::mpsc::channel(100);
        let save_file_path = format!("{}/{}", temp_dir, "country.bin");
        tokio::spawn(async move {
            let _ = nustat_core::net::http::download_file_with_progress(country_db_url, save_file_path, progress_tx).await;
        });
        while let Some(progress) = progress_rx.recv().await {
            println!("Downloaded: {:?}", progress);
        }
    });
}
