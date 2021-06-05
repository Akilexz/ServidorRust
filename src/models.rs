use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::schema::homeworks;
use crate::schema::homeworks::dsl::homeworks as all_homeworks;

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Homework {
    pub id: i32,
    pub title: String,
    pub published: bool,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "homeworks"]
pub struct NewHomework {
    pub title: String,
    pub published: bool,
}

impl Homework {
    pub fn show(id: i32, conn: &PgConnection) -> Vec<Homework> {
        all_homeworks
            .find(id)
            .load::<Homework>(conn)
            .expect("Error Loading homework")
    }

    pub fn all(conn: &PgConnection) -> Vec<Homework> {
        all_homeworks
            .order(homeworks::id.desc())
            .load::<Homework>(conn)
            .expect("Error loading the homeworks")
    }

    pub fn update_by_id(id: i32, conn: &PgConnection, homework: NewHomework) -> bool {
        use crate::schema::homeworks::dsl::{published as p, title as t};
        let NewHomework {
            title,
            published,
        } = homework;
        
        diesel::update(all_homeworks.find(id))
            .set((p.eq(published), t.eq(title)))
            .get_result::<Homework>(conn)
            .is_ok()
    }

    pub fn insert(homework: NewHomework, conn: &PgConnection) -> bool {
        diesel::insert_into(homeworks::table)
            .values(&homework)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
        if Homework::show(id, conn).is_empty() {
            return false;
        };
        diesel::delete(all_homeworks.find(id)).execute(conn).is_ok()
    }

    pub fn all_by_title(title: String, conn: &PgConnection) -> Vec<Homework>{
        all_homeworks
            .filter(homeworks::title.eq(title))
            .load::<Homework>(conn)
            .expect("Error loading homeworks by title")
    }
}