struct DatabaseConnection {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
    }
    struct DatabaseConnectionBuilder {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
    }
    impl DatabaseConnectionBuilder {
    fn new(host: &str) -> Self {
    DatabaseConnectionBuilder {
    host: host.to_string(),
    port: 3306,
    username: String::new(),
    password: String::new(),
    database: String::new()
    }
    }
    fn port(mut self, port:u16) -> Self {
    self.port = port;
    self
    }
    fn username(mut self, username: &str) -> Self {
    self.username = username.to_string();
    self
    }

    fn password(mut self, password: &str) -> Self {
        self.password = password.to_string();
        self
        }
        fn database(mut self, database: &str) -> Self {
        self.database = database.to_string();
        self
        }
        fn build(self) -> DatabaseConnection {
        DatabaseConnection {
        host: self.host,
        port: self.port,
        username: self.username,
        password: self.password,
        database: self.database,
        }
        }
        }
        fn main() {
        let connection = DatabaseConnectionBuilder::new("localhost")
        .port(5432)
        .username("master_user123")
        .password("secretpassword")
        .database("masteringbackenddb")
        .build();
        println!("Database Connection: ");
        println!("Host: {}", connection.host);
        println!("Port: {}", connection.port);
        println!("Username: {}", connection.username);
        println!("Password: {}", connection.password);
        println!("Database: {}", connection.database);
        }
        