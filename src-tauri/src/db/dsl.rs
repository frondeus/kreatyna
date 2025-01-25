use diesel::{dsl::{AsSelect, Select}, prelude::*, sqlite::Sqlite};

use super::{models::Document, schema::documents};

type All = Select<documents::table, AsSelect<Document, Sqlite>>;

impl Document {
    pub fn all() -> All {
        documents::table.select(Document::as_select())
    }

    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn by_kind(kind: String) -> _ {
        let all: All = Document::all();
        all.filter(documents::kind.eq(kind))
    }
}