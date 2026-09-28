use serde::{Deserialize, Serialize};
use mongodb::{bson::oid::ObjectId, bson::doc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Auteur {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    prenom: String,
    nom: String,
    email: String
}

impl Auteur {
    pub fn new(prenom: String, nom: String, email: String) -> Self {
        Auteur { id: None, prenom, nom, email }
    }
}
