use reqwest::blocking::get;
use reqwest::blocking::Client;
use reqwest::{Error};
use regex::Regex;
use std::fs;
use std::io;

pub fn streamable_downloader(url: &str)
{  
    //Extract website data from the supplied URL
    let website_data: String = match get_data(url){
        Ok(data) => data,
        Err(error) => {
            print!("Failure obtaining data from the given URL [ {} ]\n", error);
            return;
        }
    };

    //[0] = Video donwload URL | [1] = Video Title
    let string_bundle = find_info(website_data);


    match download_streamable(string_bundle)
    {
        Ok(_) => (),
        Err(e) => 
        {
            println!("Encountered an error while downloading the video from Streamable: {}", e);
        }
    }

}

fn get_data(url: &str) -> Result<String, Error> 
{
    let response = get(url)?;
    let data = response.text()?;

    Ok(data)
}

fn find_info(website_data: String) -> Vec<String> 
{
    //Regex lines for finding the video download URL and title of the video from the website data from Streamable.com 
    let video_regex = Regex::new("meta property=\"og:video:secure_url\" content=\"(.*?)\"").unwrap();
    let title_regex = Regex::new("meta property=\"og:title\" content=\"(.*?)\"").unwrap();

    //Extract these strings using the regex_capture function
    let video_string = regex_capture(&video_regex, &website_data);
    let title_string = regex_capture(&title_regex,&website_data);

    //Pack strings into Vector to be used later
    let output_strings = vec![video_string,title_string];
    
    //Return Vector of 2 strings
    output_strings
}

fn regex_capture(regex: &Regex, data: &str) -> String
{
    //Instantly return the output of the regex as a String, Default is an string of length 1
    regex
    .captures(data)
    .map(|captures| captures.get(1).unwrap().as_str().to_string())
    .unwrap_or_else(|| " ".to_string())
}

fn download_streamable(string_bundle: Vec<String>) -> io::Result<()>
{
    let client = Client::new();
    let mut response = client.get(string_bundle[0].as_str()).send()
    .map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;
    if response.status().is_success()
    {
        let mut donwload_file = fs::File::create(format!("{}.mp4",string_bundle[1].as_str()))?;

        std::io::copy(&mut response, &mut donwload_file)?;
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to download file : {}",
            response.status()),
        ))
    }
}