use std::fs;


fn main() {
    println!("Here's your project!");
    create_project(); 
}

//this function creates a parent folder
//parent folder holds markdown and template subfolders
fn create_project() {

    let folder_path = "./Hello";

    create_md();
    create_template();
}

//this function creates the markdown folder and file
fn create_md() {
    let folder_path = "Hello/markdown";
    let file_path = "Hello/markdown/home.md";

    fs::create_dir(folder_path);
    fs::write(file_path, b"# Hello, World!");
  

}

//This function creates the template fold and an html file
fn create_template() {

    let folder_path = "Hello/template";
    let file_path = "Hello/template/home.html";

    fs::create_dir(folder_path, );
    fs::write(file_path);
}
