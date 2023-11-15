# Database migrations
Some notes here:
1. Migrations will not be backwards compatible in the prototype (L)
2. Migration 100 is supposed to be a redis database but to reduce dependencies PostgreSQL is used

Other stuff to say
1. When a post or thread is created it stays in the database forever (Unless if there is a data deletion request)
   1. This is for moderation and to stop people from posting bad things and just deleting the evidence

