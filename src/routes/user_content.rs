use actix_web::{
    get, post, patch, delete,
    HttpResponse, web::{Path, Json, Data}
};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use crate::routes::models::{
    user_content::{Link, LinkRequest, LinkEditRequest}
};


#[get("/links/{user_id}")]
pub async fn get_links(db: Data<PgPool>, path: Path<Uuid>) -> HttpResponse {
    let user_id = path.into_inner();

    let user_links_res = sqlx::query_as!(
        Link,
        r#"SELECT * FROM "link" WHERE (user_id = $1)"#,
        user_id
    ).fetch_all(&**db).await;

    match user_links_res {
        Ok(links) => {
            return HttpResponse::Ok().json(
                json!({
                    "links": links
                })
            )
        },
        Err(_) => return HttpResponse::NotFound().finish()
    };
}

#[post("/link")]
pub async fn create_link(db: Data<PgPool>, link: Json<LinkRequest>) -> HttpResponse {
    let link_count = sqlx::query!(
        r#"SELECT COUNT(id) AS count FROM "link" WHERE (user_id = $1)"#,
        link.user_id.clone()
    ).fetch_one(&**db).await.unwrap().count.unwrap();


    let create_link = sqlx::query!(
        r#"
        INSERT INTO "link" (id, user_id, "order", label, link, is_nsfw)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        Uuid::new_v4(), link.user_id, i32::try_from(link_count).unwrap() + 1, link.label, link.link, link.is_nsfw
    ).execute(&**db).await;

    match create_link {
        Ok(_) => return HttpResponse::Ok().finish(),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[patch("/link/{link_id}")]
pub async fn edit_link(db: Data<PgPool>, path: Path<Uuid>, link_req: Json<LinkEditRequest>) -> HttpResponse {
    let link_id = path.into_inner();

    if link_req.order.is_some() {
        let query = sqlx::query!(
            r#"
            UPDATE "link"
            SET "order" = $2
            WHERE id = $1
            "#, link_id.clone(), link_req.order
        ).execute(&**db).await;

        if query.is_err() {
            return HttpResponse::InternalServerError().body(
                query.err().unwrap().to_string()
            )
        }
    }

    if link_req.label.is_some() {
        let query = sqlx::query!(
            r#"
            UPDATE "link"
            SET "label" = $2
            WHERE id = $1
            "#, link_id.clone(), link_req.label
        ).execute(&**db).await;

        if query.is_err() {
            return HttpResponse::InternalServerError().body(
                query.err().unwrap().to_string()
            )
        }
    }

    if link_req.link.is_some() {
        let query = sqlx::query!(
            r#"
            UPDATE "link"
            SET "link" = $2
            WHERE id = $1
            "#, link_id.clone(), link_req.link
        ).execute(&**db).await;

        if query.is_err() {
            return HttpResponse::InternalServerError().body(
                query.err().unwrap().to_string()
            )
        }
    }

    if link_req.is_nsfw.is_some() {
        let query = sqlx::query!(
            r#"
            UPDATE "link"
            SET "is_nsfw" = $2
            WHERE id = $1
            "#, link_id.clone(), link_req.is_nsfw
        ).execute(&**db).await;

        if query.is_err() {
            return HttpResponse::InternalServerError().body(
                query.err().unwrap().to_string()
            )
        }
    }

    HttpResponse::Ok().finish()
}

#[delete("/link/{link_id}")]
pub async fn delete_link(db: Data<PgPool>, path: Path<Uuid>) -> HttpResponse {
    let link_id = path.into_inner();

    let delete_link = sqlx::query!(
        r#"
        DELETE FROM "link"
        WHERE (id = $1)
        "#, link_id
    ).execute(&**db).await;

    match delete_link {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

