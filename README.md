
# Automatdeck agent

Website: https://automatdeck.com
Documentation: https://doc.automatdeck.com

Automatdeck agent is a simple lightweight IT automation tool for your workflows, tickets and deployments which can be triggered on demand via REST API.

It allows you to manage your automations in a simple intuitive yet powerful configuration syntax which is quite similar to gitlab ci/cd.

Key Features of automatdeck:

- **Workflows:** Configure your workflows in terms of stages and jobs. Allowing you to trigger a specific workflow configuration via REST/API.
- **Variables:** Configure variables to be used by commands.
- **Meta-Data:** Allows to run flow using dynamic data.
- **Execution Strategy:** Run commands in same shell env or each command in different shell env.
- **Retry Policy:** Configure retry.
- **Pre & Post Execution scripts:** Run scripts before and after executions.
- **Custom Plugins:** Create your own python modules and use them as plugins.



## Documentation

Documentation is available on [Automatdeck documentation website](https://doc.automatdeck.com)

## Developing Automatdeck Agent

If you wish to contribute to automatdeck agent, you will first need to install Rust on your machine. Recommended Rust version is >=1.59.0

Secondly you will need to create an automatdeck account, create an agent, configuration and request. Please refer to the [documentation](https://doc.automatdeck.com).


### Build
To build your modified version of automatdeck agent:

```bash
cargo build
```

### Run

You will now need to modify the `config/settings.toml` with relevant info based on your account and agent.

```bash
cargo run --launch -d
```

### Diagnose

```bash
cargo run --diagnose --debug
```

```
Perform diagnosis

USAGE:
    ad-agent diagnose [OPTIONS]

OPTIONS:
        --debug                Print debug info
    -h, --help                 Print help information
        --list-new-requests    List new requests
```

> **Warning**
> Make sure to revert back your changes in `config/settings.toml` before doing a Pull Request. Unless of course changes in the latter concerns optimizations. 


## Authors

Automatdeck agent was created by Kheshav Sewnundun.


## License

[MIT](https://choosealicense.com/licenses/mit/)

