use anyhow::Result;
pub trait FromDb: Sized {
    type DbType;
    fn from_db(db: Self::DbType) -> Result<Self>;
}

pub trait ToDb {
    type DbType;
    fn to_db(&self) -> Result<Self::DbType>;
}

mod document {
    use super::{FromDb, Result, ToDb};
    use crate::ui::models::{self};
    use crate::db::models as db_models;


    impl ToDb for models::NewDocument {
        type DbType = db_models::NewDocument;
        
        fn to_db(&self) -> Result<Self::DbType> {
            let kind = serde_json::to_string(&self.kind)?;
            let just_kind: &'static str = models::JustDocumentKind::from(&self.kind).into();
            Ok(db_models::NewDocument {
                title: self.title.as_ref().to_string(),
                kind: just_kind.to_string(),
                body: kind,
            })
        }
    }
    
    impl FromDb for models::Document {
        type DbType = db_models::Document;

        fn from_db(db: Self::DbType) -> Result<Self> {
            let kind: models::DocumentKind = serde_json::from_str(&db.body)?;
            Ok(Self { id: db.id.into(), title: db.title.into(), kind })
        }
    }
}
