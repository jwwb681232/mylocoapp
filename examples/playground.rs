#[allow(unused_imports)]
use loco_rs::{cli::playground, prelude::*};
use mylocoapp::{app::App,models::_entities::articles};

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    let ctx = playground::<App>().await?;

    // let active_model: articles::ActiveModel = articles::ActiveModel {
    //     title: Set(Some("how to build apps in 3 steps".to_string())),
    //     content: Set(Some("use Loco: https://loco.rs".to_string())),
    //     ..Default::default()
    // };
    // active_model.insert(&ctx.db).await.unwrap();

    // let res = articles::Entity::find().all(&ctx.db).await.unwrap();
    // println!("{:?}", res);
    // println!("welcome to playground. edit me at `examples/playground.rs`");

    let active_model:articles::ActiveModel = articles::ActiveModel{
        title: Set(Some("how to build apps in 3 steps".to_string())),
        content: Set(Some("use Loco: https://loco.rs".to_string())),
        ..Default::default()
    };
    active_model.insert(&ctx.db).await?;

    let res = articles::Entity::find().all(&ctx.db).await?;
    println!("{:#?}", res);

    Ok(())
}
