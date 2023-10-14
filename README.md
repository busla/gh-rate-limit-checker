# Github rate-limit checker

Simple cli util to check the rate-limit against a PAT and output as a table.

Instead of curling and `jq` magic:

```json
// curl -i -H "Authorization: token <some-token>" https://api.github.com/rate_limit | jq -r '...'

{
  "resources": {
    "core": {
      "limit": 5000,
      "used": 0,
      "remaining": 5000,
      "reset": 1697283778
    },
    "search": {
      "limit": 30,
      "used": 0,
      "remaining": 30,
      "reset": 1697280238
    }
    // ....
  }
}
```

We get a colorized table output (no colors in md snippets 🤷).

```console
Resource                    | Limit | Used | Remaining | Reset
core                        | 5000  | 0    | 5000      | 2023-10-14 11:59:06
search                      | 30    | 0    | 30        | 2023-10-14 11:00:06
graphql                     | 5000  | 99   | 4901      | 2023-10-14 11:01:01
integration_manifest        | 5000  | 0    | 5000      | 2023-10-14 11:59:06
source_import               | 100   | 0    | 100       | 2023-10-14 11:00:06
code_scanning_upload        | 1000  | 0    | 1000      | 2023-10-14 11:59:06
actions_runner_registration | 10000 | 0    | 10000     | 2023-10-14 11:59:06
scim                        | 15000 | 0    | 15000     | 2023-10-14 11:59:06
dependency_snapshots        | 100   | 0    | 100       | 2023-10-14 11:00:06
audit_log                   | 1750  | 0    | 1750      | 2023-10-14 11:59:06
code_search                 | 10    | 0    | 10        | 2023-10-14 11:00:06
Overall                     | 5000  | 0    | 5000      | 2023-10-14 11:59:06
```
