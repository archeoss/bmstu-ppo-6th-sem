// // Let it be for now
mod tests {
    use app::models::customs::inspector::Inspector;
    use app::models::customs::Customs;
    use app::models::declaration::{Declaration, Pending};
    use app::models::misc::location::Location;
    use app::models::participants::client;
    use app::models::participants::representative::Representative;
    use app::models::{declaration::Draft, participants::declarant::Declarant};
    use app::repository::surrealdb::SurrealRepo;
    use app::repository::Repository;
    use serde::{Deserialize, Serialize};
    use std::{error::Error, fs::File, io::Read};
    use surrealdb::{
        engine::remote::ws::{Client, Ws},
        opt::auth::Root,
        Surreal,
    };

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

    async fn connect() -> Surreal<Client> {
        let credentials = read_root_credential(CREDENTIALS_FILE).unwrap();
        let db = Surreal::new::<Ws>(format!("{}:{}", credentials.host, credentials.port))
            .await
            .unwrap();
        let scope = credentials.sc;
        let root = Root {
            username: &credentials.username,
            password: &credentials.password,
        };
        db.signin(root).await.unwrap();
        db.use_ns(credentials.ns)
            .use_db(credentials.db)
            .await
            .unwrap();

        db
    }

    #[tokio::test]
    async fn test_put_get() {
        let db = connect().await;
        let repository: SurrealRepo<Declarant> = SurrealRepo::new(db).unwrap();

        let decl = Declarant::new("Thomas").await;
        repository
            .save(decl.id().await, decl.clone())
            .await
            .unwrap();
        let declarant: Declarant = dbg!(repository.get(decl.id().await).await.unwrap());
        assert_eq!(declarant, decl);
    }

    #[tokio::test]
    async fn test_update() {
        let db = connect().await;
        let repository: SurrealRepo<Declarant> = SurrealRepo::new(db).unwrap();

        let mut decl = Declarant::new("Thomas").await;
        repository
            .save(decl.id().await, decl.clone())
            .await
            .unwrap();
        decl.set_name("Peter").await;
        repository
            .save(decl.id().await, decl.clone())
            .await
            .unwrap();
        let declarant: Declarant = dbg!(repository.get(decl.id().await).await.unwrap());
        assert_eq!(declarant, decl);
    }

    #[tokio::test]
    async fn test_delete() {
        let db = connect().await;
        let repository: SurrealRepo<Declarant> = SurrealRepo::new(db).unwrap();

        let decl = Declarant::new("Thomas").await;
        repository
            .save(decl.id().await, decl.clone())
            .await
            .unwrap();
        let declarant: Declarant = dbg!(repository.get(decl.id().await).await.unwrap());
        assert_eq!(declarant, decl);

        repository.delete(decl.id().await).await.unwrap();
        let declarant = dbg!(repository.get(decl.id().await).await);
        assert!(declarant.is_err());
    }

    #[tokio::test]
    async fn test_delete_all() {
        let db = connect().await;
        let repository: SurrealRepo<Declarant> = SurrealRepo::new(db).unwrap();

        let decl = Declarant::new("Thomas").await;
        repository
            .save(decl.id().await, decl.clone())
            .await
            .unwrap();
        let declarant: Declarant = dbg!(repository.get(decl.id().await).await.unwrap());
        assert_eq!(declarant, decl);

        repository.delete_all().await.unwrap();
        let declarant = dbg!(repository.get(decl.id().await).await);
        assert!(declarant.is_err());
    }

    #[tokio::test]
    async fn test_generics() {
        let db = connect().await;
        let repository1: SurrealRepo<Declarant> = SurrealRepo::new(db.clone()).unwrap();
        let repository2: SurrealRepo<Declaration<Draft>> = SurrealRepo::new(db.clone()).unwrap();
        let repository3: SurrealRepo<Declaration<Pending>> = SurrealRepo::new(db.clone()).unwrap();
        let repository4: SurrealRepo<Inspector> = SurrealRepo::new(db.clone()).unwrap();
        let repository5: SurrealRepo<Customs> = SurrealRepo::new(db.clone()).unwrap();
        let repository6: SurrealRepo<Location> = SurrealRepo::new(db.clone()).unwrap();
        let repository7: SurrealRepo<Representative> = SurrealRepo::new(db.clone()).unwrap();
        let repository8: SurrealRepo<client::Client> = SurrealRepo::new(db.clone()).unwrap();

        let decl = Declarant::new("Thomas").await;
        repository1
            .save(decl.id().await, decl.clone())
            .await
            .unwrap();
        let declarant: Declarant = dbg!(repository1.get(decl.id().await).await.unwrap());
        assert_eq!(declarant, decl);

        let mut decl = Declaration::<Draft>::new().await;
        // let decl.
        decl.set_sender_name("Test");
        repository2.save(decl.id().await, decl.clone()).await;
        let declaration: Declaration<Draft> = dbg!(repository2.get(decl.id().await).await.unwrap());
        assert_eq!(declaration, decl);
        //
        let mut decl: Declaration<Pending> = Declaration::new().await.into();
        decl.set_sender_name("Test");
        repository3
            .save(decl.id().await, decl.clone())
            .await
            .unwrap();
        let declaration: Declaration<Pending> =
            dbg!(repository3.get(decl.id().await).await.unwrap());
        assert_eq!(declaration, decl);

        let insp = Inspector::new("test", "test", "test").await;
        repository4
            .save(insp.id().await, insp.clone())
            .await
            .unwrap();
        let inspector: Inspector = dbg!(repository4.get(insp.id().await).await.unwrap());
        assert_eq!(inspector, insp);

        let customs = Customs::new("test", &Location::default()).await;
        repository5
            .save(customs.id().await, customs.clone())
            .await
            .unwrap();
        let cust: Customs = dbg!(repository5.get(customs.id().await).await.unwrap());
        assert_eq!(cust, customs);

        let mut location = Location::default();
        location.set_city("TestCity");
        repository6
            .save(location.id().await, location.clone())
            .await
            .unwrap();
        let loc: Location = dbg!(repository6.get(location.id().await).await.unwrap());
        assert_eq!(location, loc);

        let rep = Representative::new("test").await;
        repository7.save(rep.id().await, rep.clone()).await.unwrap();
        let repres: Representative = dbg!(repository7.get(rep.id().await).await.unwrap());
        assert_eq!(*repres.name_ref().await, *rep.name_ref().await);

        let client = client::Client::new("test").await;
        repository8
            .save(client.id().await, client.clone())
            .await
            .unwrap();
        let clnt: client::Client = dbg!(repository8.get(client.id().await).await.unwrap());
        assert_eq!(*clnt.name_ref().await, *client.name_ref().await);
    }
}
