mod streamable;
//mod screens 
//{
//    pub mod main_screen;
//}

fn main() {
    let url = "https://streamable.com/7ws82";
    streamable::streamable_downloader(url);
    
}
