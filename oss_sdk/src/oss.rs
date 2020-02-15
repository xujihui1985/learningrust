pub struct OSS {
    access_key_id: String,
    access_key_secret: String,
    endpoint: String,
}

impl OSS {
    pub fn get_access_key_secret(&self) -> &str {
        self.access_key_secret.as_str()
    }

    pub fn get_access_key_id(&self) -> &str {
        self.access_key_id.as_str()
    }

    pub fn get_endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
}

impl OSS {
    pub fn new(endpoint: &str, access_key_id: &str, access_key_secret: &str) -> Self {
        OSS {
            access_key_id: access_key_id.to_string(),
            access_key_secret: access_key_secret.to_string(),
            endpoint: endpoint.to_string(),
        }
    }
}
