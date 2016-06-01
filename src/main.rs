use std::io;
use std::fs::File;
use std::io::prelude::*;
extern crate rustc_serialize;
extern crate mustache;
extern crate time;

#[derive(RustcEncodable)]
struct Post {
    categories: String,
    content: String,
    created: String,
    title: String,
}

impl Post {
    fn new() -> Post {
        Post {
            categories: "".to_owned(),
            content: String::new(),
            created: time::strftime("%Y-%m-%d %H:%M:%S +0100", &time::now()).unwrap(),
            title: "".to_owned(),
        }
    }

    fn get_file_name(&self) -> String {
        let lo_title = self.title.to_lowercase().replace(" ", "-");
        let today_date_str = time::strftime("%Y-%m-%d-", &time::now()).unwrap();

        format!("../_posts/{}{}.markdown", today_date_str, lo_title)
    }
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

    let mut new_file: File = match File::create(post.get_file_name()) {
        Err(y) => panic!("{:?}", y),
        Ok(f) => f,
    };

    template = mustache::compile_str(&template_content);

    template.render(&mut new_file, post).unwrap();

    println!("Successfully created new blog post: {}", post.get_file_name());
}

fn main() {
    let post: Post = init_post();
    handle_template(&post);
}
