# Project Manager

[![Test app](https://github.com/zackfall/project-manager/actions/workflows/test_app.yml/badge.svg)](https://github.com/zackfall/project-manager/actions/workflows/test_app.yml)

A Project Manager in Terminal wrote with The Rust Programming Language using the crate [TUI-rs](https://github.com/fdehau/tui-rs) for the Terminal User Interface and
[Octocrab](https://github.com/XAMPPRocky/octocrab.git) to interact with the GithubAPI.

This app will help me to improve my abilities with the language and know more about the terminal GUIs, also I will use this app for my personal use, to handle the projects
I work in, easily.

I will try to make test before make some implementation, this means we're gonna do TDD (Test Driven Development), so we can have a better understanding of what the app does,
and what we want to get from a function, also, this allows us to avoid some types of errors that may bother us in the future.

For each task or feature, or bug, or whatever it needs a commit, except typo errors, I will try create an issue, and in the commit description, link the issue describing what
we are making in the commit.

## Features
* [x] Connect to Github API.
* [ ] You can see the files that make up the project.
* [ ] Manage issues with the Github API.
  * [ ] Manage comments.
  * [ ] Manage Labels.
  * [ ] Manage Assignees.
* [ ] Work with pull requests.
* [ ] Integrated editor to edit issues, repositories, etc.
