#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::jobs;
use crate::models::jobs::ActiveModel;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub employer_admin_id: i32,
    pub industry_id: i32,
    pub title: String,
    pub description: String,
    pub demand: i32,
    pub start_time: DateTimeWithTimeZone,
    pub end_time: DateTimeWithTimeZone,
    pub hourly_rate: Decimal,
    pub required_gender: i16,
    pub required_ages: Vec<i32>,
    pub required_languages: Vec<String>,
    pub required_nationalities: Vec<String>,
    pub status: i16,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.employer_admin_id = Set(self.employer_admin_id);
        item.industry_id = Set(self.industry_id);
        item.title = Set(self.title.clone());
        item.description = Set(self.description.clone());
        item.demand = Set(self.demand);
        item.start_time = Set(self.start_time);
        item.end_time = Set(self.end_time);
        item.hourly_rate = Set(self.hourly_rate);
        item.required_gender = Set(self.required_gender);
        item.required_ages = Set(Value::from(self.required_ages.clone()));
        item.required_languages = Set(Value::from(self.required_languages.clone()));
        item.required_nationalities = Set(Value::from(self.required_nationalities.clone()));
        item.status = Set(self.status);
    }
}

pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let mut item: ActiveModel = Default::default();
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;

    format::json(item)
}

pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let res = jobs::Entity::find().all(&ctx.db).await?;
    format::json(res)
}


pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/jobs")
        .add("/", post(add))
        .add("/", get(list))
}
