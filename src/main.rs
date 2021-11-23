use std::fs;
use std::path::Path;

extern crate markdown;

fn main() {

    new_project_prompts();
    create_project(); 
    println!("Your project's been generated!\n");
    
}

//asks questions about the project
fn new_project_prompts () {

    let mut project_name = String::new();
    println!("Welcome to Tzalan! a Static Site Generator.\n\nWhat is your project name? ");
    let _project = std::io::stdin().read_line(&mut project_name).unwrap();
    println!("\nCreating a project named {}", project_name);
    


}

//this function creates a parent folder
//parent folder holds markdown and template subfolders
fn create_project() {

    let folder_path = "./Hello";
    assert!(fs::create_dir(folder_path).is_ok());

    create_md();
    create_template();


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
    let md_path = Path::new("Hello/markdown/home.md");
    let md = markdown::file_to_html(md_path);
    let file_content : String = String::from(md.unwrap()); 

    assert!(fs::create_dir(folder_path).is_ok());
    assert!(fs::write(file_path, file_content).is_ok());
}