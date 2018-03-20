# Ticket
Insert issue tracking ticket references into your commit messages automatically

## Install

Download the binary release, or build using `cargo build --release`. The resulting executable should be placed on your `PATH` so that it's accessable with the name `ticket`.

## Usage

In your repository run `ticket init` to install the git hook, then use `ticket set REF` to set the ticket reference. See `ticket --help` for further details.

It's *strongly* recommended that you add `.ticket` to your global gitignore file.

## Internals

Ticket creates a file named `.ticket` in the root of your repository that contains the current ticket reference. `ticket set` overwrites this file, and the installed git hook reads from it to know what to insert into your commit messages.

The commit hook is a `prepare-commit-msg` hook, and requires that `ticket` is available on your `PATH` to work.
