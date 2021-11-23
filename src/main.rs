use std::fs;
//use std::fs::read;

//use askama::Template;

//template structure is here


fn main() {
    println!("Your project's been generated!");
    create_project(); 
}

//this function creates a parent folder
//parent folder holds markdown and template subfolders
fn create_project() {

    let folder_path = "./Hello";
    assert!(fs::create_dir(folder_path).is_ok());

    create_md();
    create_template();

    // let hello = HomeTemplate {name: read("Hello/template/home.html") };
    // println!("{}", hello.render().unwrap());
}

//this function creates the markdown folder and file
fn create_md() {
    let folder_path = "Hello/markdown/";
    let file_path = "Hello/markdown/home.md";

    assert!(fs::create_dir(folder_path).is_ok());
    assert!(fs::write(file_path, b"# Hello, World!").is_ok());
  

}

//This function creates the template folder and an html file
fn create_template() {


    let folder_path = "Hello/template";
    let file_path = "Hello/template/home.html";
    let file_content = "Hello, {{name}}!"; 

    assert!(fs::create_dir(folder_path).is_ok());
    assert!(fs::write(file_path, file_content).is_ok());
}

// #[derive(Template)]
// #[template(path = "Hello/template/home.html")]

// struct HomeTemplate<'a> {

//     name: &'a str,
// }
