---
title: Getting started
description: Guides you through the installation of smu and a basic configuration
---

Setting up SMU **is super simple**

### Pre-requisites

- [Homebrew](https://docs.brew.sh/Installation)
- Bash, zsh, or fish

## Installation

```bash
$ brew install dmnkgrc/smu/smu
```

## Create your first project

Projects are loaded from `~/.smu/projects` and they should be YAML files

Here is a super simple example:

```yaml
name: "Demo"
description: "This is a demo project."
steps:
  - description: "Install Node and Pnpm"
    run:
      - brew: fnm pnpm
  - description: Install Neovim
    run:
      - brew: neovim
      - pnpm: neovim
      - note: "Notes are great for giving more context about what's going on."
```

Guides lead a user through a specific task they want to accomplish, often with a sequence of steps.
Writing a good guide requires thinking about what your users are trying to do.

## Further reading

- Read the [configuration section](/reference/configuration) for more information on the configuration options
