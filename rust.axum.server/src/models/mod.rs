pub mod configuration;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub framework: String,
    pub first_name: String,
    pub last_name: String,
    pub birthday: Option<String>,
    pub email: String,
}

impl User {
    pub fn new(
        id: String,
        framework: String,
        first_name: String,
        last_name: String,
        birthday: Option<String>,
        email: String,
    ) -> Self {
        Self {
            id,
            framework,
            first_name,
            last_name,
            birthday,
            email,
        }
    }
}