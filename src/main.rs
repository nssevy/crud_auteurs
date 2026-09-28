mod models;
mod repositories;
use crate::models::Auteur;
use crate::repositories::auteur_repository::AuteurRepository;
use mongodb::{Client, Collection, bson::doc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "mongodb://localhost:27017";
    let client = Client::with_uri_str(url).await?;
    let db = client.database("apprentissage");

    let repo = AuteurRepository::new(&db);

    let auteur_collection: Collection<Auteur> = db.collection("authors");

    let auteur_sans_id = Auteur::new(
        "Guillaume".into(),
        "Gomez".into(),
        "guillaumegomez@gmail.com".into(),
    );

    repo.creer(auteur_sans_id)

    println!("Inséré avec l'_id : {:?}", resultat_insertion.inserted_id);
    match auteur_collection
        .find_one(doc! {"prenom": "Guillaume"})
        .await?
    {
        Some(auteur) => println!("Relu {:?}", auteur),
        None => println!("Utilisateur introuvable"),
    }

    Ok(())
}

/*
Définis une struct Auteur avec un champ id de type Option<ObjectId>, plus prenom, nom et email.

Règle serde pour que :

le champ id corresponde au champ _id du document ;
il ne soit pas envoyé quand il vaut None.

Dans main, connecte-toi à la base apprentissage, crée un auteur sans id, insère-l
dans authors, puis affiche l'_id renvoyé par l'insertion.

Relis ensuite cet auteur par son prénom, et affiche-le entièrement.

Résultat attendu :
   Inséré avec l'_id : ObjectId("6ab...")
   Relu : Auteur { id: Some(ObjectId("6ab...")), prenom: "George", nom: "Sand", email: "..." }
*/
