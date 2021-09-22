pub fn return_json_array(mut string: String) -> anyhow::Result<String, anyhow::Error> {
    match string.pop() {
            None | Some('[') => return Ok("[]".to_string()),
            Some(',') => {
                string.push_str("]");
                return Ok(string);
            }
            _ => return Err(anyhow::anyhow!("Database corrupted!"))
        }

}
