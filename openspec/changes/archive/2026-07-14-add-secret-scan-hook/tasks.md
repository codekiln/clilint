## 1. Configure the hook

- [x] 1.1 Add lefthook to `mise.toml` and install hooks when mise enters the trusted project.
- [x] 1.2 Add the name-free `identity-guard` pre-commit command to `lefthook.yml`.

## 2. Verify the guard

- [x] 2.1 Install the hook and verify the global `secrets:scan` task accepts the intended clilint files.
- [x] 2.2 Stage a temporary protected value, verify the hook blocks it, then remove the temporary file and staging entry.
- [x] 2.3 Run strict OpenSpec validation and review the repository diff for private scan data.
