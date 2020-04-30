
pub struct User {
    key: String,
    secret: String
}

impl User {
    pub fn new(key: String, secret: String) -> User {
        User {
            key: key,
            secret: secret,
        }
    }

    pub fn from_env() -> Result<User, Box<dyn std::error::Error>> {
        let key = std::env::var("APCA_API_KEY_ID")?;

        let secret = std::env::var("APCA_SECRET_API_KEY")?;

        Ok(User::new(key, secret))
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    pub fn get_secret(&self) -> String {
        self.secret.clone()
    }
}
