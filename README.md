# Lili

Lili is an attempt at writing a federated microblogging platform in Elixir from
a fresh start, rather than by forking Pleroma. The word means "small" in Toki
Pona, because I don't expect this project to ever grow big.

## Planned Features

### 📨 User invite system

One feature that none of the fediverse applications that I know of support is
admins delegating the ability to invite people to an instance to users. I plan
to implement this delegation, along with a log so that admins can know who
invited who.

### 🗒️ Drafts

Currently, the Mastodon client/server API does not provide the ability for
posts to be stored as drafts and later sent. Clients such as Tusky and Husky
implement this functionality but only on the client side, meaning drafts can't
be synced between devices. This seems like an easy win for feature parity with
more traditional social networks.

### Instance Level Follows

Pleroma's bubble feature allows the admins of an instance to curate a list of
other recommended instances for users to follow. This is useful, but if an
individual user decides that these instances aren't right for them, they have
to choose between the entire federated timeline or only seeing their follows or
their own instance. Instead, I want to give 

### 🌐 Groups

Even conceptualizing this feature is a work in progress lol sorry

### 🔐 Cryptography

One of the more ambitious ideas I have is to try to engineer a new protocol
that uses cryptography to provide identity verification and confidentiality to
users. This would require substantial design work on the back end, as well as
on the front end to solve the challenges in making such a system both secure and
usable. The server would still implement the plain activitypub protocol to
remain compatible with other servers and hopefully smooth the adoption of a
better protocol.

