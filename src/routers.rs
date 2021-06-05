use db::Conn as DbConn;
use rocket_contrib::json::Json;
use super::models::{Homework, NewHomework};
use serde_json::Value;

#[get("/homeworks", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let homeworks = Homework::all(&conn);

    Json(json!({
        "status": 201,
        "data": homeworks,
    }))
}

#[post("/homework", data = "<new_homework>")]
pub fn new(conn: DbConn, new_homework: Json<NewHomework>) -> Json<Value> {
    
    Json(json!({
        "status": Homework::insert(new_homework.into_inner(), &conn),
        "data": Homework::all(&conn).first(),
    }))
}

#[get("/homework/<id>", format = "aplication/json")]
pub fn show(conn: DbConn, id: i32) ->Json<Value> {
    let data = Homework::show(id, &conn);
    let status = if data.is_empty(){404} else { 201};
    Json(json!({
        "status": status,
        "data": data.get(0)
    }))
}

#[put("/homework/<id>", format = "application/json", data = "<homework>")]
pub fn update(conn: DbConn, id: i32, homework: Json<NewHomework>) -> Json<Value> {
    let status = if Homework::update_by_id(id, &conn, homework.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "data": Homework::all(&conn).first(),
    }))
}

#[delete("/homework/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<Value> {
    let status = if Homework::delete_by_id(id, &conn) {
        200
    } else {
        404
    };
    Json(json!({
        "status": status,
        "result": "Se elimino la tarea seleccionada",
    }))
}