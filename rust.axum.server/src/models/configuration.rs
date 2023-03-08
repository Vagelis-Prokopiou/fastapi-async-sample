#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub max_connections: u32,
}

impl DatabaseSettings {
    pub fn new(
        username: String,
        password: String,
        port: u16,
        host: String,
        database_name: String,
        max_connections: u32,
    ) -> Self {
        return Self {
            username,
            password,
            port,
            host,
            database_name,
            max_connections,
        };
    }

    pub fn get_connection_string_without_db(&self) -> String {
        return format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        );
    }

    pub fn get_connection_string_with_db(&self) -> String {
        return format!(
            "{}/{}",
            self.get_connection_string_without_db(),
            self.database_name
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::models::configuration::DatabaseSettings;

    #[test]
    fn test_get_connection_string_without_db() {
        let db_settings = DatabaseSettings::new(
            "foo".to_owned(),
            "bar".to_owned(),
            1000,
            "baz".to_owned(),
            "hello".to_owned(),
            10,
        );
        assert_eq!(
            db_settings.get_connection_string_without_db(),
            "postgres://foo:bar@baz:1000"
        );
    }

    #[test]
    fn test_get_connection_string_with_db() {
        let db_settings = DatabaseSettings::new(
            "foo".to_owned(),
            "bar".to_owned(),
            1000,
            "baz".to_owned(),
            "hello".to_owned(),
            10,
        );
        assert_eq!(
            db_settings.get_connection_string_with_db(),
            "postgres://foo:bar@baz:1000/hello"
        );
    }
}
