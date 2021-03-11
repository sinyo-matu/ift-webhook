# ifttt_webhook_rust

binding to the ifttt webhook api.
there is a async interface can be activate in feature `non-blocking`.
and there is a time delay trigger function for delay the trigger in feature `delay`.

the blocking interface use [ureq](https://crates.io/crates/ureq),
and the non-blocking interface use [reqwest](https://crates.io/crates/reqwest) internally.
