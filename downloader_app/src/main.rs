mod websites
{
    pub mod streamable;
    pub mod youtube;
}
//mod screens 
//{
//    pub mod main_screen;
//}

fn main() {
    let url = "https://streamable.com/7ws82";
    websites::streamable::streamable_downloader(url);
    
}
