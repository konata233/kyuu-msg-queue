pub struct RuntimeContext {
    pub local_host: String,
    pub local_port: u16,
    pub hosts: Vec<String>,
}

impl RuntimeContext {
    pub fn new(local_host: String, local_port: u16) -> RuntimeContext {
        RuntimeContext {
            local_host,
            local_port,
            hosts: Vec::new(),
        }
    }
}
