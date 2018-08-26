# Log Analyzer

Analyze customer log files to detect patterns automatically.

## Define a pattern

We use a `.json` file to define patterns in a machine readable format.

```
{
  "name": "platform log",
  "steps": [
    {
      "name": "sync_org",
      "identifier": "[task:sync_user_repos]",
      "payload": [
        "/user=(\w+)/",
        "/user_id=(\d+)/",
        "/repos_count=(\d+)/"
      ]
    }
  ]
}
```

This allows us to read a log file and dynamically apply rules to extract important information.