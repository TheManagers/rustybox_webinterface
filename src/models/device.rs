use bson;
use wither;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Device {
    /// The device's unique ID.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub appname: String,
    pub name: String,
    pub description: String,
    pub active: bool
}

impl<'a> wither::Model<'a> for Device {

    /// The name of this model's collection.
    const COLLECTION_NAME: &'static str = "devices";

    /// Implement the getter for the ID of a model instance.
    fn id(&self) -> Option<bson::oid::ObjectId> {
        return self.id.clone();
    }

    /// Implement the setter for the ID of a model instance.
    fn set_id(&mut self, oid: bson::oid::ObjectId) {
        self.id = Some(oid);
    }

    // TODO: Index richtig einstellen
    /*
    fn indexes() -> Vec<IndexModel> {
        return vec![
            IndexModel{
                keys: doc!{"appname" => 1, "name" => 2, "id" => 3},
                options: wither::basic_index_options("unique-device", true, Some(true), None, None),
            },
        ];
    }
    */
}