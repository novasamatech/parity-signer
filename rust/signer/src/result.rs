pub fn return_json_array(mut string: String) -> std::result::Result<String, Box<dyn std::error::Error>> {
    match string.pop() {
            None | Some('[') => return Ok("[]".to_string()),
            Some(',') => {
                string.push_str("]");
                return Ok(string);
            }
            _ => return Err(Box::from("Database corrupted!"))
        }

}
