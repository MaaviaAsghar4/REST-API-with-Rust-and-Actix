use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, post};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use diesel::query_dsl::methods::{FilterDsl, LimitDsl, OrderDsl};
use diesel::result::Error;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::like::{list_likes, Like};
use crate::response::Response;

use std::str::FromStr;

use crate::{DBPool, DBPooledConnection};

use super::schema::tweets;

pub type Tweets = Response<Tweet>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<Like>,
}

impl Tweet {
    pub fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            message,
            likes: vec![],
        }
    }

    pub fn to_tweet_db(&self) -> TweetDB {
        TweetDB {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            message: self.message.clone(),
        }
    }

    pub fn add_likes(&self, likes: Vec<Like>) -> Self {
        Self {
            id: self.id.clone(),
            created_at: self.created_at.clone(),
            message: self.message.clone(),
            likes,
        }
    }
}

#[derive(Queryable, Insertable, Debug, Deserialize, Serialize)]
#[table_name = "tweets"]
pub struct TweetDB {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub message: String,
}

impl TweetDB {
    fn to_tweet(&self) -> Tweet {
        Tweet {
            id: self.id.to_string(),
            created_at: Utc.from_utc_datetime(&self.created_at),
            likes: vec![],
            message: self.message.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetRequest {
    pub message: Option<String>,
}

impl TweetRequest {
    pub fn to_tweet(&self) -> Option<Tweet> {
        match &self.message {
            Some(message) => Some(Tweet::new(message.to_string())),
            None => None,
        }
    }
}

fn list_tweets(total_tweets: i64, conn: &DBPooledConnection) -> Result<Tweets, Error> {
    use crate::schema::tweets::dsl::*;

    let _tweets = match tweets
        .order(created_at.desc())
        .limit(total_tweets)
        .load::<TweetDB>(conn)
    {
        Ok(tws) => tws,
        Err(_) => vec![],
    };

    Ok(Tweets {
        results: _tweets
            .into_iter()
            .map(|t| t.to_tweet())
            .collect::<Vec<Tweet>>(),
    })
}

fn find_tweet(_id: Uuid, conn: &DBPooledConnection) -> Result<Tweet, Error> {
    use crate::schema::tweets::dsl::*;

    let res = tweets.filter(id.eq(_id)).load::<TweetDB>(conn);
    match res {
        Ok(tweets_db) => match tweets_db.first() {
            Some(tweet_db) => Ok(tweet_db.to_tweet()),
            _ => Err(Error::NotFound),
        },
        Err(err) => Err(err),
    }
}

fn create_tweet(tweet: Tweet, conn: &DBPooledConnection) -> Result<Tweet, Error> {
    use crate::schema::tweets::dsl::*;

    let tweet_db = tweet.to_tweet_db();
    let _ = diesel::insert_into(tweets).values(&tweet_db).execute(conn);

    Ok(tweet_db.to_tweet())
}

fn delete_tweet(_id: Uuid, conn: &DBPooledConnection) -> Result<(), Error> {
    use crate::schema::tweets::dsl::*;

    let res = diesel::delete(tweets.filter(id.eq(_id))).execute(conn);
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

#[get("/tweets")]
pub async fn list(pool: Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let tweets = web::block(move || list_tweets(50, &conn)).await.unwrap();
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let tweets_with_likes = Tweets {
        results: tweets
            .unwrap()
            .results
            .iter_mut()
            .map(|t| {
                let _likes = list_likes(Uuid::from_str(t.id.as_str()).unwrap(), &conn).unwrap();
                t.add_likes(_likes.results)
            })
            .collect::<Vec<Tweet>>(),
    };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweets_with_likes)
}

#[post("/tweets")]
pub async fn create(tweet_req: Json<TweetRequest>, pool: Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let tweet = web::block(move || create_tweet(tweet_req.to_tweet().unwrap(), &conn)).await;

    match tweet {
        Ok(tweet) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(tweet),
        _ => HttpResponse::NoContent().await.unwrap(),
    }
}

#[get("/tweets/{id}")]
pub async fn get(path: Path<(String,)>, pool: Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let tweet =
        web::block(move || find_tweet(Uuid::from_str(path.0.as_str()).unwrap(), &conn)).await;
    match tweet {
        Ok(tweet) => {
            let conn = pool.get().expect(CONNECTION_POOL_ERROR);
            let tweetBody = tweet.unwrap();
            let _likes = list_likes(Uuid::from_str(tweetBody.id.as_str()).unwrap(), &conn).unwrap();

            HttpResponse::Ok()
                .content_type(APPLICATION_JSON)
                .json(tweetBody.add_likes(_likes.results))
        }
        _ => HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
}

#[delete("/tweets/{id}")]
pub async fn delete(path: Path<(String,)>, pool: Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let _ = web::block(move || delete_tweet(Uuid::from_str(path.0.as_str()).unwrap(), &conn)).await;

    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
