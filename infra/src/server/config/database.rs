pub struct DatabaseConfig {
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub db_ssl_mode: String,
}

impl DatabaseConfig {
    pub fn load() -> Self {
        let db_host = dotenvy::var("DB_HOST")
            .expect("unable read sql host from env. Please check the environment.");
        let db_port = dotenvy::var("DB_PORT")
            .expect("unable read sql port from env. Please check the environment.")
            .parse::<u16>()
            .expect("unable to parse sql port from env. Please check the environment.");
        let db_name = dotenvy::var("DB_NAME")
            .expect("unable read sql name from env. Please check the environment.");
        let db_user = dotenvy::var("DB_USER")
            .expect("unable read sql user from env. Please check the environment.");
        let db_password = dotenvy::var("DB_PASSWORD")
            .expect("unable read sql password from env. Please check the environment.");
        let db_ssl_mode = dotenvy::var("DB_SSL_MODE")
            .expect("unable read sql ssl mode from env. Please check the environment.");
        Self {
            db_host: db_host,
            db_port: db_port,
            db_name: db_name,
            db_user: db_user,
            db_password: db_password,
            db_ssl_mode: db_ssl_mode,
        }
    }

    pub fn into_sql_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}?sslmode={}",
            self.db_user,
            self.db_password,
            self.db_host,
            self.db_port,
            self.db_name,
            self.db_ssl_mode
        )
    }
}
