use std::ops::Bound;


use chrono::Days;
#[allow(unused_imports)]
use loco_rs::{cli::playground, prelude::*};


use migration::Query;
use sea_orm::FromQueryResult;
use sea_orm::QuerySelect;

use sea_orm::StatementBuilder;
use sea_orm::Value;
use sea_orm_tstzrange::TstzRange;
use serde::Deserialize;
use serde::Serialize;
use temp_range::{app::App, models::_entities::reservation};

use sea_orm::sea_query::Alias;

use sea_orm::sea_query::Expr;
use sea_orm::sea_query::SimpleExpr;
use chrono::{DateTime, Utc};

#[derive(FromQueryResult, Debug)]
pub struct RangeModel {
    pub id: i32,
    pub timespan: Option<TstzRange>,
}

pub fn create_range_expr(start: DateTime<Utc>, end: DateTime<Utc>) -> SimpleExpr {
    let range = TstzRange::from_datetime_pair(start, end);
    SimpleExpr::Value(Value::from(range))
}

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    let _ctx = playground::<App>().await?;
    let builder = _ctx.db.get_database_backend();
    let start = Utc::now();
    let end = start + chrono::Duration::days(1);



    let range = TstzRange::new(
        Bound::Included(start),
        Bound::Excluded(end),
    );
    // let query = Query::select()
    // .expr(range.0.to_string().cast_as(Alias::new("string")))
    // .to_owned();
    // let x = query.to_string(PostgresQueryBuilder);

    // let x = range.clone().0;
    // let foo = Expr::val(x.to_string());
    // let m = Expr::cust_with_expr("$1::TSTZRANGE", SimpleExpr::Value(x.to_string().into()));
    // let v = Value::from(range.clone()).cast_as(Alias::new("TSTZRANGE"));
    // // let thing = Expr::cust_with_expr("$1::TSTZRANGE", SimpleExpr::Value(v));
    // // let stmt = Statement::from_sql_and_values(
    // //     DatabaseBackend::Postgres,
    // //     "INSERT INTO reservation (timespan) VALUES ($1) RETURNING id",
    // // );
    // // let result = _ctx.db.execute(stmt).await?;
    // // println!("{:?}", result);
    // let query = Query::select().expr(v).to_owned();
    // let query_string = query.clone().to_string(PostgresQueryBuilder);
    // println!("{}", query_string);
    let range_string = range.0.to_string();

    // Build insert query with explicit casting
    // let insert = Query::insert()
    //     .into_table(reservation::Entity)
    //     .columns([reservation::Column::Timespan])
    //     .values_panic([Expr::cust_with_expr("$1::TSTZRANGE", SimpleExpr::Value(range_string.into()))])
    //     .to_owned();

    // let builder = _ctx.db.get_database_backend();
    // let stmt = builder.build(&insert);
    // let result = _ctx.db.execute(stmt).await?;
    // println!("Inserted reservation with last insert ID: {:?}", result);

    // active_model.insert(&_ctx.db).await.unwrap();
    // let active_model: articles::ActiveModel = articles::ActiveModel {
    //     title: Set(Some("how to build apps in 3 steps".to_string())),
    //     content: Set(Some("use Loco: https://loco.rs".to_string())),
    //     ..Default::default()
    // };
    // active_model.insert(&_ctx.db).await.unwrap();
    // let res = articles::Entity::find().all(&ctx.db).await.unwrap();

    let one: Option<RangeModel> = reservation::Entity::find()
        .expr(Expr::col(reservation::Column::Timespan).cast_as(Alias::new("TEXT")))
        .into_model()
        .one(&_ctx.db).await.unwrap();
    println!("{:#?}", one);



    let res = reservation::Entity::find()
        .expr(Expr::col(reservation::Column::Timespan).cast_as(Alias::new("TEXT")))
        .all(&_ctx.db).await.unwrap();
    println!("{:#?}", res);

    for r in res {
        let am = r.into_active_model();
        let range = am.timespan.as_ref().clone().map(|x| {
            println!("timespan str: {}", x);
            TstzRange::from_string(&x).unwrap()
        }).unwrap();

        let updated_end = range.end().and_then(|dt| dt.checked_add_days(Days::new(1))).unwrap();
        let new = TstzRange::from_datetime_pair(range.start().unwrap(), updated_end);
        println!("original: {:#?}, new: {:#?}", &range, &new);
        let v = Value::from(new.clone());
        let range_expr = Expr::cust_with_expr("$1::TSTZRANGE", SimpleExpr::Value(v));

        let update = Query::update()
        .table(reservation::Entity)
        .value(reservation::Column::Timespan, range_expr)
        .and_where(Expr::col(reservation::Column::Id).eq(am.id.into_value().unwrap()))
        .to_owned();


        let stmt = builder.build(&update);
        let result = _ctx.db.execute(stmt).await?;
        println!("Updated result: {:?}", result);

    // .columns([reservation::Column::Timespan])
    // .values_panic([Expr::cust_with_expr("$1::TSTZRANGE", SimpleExpr::Value(range_string.into()))])

    }
    let res2: Vec<RangeModel> = reservation::Entity::find()
        .expr(Expr::col(reservation::Column::Timespan).cast_as(Alias::new("TEXT")))
        .into_model()
        .all(&_ctx.db).await.unwrap();
    println!("{:#?}", res2);



    Ok(())
}
