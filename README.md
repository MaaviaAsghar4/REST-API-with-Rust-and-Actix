# REST-API-with-Rust-and-Actix

This is a simple REST API developed using RUST Actix.

[Here is the link to the tutorial](https://hub.qovery.com/guides/tutorial/create-a-blazingly-fast-api-in-rust-part-1/)

### API design
Our REST API needs to have three endpoints :

- /tweets
```
  GET: list last 50 tweets
  POST: create a new tweet
```
- /tweets/:id
```
  GET: find a tweet by its ID
  DELETE: delete a tweet by its ID
```
- /tweets/:id/likes
```
  GET: list all likes attached to a tweet
  POST: add +1 like to a tweet
  DELETE: add -1 like to a tweet
```
### Curl API to test API endpoints

```
# list tweets
curl http://localhost:8000/tweets

# get a tweet (return status code: 204 because there is no tweet)
curl http://localhost:8000/tweets/abc

# create a tweet
curl -X POST -d '{"message": "This is a tweet"}' -H "Content-type: application/json" http://localhost:8000/tweets

# delete a tweet (return status code: 204 in any case)
curl -X DELETE http://localhost:8000/tweets/abc

# list likes from a tweet
curl http://localhost:8000/tweets/abc/likes

# add one like to a tweet
curl -X POST http://localhost:8000/tweets/abc/likes

# remove one like to a tweet
curl -X DELETE http://localhost:8000/tweets/abc/likes
```
