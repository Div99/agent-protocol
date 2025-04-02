# CI/CD Pipeline Documentation

This document explains how the CI/CD pipeline works for this project, including versioning, building, and publishing NuGet packages.

## Overview

The project uses GitHub Actions for CI/CD with the following key components:

1. **GitVersion** - Handles automatic semantic versioning
2. **NuGet Package Build** - Builds and packages the library
3. **GitHub Packages** - Hosts NuGet packages for development
4. **NuGet.org** - Hosts public releases

## Workflow Files

The repository contains the following workflow files:

- `.github/workflows/nuget-publish.yml` - Main pipeline that builds, tests, versions, and publishes packages
- `.github/workflows/release.yml` - Dedicated workflow for creating releases (manually triggered)

## Versioning Strategy

We use GitVersion to automatically calculate semantic versions based on Git history and branching patterns:

- **GitVersion.yml** - Defines the versioning rules for different branches

### Branch-Based Versioning

Versions are automatically calculated based on the branch:

- **main**: Production releases with clean semantic versions (e.g., `1.2.3`)
- **develop**: Beta pre-releases (e.g., `1.3.0-beta.1`)
- **feature branches**: Alpha pre-releases (e.g., `1.3.0-alpha.feature-name.1`)
- **hotfix branches**: Release candidates (e.g., `1.2.4-rc.hotfix-name.1`)
- **PR branches**: Pre-release with PR number (e.g., `1.3.0-pr.123.1`)

### Commit Messages for Versioning

You can control version increments using special commit messages:

- `+semver: major` or `+semver: breaking` - Bump major version
- `+semver: minor` or `+semver: feature` - Bump minor version
- `+semver: patch` or `+semver: fix` - Bump patch version
- `+semver: none` or `+semver: skip` - Don't increment version

## Build Process

The build process follows these steps:

1. Checkout code with full history (required for GitVersion)
2. Set up .NET environment
3. Calculate version using GitVersion
4. Restore dependencies
5. Build with the version from GitVersion
6. Run tests
7. Package as NuGet with appropriate version
8. Upload package as artifact

## Publishing

Packages are published based on the branch and event:

- **GitHub Packages**:
  - All packages from `main` branch
  - All packages from manual workflow runs

- **NuGet.org**:
  - Only packages from `main` branch
  - Only for push events (not PRs)
  - Requires `NUGET_API_KEY` secret

## Creating Releases

To create a new release:

1. Go to the "Actions" tab in GitHub
2. Select the "Create Release" workflow
3. Click "Run workflow"
4. Select the type of release (patch, minor, major)
5. Click "Run workflow" again

This will:

1. Create a commit with the selected semver increment
2. Push it to the main branch
3. Create a Git tag for the new version
4. Trigger the main workflow to build and publish the package

## Required Secrets

The pipeline requires the following GitHub secrets:

- `GITHUB_TOKEN` (automatically provided by GitHub)
- `NUGET_API_KEY` (for publishing to NuGet.org)
- `RELEASE_PAT` (for the release workflow to push to protected branches)

## NuGet Package Metadata

Package metadata is defined in `Directory.Build.props` including:

- Authors
- Description
- License
- Project URL
- Repository info
- Tags
- Icon
- README

## Debugging

If you encounter issues with the pipeline:

1. Check the GitHub Actions logs
2. Look for GitVersion output (shows calculated version)
3. Review the package artifacts for correct versioning
4. Ensure required secrets are configured
