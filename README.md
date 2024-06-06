# A Netscape Bookmark File parser

That is a fork of [bookmarkt](https://crates.io/crates/bookmarkt) crate.
Forked from the [original repository](https://git.sr.ht/~vlnk/bookmarkt).

I needed to deduplicate my bookmarks, but unfortunately, original crate did not
exposed `item::Item` struct, which made my task of deduplication quite
problematic.
