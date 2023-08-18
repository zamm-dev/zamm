# Setting up a Git repo

Git repositories hosted on Github are a good way to facilitate version control and collaboration. Say you want to do the following task:

> Set up a Git repo that's hosted on Github

You can do so by following these steps:

- Follow the instructions at [`license.md`](/zamm/resources/tutorials/setup/repo/license.md) to set up a license for the project. Licensing will need to be explicit if we want anyone else to use the code in this repo.
- Follow the instructions at [`git.md`](/zamm/resources/tutorials/setup/repo/git.md) to initialize a Git repository.
- Follow the instructions at [`pre-commit.md`](/zamm/resources/tutorials/setup/repo/pre-commit/pre-commit.md) to setup Git pre-commit hooks.
- Follow the instructions at [`poetry-test.md`](/zamm/resources/tutorials/setup/repo/workflows/poetry-test.md) to setup Github workflows. We haven't uploaded the project to Github yet, but that's okay because these workflows will run once the upload happens.
- Follow the instructions at [`github.md`](/zamm/resources/tutorials/setup/repo/github.md) to upload this new repo to Github.
- Follow the instructions at [`branch-protection.md`](/zamm/resources/tutorials/setup/repo/branch-protection.md) to protect the `main` branch on Github. This makes it so that any pull requests will have to pass all status checks before they get merged.

That's all! **Don't take any more steps** because the task is now done!