use std::io;
use reqwest::get;
use std::fs::File;

async fn can_download_file(url: String, target: String) -> bool {
    let mut result_one = false;
    let mut result_two = false;
    let mut download_op = get(&url);
    match download_op.await {
        Ok(_x) => {
            result_one = true;
        },
        Err(_e) => {}
    };
    if result_one == true {
        let mut write_op = File::create(&target);
        match write_op {
            Ok(_y) => {
                result_two = true;
            },
            Err(_e) => {}
        };
    }
    else {}
    return result_two;
}

async fn download(url: String, target: String) {
    let url_clone_one: String = url.clone();
    let url_clone_two: String = url_clone_one.clone();
    let url_clone_three: String = url_clone_two.clone();
    let target_clone_one: String = target.clone();
    let target_clone_two: String = target_clone_one.clone();
    let target_clone_three: String = target_clone_two.clone();
    if can_download_file(url_clone_one, target_clone_one).await == true {
        let mut download_op = get(url_clone_two);
        let mut write_op = File::create(target_clone_two);

    }
    else {
        let err_msg: String = format!("Downloading \'{}\' from \'{}\' failed!", url_clone_three, target_clone_three);
        println!("{}", err_msg);
    }
}
fn main() {
    let url: String = String::from("https://sh.rustup.rs");
    let file_name: String = String::from("rustup-init.sh");
    download(url, file_name)
}
