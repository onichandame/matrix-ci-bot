# Matrix CI Bot

This is a Matrix CI bot that integrates GitHub Actions with Matrix. It automatically sends workflow status updates to specified Matrix rooms when GitHub Actions workflows complete.

*No need to change existing workflow definition!*

![example](https://res.cloudinary.com/onichandame/image/upload/v1764129762/matrix-ci-bot-example.png)

## Features

- Automatically sends GitHub Actions workflow status updates to Matrix rooms
- Supports rich text formatting for detailed workflow information
- Provides fallback plain text content for compatibility

## Why and How

I've found some existing github actions that can send ci status to matrix. The best one is [this](https://github.com/Cadair/matrix-notify-action). But I did not find it ergonomic to use, also it lacks notification on the start of workflow. In essence:

- To use it I have to change my existing workflow yaml, add a new step. This is bad cuz the person writing the main workflow does not want to learn the notification part.
- For long-running workflows, I often need to know when it starts and when it completes.

None of the existing tools match my needs. So I created it myself.

It is an ephemeral workflow that does not pollute any existing ones, utilizing [workflow_run](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#workflow_run) event.

## Usage

See [the example workflow](./.github/workflows/notification.yml])

## Development

### Prerequisites

- Rust 1.91.1 (via Cargo)
- Matrix bot account
- GitHub Actions repository with existing workflows to track

### Installation

1. Clone the repository
2. Install dependencies:
   ```bash
   cargo build
   ```

## Contributing

Contributions are welcome! Please submit a pull request with your changes.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

For questions or issues, please open an issue on GitHub. You can also reach out through the [Matrix room #xiao-oss-general:envs.net](https://matrix.to/#/#xiao-oss-general:envs.net?via=github.com).
