use actix_web::{post, HttpResponse};

use serde::{Serialize, Deserialize};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use actix_web::web::{Data, Json, Path};

use diesel::result::Error;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use crate::{DBPool, DBPooledConnection};

use super::schema::geo;
use diesel::query_dsl::methods::{FilterDsl, LimitDsl, OrderDsl};
use std::str::FromStr;


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
    /*
    pub fn to_db(&self) -> GeoDB {
        GeoDB {
            id: self.id.clone(),
            d: Utc.from_utc_datetime(&self.d),
            lat: self.lat,
            lon: self.lon,
            sat: self.sat,
            s: self.s,
        }
    }*/
}

#[table_name = "geo"]
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
pub async fn create(geo_req: Json<Geo>) -> HttpResponse {
    let geo = Geo {
        id: geo_req.id.clone(),
        d: Utc::now(),
        lat: geo_req.lat,
        lon: geo_req.lon,
        sat: geo_req.sat,
        s: geo_req.s,
    };
    HttpResponse::Created().json(geo)
}
