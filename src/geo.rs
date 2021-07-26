use actix_web::{post,web,  HttpResponse};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use actix_web::web::{Data, Json, Path};

use diesel::result::Error;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use crate::{DBPool, DBPooledConnection};

use super::schema::geo_locations;



use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};

#[derive(Debug, Deserialize, Serialize)]
pub struct Geo {
    pub id: String,
    pub d: DateTime<Utc>,
    pub lat: f32,
    pub lon: f32,
    pub sat: i32,
    pub s: f32,
}

impl Geo {
    pub fn new(id: String, d: DateTime<Utc>, lat: f32, lon: f32, sat: i32, s: f32) -> Self {
        Self {
            id: id,
            d: d,
            lat: lat,
            lon: lon,
            sat: sat,
            s: s,
        }
    }
    
    pub fn to_db(&self) -> GeoDB {
        GeoDB {
            id: self.id.clone(),
            d: DateTime::naive_utc(&self.d),
            lat: self.lat,
            lon: self.lon,
            sat: self.sat,
            s: self.s,
        }
    }
}

#[table_name = "geo_locations"]
#[derive(Queryable, Insertable)]
pub struct GeoDB {
    pub id: String,
    pub d: NaiveDateTime,
    pub lat: f32,
    pub lon: f32,
    pub sat: i32,
    pub s: f32,
}

impl GeoDB {
    fn to_geo(&self) -> Geo {
        Geo {
            id: self.id.clone(),
            d: Utc.from_utc_datetime(&self.d),
            lat: self.lat,
            lon: self.lon,
            sat: self.sat,
            s: self.s,
        }
    }
}



#[post("/geo")]
pub async fn create(geo_req: Json<Geo>, pool: Data<DBPool>) -> HttpResponse {

    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let geo = web::block(move || create_geo(geo_req.to_db(), &conn)).await;

    match geo {
        Ok(geo) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(geo),
        _ => HttpResponse::NoContent().await.unwrap(),
    }
}


fn create_geo (geo: GeoDB, conn: &DBPooledConnection) -> Result<Geo, Error> {
    use crate::schema::geo_locations::dsl::*;
    let _ = diesel::insert_into(geo_locations).values(&geo).execute(conn);
    Ok(geo.to_geo())
}
