# Sparks!

A Discord bot for indie TRPGs.

(she/her)

Sparks uses [serenity.rs](https://github.com/serenity-rs/) to talk to Discord, and currently supports Forged in the Dark, Sparked by Resistance, Powered by the Apocalypse, and Wild Words rolls. Thanks to [River Ray](https://riverray.itch.io) for the original TypeScript implementation of Wild Words, on which Sparks' current Rust code is based.

(A brief note: the official help text asks users to report issues on Sparks' itch.io page. You're welcome to file them here as well, if you have a Github account; itch is given as the first point of contact only because most of Sparks' users are probably not programmers themselves.)

## Code

The meat of Sparks' code can be found in `src/interpreters`, which is responsible for taking vectors of dice and generating results from them. It uses a struct called `Rolls` to store dice, which can be found in `src/lib.rs`. `src/commands/builders` is where the Discord user commands are constructed. `src/commands/handlers` contains the code that handles receiving user input, invoking dice rolls, and passing the results of rolls to the appropriate interpreter. `src/main.rs` handles the initial setup, and routes commands to code located in the appropriate `src/commands/` file to await an actual reply.

## Contributions

I am open to these! If you have ideas for how to improve Sparks, please let me know.

## Privacy Policy

Sparks stores no data on its users.

## Terms of Service

You are free to use this bot in any way you wish. To use it on Discord, you will have to comply with Discord's own [Terms of Service](https://discord.com/terms) and [Community Guidelines](https://discord.com/guidelines).
