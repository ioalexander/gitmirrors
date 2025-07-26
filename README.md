# GitMirrors - clone repositories automatically

## Running GitMirrors

```
git clone
```

```sh
cp .env.example .env
```

Fill the `.env` file.

Your `GID` and `UID` should be filled

```
docker-compose up --build
```

## Develop GitMirrors

```
git clone
```

Fill the `.env` file.

Your `GID` and `UID` should be filled

```
docker-compose -f docker-compose.dev.yml up
```

## First login

Username: `admin`
Password: `any_random_string`

**Warning!** You should change the password after that.
Otherwise, the password will still be any.

## Roadmap

[x] basic functionality - repositories are being cloned
[ ] cleaning code base (e.g. linters, refactorings)
[ ] docker image building pipelines
[ ] pull repositories data
[ ] advanced input validation
[ ] users management
[ ] mobile version (right now it's still usable, but I still want to fix sidebar)
