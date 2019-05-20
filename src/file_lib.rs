struct file_result {
    pub relative_path: String,
    pub result: Result<i32, &str>
}

pub fn get_path(command_input: String) -> file_result {   
    return file_result {
        relative_path: "/",
        result: Err("File not found")
    };
}

fn file_exists(file_input: String) {

}

fn file_executable(file_input: String) {

}

fn relative_path(file_input: String) {

}
