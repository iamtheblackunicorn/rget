/*
RGET by Alexander Abraham,
a.k.a. "Angeldust Duke" a.k.a. "The Black Unicorn".
Licensed under the MIT license.
*/

use tokio;
use std::io;
use cleasy::App;
use reqwest::get;
use std::fs::File;
use tokio::fs::write;
use colored::Colorize;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

/// Tries to create a file and returns
/// a boolean depending on whether the
/// operation succeeded.
fn create_file(filename: String) -> bool {
    let mut result: Vec<bool> = Vec::new();
    let new_file = File::create(filename);
    match new_file {
        Ok(_n) => result.push(true),
        Err(_x) => result.push(false)
    }
    return result[0];
}

/// Gets the last element from a string vector.
pub fn get_last_elem(subject: Vec<String>) -> String {
    let vec_len: usize = subject.len();
    let last_index: usize = vec_len - 1;
    return subject[last_index].clone();
}

/// Checks if a file can be downloaded from "url" and saved
/// to "target".
pub async fn can_download_file(url: String, target: String) -> bool {
    let mut result_one = false;
    let mut download_op = get(&url);
    match download_op.await {
        Ok(_x) => {
            result_one = true;
        },
        Err(_e) => {}
    };
    return result_one;
}

/// Downloads a file from "url" and saves
/// it to "target".
pub async fn download(url: String, target: String) {
    let url_clone_one: String = url.clone();
    let url_clone_two: String = url_clone_one.clone();
    let url_clone_three: String = url_clone_two.clone();
    let target_clone_one: String = target.clone();
    let target_clone_two: String = target_clone_one.clone();
    let target_clone_three: String = target_clone_two.clone();
    let target_clone_four: String = target_clone_three.clone();
    if can_download_file(url_clone_one, target_clone_one).await == true {
        let mut download_data = get(url_clone_two).await.unwrap().bytes().await.unwrap();
        create_file(target_clone_two);
        write(target_clone_four, download_data).await;
    }
    else {
        let err_msg: String = format!("Downloading \'{}\' from \'{}\' failed!", url_clone_three, target_clone_three);
        println!("{}", err_msg);
    }
}

// Returns a vector of strings from a character split for a string.
/// Both the string and split character have to be strings.
pub fn clean_split(subject: String, split_char: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for item in subject.split(&split_char) {
        let new_item: String = item.to_string();
        result.push(new_item);
    }
    return result;
}

/// RGET's command-line interface.
pub async fn cli(){
   let name: String = String::from("RGET");
   let version: String = String::from("1.0.0");
   let author: String = String::from("Alexander Abraham");
   let mut rget_cli: App = App::new(name, version, author);
   rget_cli.add_arg("download".to_string(), "Downloads a file from the supplied URL.".to_string(), "true".to_string());
   if rget_cli.version_is() == true {
       println!("{}", format!("{}",rget_cli.version()).cyan().to_string());
   }
   else if rget_cli.help_is() == true {
       println!("{}", format!("{}",rget_cli.help()).cyan().to_string());
   }
   else if rget_cli.arg_was_used("download".to_string()) == true {
       let url: String = rget_cli.get_arg_data("download".to_string());
       let url_clone_one: String = url.clone();
       let url_clone_two: String = url_clone_one.clone();
       let url_clone_three: String = url_clone_two.clone();
       let target: String = get_last_elem(clean_split(url_clone_one, String::from("/")));
       let target_clone_one: String = target.clone();
       let target_clone_two: String = target_clone_one.clone();
       let target_clone_three: String = target_clone_two.clone();
       let dl_msg: String = format!("Downloading \'{}\' from \'{}\'...", target_clone_one, url_clone_three).cyan().to_string();
       let dl_done_msg: String = format!("Downloading \'{}\' finished!", target_clone_three).cyan().to_string();
       let now: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
       println!("{}", dl_msg);
       download(url_clone_two, target_clone_two).await;
       let then: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
       let duration: u64 = then - now;
       let duration_message: String = format!("Operation completed in {:?} seconds.", duration).cyan().to_string();
       println!("{}", dl_done_msg);
       println!("{}", duration_message);

   }
   else {
       println!("{}", format!("{}",rget_cli.help()).red().to_string() );
   }
}
