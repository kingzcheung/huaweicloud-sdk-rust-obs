use crate::{client::Client, object::ObjectTrait, error::ObsError};

pub struct Bucket<'a> {
    name: &'a str,
    client: &'a Client,
}

impl<'a> Bucket<'a> {
    pub fn new(name: &'a str, client: &'a Client) -> Self { Self { name, client } }

    pub async fn put_object(
        &self,
        key: &str,
        object: &'static [u8],
    ) -> Result<(), ObsError> {
        self.client.put_object(self.name, key, object).await
    }

    pub async fn copy_object(&self, src: &str, dest: &str) -> Result<(), ObsError> {
        self.client.copy_object(self.name, src, dest).await
    }
}

