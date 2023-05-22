use std::{error::Error, fs::File, io::Read};

use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

const CREDENTIALS_FILE: &str = "./build/credentials.json";

#[derive(Serialize, Deserialize)]
struct Credentials {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub ns: String,
    pub db: String,
    pub sc: String,
}

fn read_root_credential(filename: &str) -> Result<Credentials, Box<dyn Error>> {
    let credential: Credentials = serde_json::from_str(&read_file(filename)?)?;

    Ok(credential)
}

fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub async fn create_db() -> Result<(), Box<dyn Error>> {
    let credentials = read_root_credential(CREDENTIALS_FILE)?;
    let db = Surreal::new::<Ws>(format!("{}:{}", credentials.host, credentials.port)).await?;
    let scope = credentials.sc;
    let root = Root {
        username: &credentials.username,
        password: &credentials.password,
    };
    db.signin(root).await?;
    db.use_ns(credentials.ns).use_db(credentials.db).await?;

    // read init sql file
    let sql = read_file("./build/init.sql")?;
    db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE FIELD id ON customs TYPE uuid UNIQUE"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE INDEX idx_customs ON customs COLUMNS id UNIQUE"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE TABLE inspector SCHEMAFULL
    //         PERMISSIONS
    //             FOR select WHERE true,
    //             FOR create, delete, update WHERE id = $auth.id AND $auth.role containsany [role:inspector]"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //

    //
    // let sql = r#"
    //     DEFINE FIELD name ON customs TYPE string"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE FIELD address ON customs TYPE string"#;
    //
    // let sql = r#"
    //     DEFINE FIELD name ON interrogator TYPE string"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE FIELD age ON interrogator TYPE int"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE FIELD gender ON interrogator TYPE string"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE FIELD nationality ON interrogator TYPE string"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE TABLE interrogator SCHEMAFULL
    //         PERMISSIONS
    //             FOR select WHERE true,
    //             FOR create, delete, update WHERE id = $auth.id AND $auth.role containsany [role:interrogator]"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE FIELD name ON interrogator TYPE string"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE FIELD age ON interrogator TYPE int"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE FIELD gender ON interrogator TYPE string"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE FIELD nationality ON interrogator TYPE string"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE TABLE verdict SCHEMAFULL
    //         PERMISSIONS
    //             FOR select WHERE true,
    //             FOR create, delete, update WHERE id = $auth.id AND $auth.role containsany [role:interrogator]"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE FIELD correct ON verdict TYPE bool"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE TABLE session SCHEMAFULL
    //         PERMISSIONS
    //             FOR select WHERE true,
    //             FOR create, delete, update WHERE id = $auth.id AND $auth.role containsany [role:interrogator]"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE FIELD time_frame ON session TYPE datetime"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE FIELD time_spent ON session TYPE datetime"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE TABLE question SCHEMAFULL
    //         PERMISSIONS
    //             FOR select WHERE true,
    //             FOR create, delete, update WHERE id = $auth.id AND $auth.role containsany [role:interrogator]"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE FIELD text ON question TYPE string"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE TABLE computer SCHEMAFULL
    //         PERMISSIONS
    //             FOR select WHERE true,
    //             FOR create, delete, update WHERE id = $auth.id AND $auth.role containsany [role:computer]"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE FIELD model ON computer TYPE string"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE FIELD developed_by ON computer TYPE string"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE TABLE answer SCHEMAFULL
    //         PERMISSIONS
    //             FOR select WHERE true,
    //             FOR create, delete, update WHERE id = $auth.id AND $auth.role containsany [role:computer, role:human]"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     DEFINE FIELD text ON answer TYPE string"#;
    // db.query(sql).await?;
    //
    // let sql = r#"
    //     INFO FOR DB"#;
    // println!("{:?}", db.query(sql).await?);

    Ok(())
}
