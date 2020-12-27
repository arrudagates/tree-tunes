use audiotags::*;
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal, ExecutableCommand, Result,
};
use libmpv::{FileState, Mpv, events::*};
use math::round;
use std::ffi::OsString;
use std::io::stdout;
use walkdir::{DirEntry, WalkDir};

mod bstree;

fn is_flac(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".flac"))
        .unwrap_or(false)
}

fn rinsert(
    mut tree: bstree::BST<String>,
    mut names: Vec<(OsString, i32)>,
    paths: Vec<(i32, std::path::PathBuf)>,
) -> bstree::BST<String> {
    if names.len() > 0 {
        let middle = round::floor((names.len() / 2) as f64, -1);
        tree.insert(
            Tag::default()
                .read_from_path(paths[names[middle as usize].1 as usize].1.to_path_buf())
                .unwrap()
                .title()
                .unwrap_or(names[middle as usize].0.to_str().unwrap())
                .to_string()
                .to_lowercase()
                .replace(" ", "")
                .chars()
                .filter(|&c| "abcdefghijklmnopqrstuvwxyz1234567890".contains(c))
                .collect(),
            paths[names[middle as usize].1 as usize].1.to_path_buf(),
        );
        names.remove(middle as usize);
        let names2 = names.split_off(middle as usize);
        tree = rinsert(tree, names, paths.clone());
        tree = rinsert(tree, names2, paths.clone());
    }
    return tree;
}

fn main() -> Result<()> {
    let mut tree = bstree::BST::new();
    let mut names = vec![];
    let mut paths = vec![];

    let walker = WalkDir::new("/home/arruda/Music/").into_iter();

    let mut i = 0;

    for entry in walker.filter(|e| is_flac(&e.as_ref().unwrap())) {
        let file = entry.unwrap();
        names.push((file.file_name().to_os_string(), i));
        paths.push((i, file.path().to_path_buf()));
        i = i + 1;
    }
    names.sort();

    tree = rinsert(tree, names, paths);
    //println!("{:#?}", tree);


    let mpv = Mpv::new().unwrap();
    mpv.set_property("vo", "null").unwrap();
   let mut ev_ctx = mpv.create_event_context();
    ev_ctx.disable_deprecated_events().unwrap();



stdout().execute(crossterm::cursor::SavePosition).unwrap().execute(terminal::Clear(terminal::ClearType::All)).unwrap();

  crossbeam::scope(|scope| {
      scope.spawn(|_| {

          loop {
              stdout()
                .execute(crossterm::cursor::MoveTo(0,4)).unwrap()
                 .execute(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap()
                    .execute(SetForegroundColor(Color::Blue)).unwrap()
                .execute(crossterm::cursor::MoveTo(0, terminal::size().unwrap().1)).unwrap()
            .execute(SetForegroundColor(Color::Blue)).unwrap()
            .execute(Print("Comando: ")).unwrap()
            .execute(ResetColor).unwrap();

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();

        match buffer.trim().to_lowercase().as_str() {
            "play" => {
                stdout()
                    .execute(SetForegroundColor(Color::Blue)).unwrap()
                    .execute(Print("Que música quer ouvir? ")).unwrap()
                    .execute(ResetColor).unwrap();

                let mut buffer = String::new();
                std::io::stdin().read_line(&mut buffer).unwrap();
               
                match tree.find(buffer.trim().to_string().to_lowercase().replace(" ", "")) {
                    Some(x) => {
                        mpv.playlist_load_files(&[(
                            x.into_os_string().as_os_str().to_str().unwrap(),
                            FileState::AppendPlay,
                            None,
                        )])
                        .unwrap();
                    }
                    None => println!("Música não encontrada"),
                }
            },
            "stop" => {
                mpv.playlist_clear().unwrap();
                mpv.playlist_remove_current().unwrap();
            },
            "pause" => mpv.pause().unwrap(),
            "resume" => mpv.unpause().unwrap(),
            "next" => mpv.playlist_next_force().unwrap_or(()),
            "previous" => mpv.playlist_previous_weak().unwrap_or(()),
            "volume" =>{
                stdout()
                    .execute(SetForegroundColor(Color::Blue)).unwrap()
                    .execute(Print("Digite o volume ")).unwrap()
                    .execute(ResetColor).unwrap();

                let mut buffer = String::new();
                std::io::stdin().read_line(&mut buffer).unwrap();

               mpv.set_property("volume", buffer.trim().to_string().parse::<i64>().unwrap()).unwrap();
            },
            _ => (),
        }
    }
 });

                   scope.spawn(|_| loop {
                      let ev = ev_ctx.wait_event(600.).unwrap();

            match ev {
                Ok(Event::StartFile) | Ok(Event::PropertyChange{name: "pause", .. }) => {
                    stdout()
                        .execute(crossterm::cursor::MoveTo(0,1)).unwrap()
                       .execute(terminal::Clear(terminal::ClearType::CurrentLine)).unwrap()
                    .execute(SetForegroundColor(Color::Blue)).unwrap()
                    .execute(Print(format!("{}", &mpv.get_property::<String>("media-title").unwrap()))).unwrap()
                                                                                                       .execute(ResetColor).unwrap()
      .execute(crossterm::cursor::MoveTo(0, terminal::size().unwrap().1)).unwrap()
                       .execute(Print("Comando: ")).unwrap();

                },
              _ => (),
            }
                  
                   });
    }).unwrap();
Ok(())
}
