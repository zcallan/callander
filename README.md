# callander

An experimental project for me to practice Rust (backend) and Svelte (front-end).

It also uses a PostgreSQL database and runs in Docker.

`callander` (pun-intended) is a 'Friend Management System', or FMS.

You can add friends (or acquaintances!) to the system, where you can then do the following:
- Add birthdays and 'met at' (with varying accuracy)
- Add ideas for each friend (e.g. gifts, conversations, activities, etc)
- Track events (e.g. meetups, life events, etc)
- More soon

There is also limited functionality for creating posts, to act as a small personal blog or diary.

I have been very cautious not to make this application 'creepy'; it's not meant to be a stalking app or to record intricate details or opinions about somebody.

It's meant to act as a secondary memory for maintaining good friendships with people.

An example could be to note a conversation that you would like to continue with somebody, that got caught off before you could finish it.

Another example could be if you read an article that you think would be an interesting talking point with someone, you could add it as a note for that person (with a link to the article or just add some bullet points).

### Technical details

I am currently using SvelteKit, however I don't want SSR so I might swap it out.

Authentication currently uses [Passkeys](https://fidoalliance.org/passkeys/). I leveraged [Hanko](https://www.hanko.io/) for the auth server, though I have to give it a kick after 7 days of inactivity. I also have two separate accounts for dev and prod which is annoying, since on the free tier you can only have 1 project per account (fair enough). Might change providers or roll my own auth server at some point.

The front-end is hosted using Cloudflare Workers. The API and database are hosted using Render. I was using Fly.io but they had serious server issues one day and moving across was super easy.

## Running the project

#### Web

Go to the `/web` directory, and run the following commands.

```bash
$ yarn
$ yarn dev
```

More information can be found in the README inside `/web`.

#### API

Go to the `/api` directory. Make sure Docker is running on your machine.

```bash
$ docker-compose up -d
```

Once your database is going, run the server.

```bash
$ cargo run
```

More information can be found in the README inside `/api`.
