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
