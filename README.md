# Plus 9k

## What it does

This action automatically replies to `+1` comments to let the user know that a more helpful way is to either react with an emoji to an existing post or to provide more context.

## How it looks like

![Example comment](/docs/img/example-comment.png "Plus9k in action")

## Usage

Full production example:

```yaml
name: Plus9k - Rust

on: [issue_comment]

jobs:
  plus9k:
    name: Plus-9k
    runs-on: ubuntu-latest
    steps:
      - name: Run Plus 9k
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          MESSAGE: "Please do not use +1 comments :slightly_frowning_face:"
        uses: docker://scepticulous/plus9:v1.0.0
```

## Input

* `GITHUB_TOKEN`-env-var is required
* `GITHUB_EVENT_PATH`-env-var is automatically used behind the scenes

### Message

The default message is:
```text
Thanks for supporting this discussion by sharing your opinion. ❤️
Did you know? Dedicated +1-comments can make it hard to follow the discussion.

Sharing your support via emoji reactions on comments avoids that problem
and helps us get a complete picture of everybody's opinion.
Make sure to use a reaction next time to upvote an idea.
```

It can be changed via the `MESSAGE` input, as shown in the example above.

## License

The scripts and documentation in this project are released under the [GPLv3](LICENSE)
