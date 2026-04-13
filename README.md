# Fluoris
Discord bot that manages MC server for CMU 2030 written in rust!!!\
Current features implemented: \
/age : returns the date of the user's command\
/ping: returns the latency of the bot\
/verify: verifys the user by adding them to a database

![image](./Nom.png)
Source: Arknights

This bot was inspired by jellyfishes. They are very beautiful creatures. Each of them glideding as if they know where to go. Hope one day we can all find our own jellyfishes gliding across the sky to follow. May we find all find an umbrella to shield us.

How to run the bot:

1) Download the repo
2) Create a .env file and add the line "DISCORD_TOKEN"= [token here]
3) Go to https://discord.com/developers/applications and create or link your account
4) Click create a new application
5) Go to bot and generate a Token and add it to where [token here] was in step two
6) Go to OAuth 2 and generate a link to invite into your server!!!
7) Go to https://www.postgresql.org/download/ and download postgresql
8) Go to Pgadmin4 and create a table with MC as the name and MCID, MCUser, Discord as columns  
9) Go to the same env file and add the PGsql connection link which follows "postgresql://postgres:{Password here}@{Address}:{Port}/postgres"
10) Go to /verify and change the role id to what ever role you are checking

Any questions feel to reach out at:\
Discord: @bocchithedepression\
GitHub: Neko-Rin\
Reddit: u/Jolly_Hour_5076 

Build: Alpha\
Status: W.I.P\
Update: 4/13/26 