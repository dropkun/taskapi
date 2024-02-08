use bson::oid::ObjectId;
use futures::StreamExt;
use mongodb::{bson::doc, Collection};

use serde::{Deserialize, Serialize, Serializer};

fn serialize_option_object_id_as_hex_string<S>(
    value: &Option<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(object_id) => {
            bson::serde_helpers::serialize_object_id_as_hex_string(object_id, serializer)
        }
        None => serializer.serialize_none(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_option_object_id_as_hex_string"
    )]
    pub id: Option<ObjectId>,
    pub name: String,
    pub description: Option<String>,
}

pub async fn create_task(collection: &Collection<Task>, task: Task) -> mongodb::error::Result<()> {
    collection.insert_one(task, None).await?;
    Ok(())
}

pub async fn delete_task(
    collection: &Collection<Task>,
    id: ObjectId,
) -> mongodb::error::Result<()> {
    collection
        .delete_one(
            doc! {
                "_id": id
            },
            None,
        )
        .await?;
    Ok(())
}

pub async fn get_task(collection: &Collection<Task>, id: ObjectId) -> mongodb::error::Result<Task> {
    let task = collection.find_one(doc! { "_id": id }, None).await?;
    match task {
        Some(task) => Ok(task),
        None => Err(mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Task not found",
        ))),
    }
}

pub async fn get_all_task(collection: &Collection<Task>) -> mongodb::error::Result<Vec<Task>> {
    let mut cursor = collection.find(None, None).await?;
    let mut tasks = Vec::new();
    while let Some(task) = cursor.next().await {
        tasks.push(task?);
    }
    Ok(tasks)
}
