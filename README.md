# Plus 9k

## What it does

This action automatcially replies to `+1` comments to let the user know that a more helpful way is to either react with an emoji to an existing post or to provide more context.

## How it looks like

![Example comment](/docs/img/example-comment.png "Plus9k in action")

## Usage

Production example:

```yaml
on: [issue_comment]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - name: Run plus-9k
      uses: scepticulous/action-plus-9k@master
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      with:
        MESSAGE: "Please do not use +1 comments :slightly_frowning_face:"
```

## Input

* `GITHUB_TOKEN`-env-var is required
* `GITHUB_EVENT_PATH`-env-var is automatically used behind the scenes
* `MESSAGE`-input is optional, the default is provided [here](/data/default-message.txt).

## License

The scripts and documentation in this project are released under the [GPLv3](LICENSE)
