---
sidebar_position: 1
---

# Get started

Setting up SMU **is almost as simple** as using SMU itself

### Pre-requisites

- [Homebrew](https://docs.brew.sh/Installation)
- Bash, zsh, or fish

## Installation

```bash
brew install dmnkgrc/smu/smu
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
