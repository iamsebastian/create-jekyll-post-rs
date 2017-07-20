use std::io;
use std::ffi::OsString;
use std::fs;
use std::fs::{DirEntry, File};
use std::path::{Path,PathBuf};
use std::io::prelude::*;
extern crate rustc_serialize;
extern crate mustache;
extern crate time;

#[derive(RustcEncodable)]
struct Photo {
    caption: String,
    alt_text: String,
    full_image_path: String,
    image_path: String,
}

impl Photo {
    fn new(path: &PathBuf, file_name: &OsString) -> Photo {
        let path = path.to_str().unwrap().to_owned().replace("./", "/");
        let image_name = file_name.to_str().unwrap().to_owned().replace("./", "/");

        Photo {
            caption: "caption".to_owned(),
            alt_text: "photo".to_owned(),
            full_image_path: path,
            image_path: image_name,
        }
    }
}

#[derive(RustcEncodable)]
struct Post {
    categories: String,
    content: String,
    created: String,
    title: String,
    photos: Vec<Photo>,
}

impl Post {
    fn new() -> Post {
        Post {
            categories: "".to_owned(),
            content: String::new(),
            created: time::strftime("%Y-%m-%d %H:%M:%S +0200", &time::now()).unwrap(),
            title: "".to_owned(),
            photos: Vec::new(),
        }
    }

    fn get_file_name(&self) -> String {
        let lo_title = self.title.to_lowercase().replace(" ", "-");
        let today_date_str = time::strftime("%Y-%m-%d-", &time::now()).unwrap();

        format!("../_posts/{}{}.markdown", today_date_str, lo_title)
    }
}

fn look_for_photos(dir: &Path) -> Vec<Photo> {
    let mut images: Vec<Photo> = Vec::new();
    let mut img_filenames: Vec<String> = Vec::new();

    let is_dir: bool = fs::metadata(dir).unwrap().is_dir();

    if is_dir {
        println!("Is directory: {}", dir.to_str().unwrap());

        let dir_iter = fs::read_dir(dir).unwrap();

        for entry in dir_iter {
            let entry: DirEntry = entry.unwrap();
            let file_name = entry.file_name().into_string().unwrap();

            img_filenames.push(file_name);
            let photo = Photo::new(&entry.path(), &entry.file_name());
            images.push(photo);
        }
    }

    let imgs: String = img_filenames.into_iter().collect();

    println!("Found images: {}", imgs);
    //println!("Found images: {}", String::from_iter(img_filenames));

    images
}

fn print_hint(placeholder: &str) {
    print!("\nPlease input your new post {}: ", placeholder);

    // Force print out, because stdout often gets buffered by line.
    io::stdout().flush().unwrap();
}

fn init_post() -> Post {
    let mut cnt = 1;
    let stdin = io::stdin();
    let mut post = Post::new();
    post.photos = look_for_photos(Path::new("./img/"));

    print_hint("title");

    for line in stdin.lock().lines() {
        //print_hint("HINT!");
        let input: String = line.unwrap().trim().to_owned();

        if input != "" {
            match cnt {
                1 => {
                    post.title = input;
                    print_hint("categories");
                }
                2 => {
                    post.categories = input;
                    //print_hint("... just HIT ENTER to exit.");
                    break
                }
                3 => {
                    break
                }
                _ => panic!("I'm so holy. Gnarf."),
            }
            cnt += 1;
        } else {
            println!("Idiot!");
        }
    }

    post
}

fn handle_template(post: &Post) {
    let mut template_content: String = String::new();
    let template: mustache::Template;
    let mut post_template: File = match File::open("./template.mustache") {
        Err(y) => panic!("{:?}", y),
        Ok(f) => f,
    };

    println!("Found template.");

    if let Err(err) = post_template.read_to_string(&mut template_content) {
        panic!("{:?}", err);
    }

    println!("Will create new file: {:?}", post.get_file_name());

    let mut new_file: File = match File::create(post.get_file_name()) {
        Err(y) => panic!("{:?}", y),
        Ok(f) => f,
    };

    template = mustache::compile_str(&template_content).expect("Could not render template content");

    template.render(&mut new_file, post).unwrap();

    println!("Successfully created new blog post: {}", post.get_file_name());
}

fn main() {
    let post: Post = init_post();
    handle_template(&post);
}
