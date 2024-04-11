# Release Process

1. Make sure state of `develop` is what is to be released. 
2. `git checkout main && git pull origin main`
3. `git merge develop`
4. `cargo release patch --execute --workspace`
5. `git checkout develop`
6. `git merge main`
7. `git push --tags origin main develop`