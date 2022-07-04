pub fn load_debug_env() {
    if cfg!(debug_assertions) {
        std::fs::read_to_string("./debug.env")
            .unwrap()
            .split('\n')
            .for_each(|it| {
                let kv = if it.trim().is_empty() || it.starts_with('#') {
                    None
                } else {
                    it.split_once('=')
                };
                if let Some((key, value)) = kv {
                    std::env::set_var(key, value);
                }
            })
    }
}
