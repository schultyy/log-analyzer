# Log Analyzer

Analyze customer log files to detect patterns based on a configuration. This tool helps me to find irregularities in log files for instance for a specific user. This works line-based and gets all information from a configuration file.

## Development

### Requirements

* Rust 1.27


## Usage

Clone this repository:

```
$ git clone git@github.com:schultyy/log-analyzer.git /tmp/log-analyzer
```

Make a release build:

```
$ cargo build --release
```

Copy this into your path:

```
$ cp target/release/log-analyzer ~/.bin
```


## Config files

Config files are defined in JSON format and do look like this:

```json
{
  "name": "worker",
  "date_identifier": "^time=(\"\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}Z\")",
  "context_arguments": [
    "job_id=(\\d+)"
  ],
  "steps": [
    {
      "name": "received amqp delivery",
      "identifier": "msg=\"received amqp delivery\"",
      "payload": [
        "processor=([@\\w\\.-]+)",
        "self=(\\w+)"
      ]
  }
}
```

`context_arguments`: These regular expressions are used to determine if a given line does contain information that needs to be processed further. 

`steps`: With these steps you can define a workflow that needs to be met for a certain user/repository/etc.. 

A single step:

`name`: The name of the step. This is a display name and not used for any evaluation
`identifier`: Regular expression to match this specific line
`payload`: Additional information that shall be extracted from this log line
