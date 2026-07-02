# Starting from the project template

A ready-made Leptos CSR starter lives at
<https://github.com/matthewjberger/nightshade-api-template-leptos>. It is the base you clone to begin
a new app and then layer musaic onto. This page covers cloning it, keeping it current, and starting
a fresh project from it.

## Clone it

```sh
git clone https://github.com/matthewjberger/nightshade-api-template-leptos.git my-app
cd my-app
```

If you work in this checkout directly, pull the latest template changes with:

```sh
git pull
```

## Start a new project from it

The template repository is set up as a GitHub template, so the cleanest start is the **"Use this
template"** button on the repository page, which creates your own repo with a clean history. Clone
that new repo and build from there.

To do the same from the command line without the button:

```sh
git clone https://github.com/matthewjberger/nightshade-api-template-leptos.git my-app
cd my-app
rm -rf .git
git init
git add -A && git commit -m "initial commit from template"
git remote add origin <your-new-repo-url>
git push -u origin main
```

## Pull later template updates into your project

Once your project has its own history, track the template as a second remote and merge its updates
when you want them:

```sh
# one-time setup
git remote add template https://github.com/matthewjberger/nightshade-api-template-leptos.git

# whenever you want the latest template changes
git fetch template
git merge template/main            # add --allow-unrelated-histories on the first merge
```

Resolve any conflicts (they will be in files you have since customized), then commit the merge. Use
`git cherry-pick <commit>` instead of `merge` if you only want specific template commits.

## Add musaic to it

With the template cloned, add musaic and start building the UI:

```toml
[dependencies]
leptos-musaic = { git = "https://github.com/matthewjberger/musaic", features = ["full"] }
```

Then follow the [Quickstart](../README.md#quickstart): drop `<MusaicStyles/>` and `<ThemeProvider>`
at the root and compose components from there. The template already mounts a Leptos CSR app and
serves with Trunk, so musaic slots straight in. The [guide](book) and the runnable
[gallery](../examples/gallery) show what to reach for next.
