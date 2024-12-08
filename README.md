# Ponger

Discord bot that announces CTFs (currently) and creates roles & channels (planned).

## Todo

- [x] Add a CI/CD pipeline
- [x] Clean up the damn code
- [x] Add proper error handling 💀
- [x] Add roles to user on reaction add
- [x] Remove role from user on reaction remove
- [x] Create a channel with appropriate perms
- [ ] Add a option to ping a role per announcement
- [ ] Add polls to choose CTFs
- [x] Create a seperate testing bot

## Polling Setup

The poll would contain 2 fields, `description` and `URLs`. The `description`
should contain each of the CTFs along with their 1 liner, i.e.:

```txt
CTF 1 - Beginner level CTF, no signup limit
CTF 2 - Intermediate CTF, 4 members per team
```

The `URLs` should contain a string with space-seperated CTFTime URLs corresponding
to the CTFs given in the `description` above.
