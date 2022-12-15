# envsync

This is a Rust port of [env-sample-sync](https://github.com/acaloiaro/env-sample-sync).

It keeps `.env` files in sync with `env.sample`.

It checks whether the local repository has an `.env` file and if one exists, it
is scrubbed of secrets/values and made available as `env.sample`. This ensures
that all application environment variables are safely and automatically
documented without leaking secrets.

## Installation

```
cargo install --git https://github.com/mre/envsync
```

## Usage

```bash
envsync
```

This will check for an `.env` file in the current directory and if one exists,
it will be scrubbed of secrets/values and made available as `env.sample`.

You can pass in custom example values for environment variables for environment variables

For example, say you have the following `.env` file:

```bash
FOO=bar
BAR=baz
BAZ=qux
```

and you want to set the value of `BAR` to `bla`, you can do so by running:

```bash
envsync -e BAR=bla
```

This will result in the following `env.sample` file:

```bash
FOO=<FOO>
BAR=bla
BAZ=<BAZ>
```

## CI/CD

I use this in my CI/CD pipelines to ensure that all environment variables are
documented and that secrets are not leaked.

## Related Projects

- [envy](https://github.com/mre/envy): A Rust library for managing environment variables
  globally. Automatically loads environment variables from a global
  config file and sets them in the current shell session when entering a directory.