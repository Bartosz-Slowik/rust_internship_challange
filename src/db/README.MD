# Challenge 2: Caching search results

## Description 
When we send requests to search for parts to an external API we do so using our users API keys. This is because EMS often have special deals with distributors and get unique price information (in addition to the part information).

Since they often look for the same parts, we want to implement _caching_. That is if a user has searched for parts with search string "AAA", and has done so again in an hour we wouldn't want to resend the request to an external API, but instead use the results stored in our cache.

In a little bit of a contrived example, we will use Postgres as our cache. To simplify things, you don't have to worry about storing the actual responses. Instead we just want to remember:

  * The search string that was used in the request
  * Whether the requests was successful or not. Sometimes requests to external APIs
    can fail because they are temporarily unavailable, for example. 

Note that the requests are unique for each API key. API keys could change for one user, and some users might work in the same company sharing an API key.

**Bonus question**

How would you efficiently check if the request was already sent? A comment is enough for this. 

## Summary
Task: Implement a suitable Postgres table and Rust struct for storing user requests.

* Search requests depend on the API key, which is given by the user that sends the request. See user migrations and [users.rs](users.rs).
* A request is always sent with some API key.
* You need to store if the request was successful.
* You don't have to worry about how to store the responses, just knowing whether we need to send a request is enough.

You might find the following commands useful when working with diesel. The diesel_cli
 should already be installed. See their description in the [docs](https://crates.io/crates/diesel_cli):
* `diesel migration generate`
* `diesel migration run`
* `diesel migration revert`
* `diesel database reset`

*Note:* if you are more familiar with `sqlx` or `tokio-postgres` you can also use that (though then you won't be able to benefit from the code we've already written).

