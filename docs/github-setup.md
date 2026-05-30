# GitHub Setup and Hardening

## 1) Create remote repository and set origin

Create the GitHub repository (replace `<owner>` and `<repo>`):

```bash
gh repo create <owner>/<repo> --private --source . --remote origin --push
```

If `origin` already exists, update it:

```bash
git remote set-url origin git@github.com:<owner>/<repo>.git
git push -u origin main
```

## 2) Apply repository hardening

Run:

```bash
./scripts/harden-github.sh <owner>/<repo>
```

This applies:

- `main` as default branch
- PR-based workflow protections on `main`
- Required checks (`test`, `codeql`)
- Code owner review + stale review dismissal
- Admin enforcement + linear history
- Branch deletion/force-push protection
- Vulnerability alerts and automated security fixes

## 3) Verify GitHub Actions

After first push, verify workflows are present and green:

- `CI`
- `Security`
- `Dependency Review`

## Notes

- If org-level policies override a setting, `gh api` calls may return authorization or policy errors.
- Update `required_status_checks.contexts` in `scripts/harden-github.sh` if workflow job names change.
