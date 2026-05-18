# Forge Strategy

RustUse may be mirrored across multiple public Git forges to improve availability, discoverability, and contributor access without fragmenting the project.

## Canonical repository

GitHub is currently the canonical write target.

Pull requests that change the main branch, tags, releases, or publishing state are finalized there.

GitLab mirror support is documented in this repository, but live activation is optional until the mirror URLs and matching SSH key material are configured.

## Mirrors

Mirrors may exist on GitLab, Codeberg or Forgejo, SourceHut, or other public forges.

- GitLab is the first supported mirror surface and may accept issues or merge requests.
- Codeberg or Forgejo is the recommended second mirror once the GitLab path is stable.
- SourceHut is treated as an optional later mirror with documentation first, manual sync if needed, and no dedicated CI for now.

## Activation checklist

The checked-in `.github/workflows/mirror.yml` workflow stays dormant until the canonical repository has:

- `GITLAB_MIRROR_URL`
- optional `CODEBERG_MIRROR_URL`
- optional `SOURCEHUT_MIRROR_URL`
- `GITLAB_MIRROR_SSH_KEY`
- optional `CODEBERG_MIRROR_SSH_KEY`
- optional `SOURCEHUT_MIRROR_SSH_KEY`

## Sync model

- GitHub `main` mirrors outward.
- Tags mirror outward.
- Releases are coordinated from GitHub.
- crates.io publishing happens only from canonical release automation.
- Mirror sync may be handled manually or by a GitHub-hosted mirroring workflow after canonical CI succeeds and remotes and secrets are configured.

## Security and provenance

- Release authority belongs to the canonical repository.
- crates.io publishing must not run from mirrors.
- Mirror repositories should not receive publish tokens.
- External contributions should preserve authorship when ported.
