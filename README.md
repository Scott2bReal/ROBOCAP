# ROBOCAP

![image]('https://user-images.githubusercontent.com/69652149/197212796-c0e3c140-80a6-4afb-a9ee-85f33968e65b.png')

A Discord bot written in Rust to keep track of "bottle caps" for tabletop RPGs.
Brought to life by [serenity](https://github.com/serenity-rs/serenity) and
[shuttle](https://www.shuttle.rs/).

Inspired by the [Glass Cannon](https://www.glasscannonnetwork.com/) podcast's
house rule of the Game Master giving out bottle caps to players when they do
something clever, bold or funny. Players can cash in these tokens at their
discretion to gain the ability to roll twice and take the better result on any
roll

`ROBOCAP` keep track of who has how many caps, and can give players a history
of all the caps they've earned. 

## Getting Started

You can use `ROBOCAP` in your own Discord server! 

1. [Create a fork](https://docs.github.com/en/get-started/quickstart/fork-a-repo) of this repository
2. Register a new bot application on Discord
3. Make a `Secrets.toml` file in the root directory that looks like this:
```toml 
GUILD_ID = 'your server ID here'
DISCORD_TOKEN = 'your discord bot token'
```
4. Follow instructions [here](https://docs.shuttle.rs/guide/installation.html) to get set up with `shuttle`

## Commands

* `/list-available` - Lists currently available bottle caps for the user who uses the command
* `/history` - Lists all caps past and present a user has earned
* `/use` - ~Use a bottle cap~ 
* `/give-cap` - Give a bottle cap to a user


## Permissions

Most likely, only the person running the game should be the one awarding bottle
caps to players. The server administrator can select who has access to the
`give-cap` command by using the built-in Discord settings! 

Server Settings > Integrations > Bots and Applications > ROBOCAP
