use libmpv::{FileState, Mpv};
use std::ffi::OsString;
use math::round;
use walkdir::{DirEntry, WalkDir};

use std::io::{stdout, Write};
use std::io::{self, Read};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
    event::read, terminal
};

mod bstree;

fn is_flac(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".flac"))
        .unwrap_or(false)
}

fn main() -> Result<()> {
    let mut tree = bstree::BST::new();
    let mut names = vec![];
    let mut paths = vec![];

    let walker = WalkDir::new("/home/arruda/Music/AJR").into_iter();

    let mut i = 0;

    for entry in walker.filter(|e| is_flac(&e.as_ref().unwrap())) {
        let file = entry.unwrap();
        names.push((file.file_name().to_os_string(), i));
        paths.push((i,file.path().to_path_buf()));
        i = i + 1;
        //tree.insert(file.file_name().to_os_string(), file.path().to_path_buf());
    }
    names.sort();
    i = 0;
    while i < names.len(){
       let middle = round::floor((names.len() / 2) as f64, -1);
        tree.insert(names[middle as usize].0.to_os_string(), paths[names[middle as usize].1].1.to_path_buf());
        names.remove(middle as usize);
     //   i = i + 1;
    }
    //let middle = round::floor((names.len() / 2) as f64, -1);
   // println!("{:#?}", paths[names[middle as usize].1].1);
   //println!("{:#?}", tree);

    let mpv = Mpv::new().unwrap();

loop{

    stdout()
       .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(SetForegroundColor(Color::Blue))?
        .execute(Print("Que música quer ouvir?"))?
        .execute(ResetColor)?;

let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
std::io::stdin().read_line(&mut buffer).unwrap();
println!("{}", buffer);

   mpv.playlist_load_files(&[(
       tree.find(OsString::from(buffer.trim()))
           .unwrap()
           .into_os_string()
           .as_os_str()
           .to_str()
           .unwrap(),
       FileState::AppendPlay,
       None,)])
   .unwrap();

}
    

    Ok(())
}
