use crate::models::Auteur;
use mongodb::{Collection, Database, bson::oid::ObjectId};

pub struct AuteurRepository {
    collection: Collection<Auteur>,
}

impl AuteurRepository {
    //un constructeur qui reçoit une référence vers la Database et prépare la collection ;
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("authors"),
        }
    }

    //une méthode creer qui reçoit un auteur, l'insère, et renvoie son ObjectId ;
    pub async fn creer(&self, auteur: &Auteur) -> mongodb::error::Result<ObjectId> {
        let resultat = self.collection.insert_one(auteur).await?;
        let id_generer = resultat.inserted_id;
        match id_generer.as_object_id() {
            Some(a) => return Ok(a),
            None => panic!("l'id est sensé etre un "),
        }
    }
}
