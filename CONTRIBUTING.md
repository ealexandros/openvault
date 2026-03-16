# Contributing to OpenVault

Thank you for your interest in contributing! 🎉

We welcome bug reports, feature requests, documentation improvements, and code contributions.

Please read the guidelines below to help keep the project consistent and easy to maintain.

---

## Getting Started

1. Fork the repository
2. Clone your fork

```bash
$ git clone https://github.com/ealexandros/openvault.git
$ cd openvault
```

3. Install dependencies

```bash
$ bun install
```

4. Start the development environment

```bash
$ bun dev
```

---

## Branching

Create a new branch for your changes.

```bash
$ git checkout -b feature/my-feature
```

Recommended branch prefixes:

| Prefix      | Purpose                                    |
| ----------- | ------------------------------------------ |
| `feature/`  | New features                               |
| `fix/`      | Bug fixes                                  |
| `docs/`     | Documentation updates                      |
| `refactor/` | Code improvements without behavior changes |
| `chore/`    | Maintenance tasks                          |

---

## Code Style

Please follow the existing code style in the repository.

General guidelines:

- Use **TypeScript**
- Keep components **small and composable**
- Use **clear, descriptive prop names**
- Avoid unnecessary dependencies

If formatting tools are configured, run:

```bash
$ bun run lint:check
$ bun run format:check
```

---

## Commit Messages

Use clear and descriptive commit messages.

Example format:

```
type: short description
```

Examples:

```
feat: add vault export feature
fix: resolve wallet connection issue
docs: update installation instructions
```

---

## Pull Requests

When submitting a pull request:

1. Ensure your branch is up to date with `main`
2. Provide a clear description of the changes
3. Link related issues if applicable

Checklist before submitting:

- [ ] Code compiles successfully
- [ ] Husky pre-commit hooks pass (handles formatting and linting)
- [ ] No unnecessary files included
- [ ] Documentation updated if needed

---

## Reporting Issues

If you encounter a bug, please open an issue and include:

- A clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Screenshots or logs if helpful

---

## Feature Requests

Feature requests are welcome! Please open an issue describing:

- The problem you're trying to solve
- Your proposed solution
- Any alternatives you've considered

---

## Questions

If you have questions, feel free to open a discussion or issue.

Thanks for contributing! 🚀
