


# Teloxide Telegram Bot Template

This project provides a template for creating Telegram bots using Rust and the Teloxide framework. It includes SQLite database integration, Docker support, and serves as a starting point for building custom Telegram bots.

## Features

- Basic bot structure using Teloxide
- SQLite database integration
- Todo list functionality as an example
- Docker support for easy deployment
- Modular architecture for easy expansion

## Getting Started

### Prerequisites

- Rust
- Docker (optional)

### Installation

1. Clone the repository:
```
git clone https://github.com/ju8sho/teloxide-telegram-bot-template.git
```
2. Navigate to the project directory:
```
cd telegram-bot-boshlangish
```

### Running the Bot

### Without Docker

1. Set up your Telegram Bot Token:
   ```
   export TELOXIDE_TOKEN=your_bot_token_here
   ```
2. Run the bot:
   ```
   cargo run
   ```

### With Docker

1. Build the Docker image:
   ```
   docker build -t teloxide-telegram-bot-template .
   ```
2. Run the container in detached mode:
   ```
   docker run -d -e TELOXIDE_TOKEN=your_bot_token_here teloxide-telegram-bot-template
   ```

   Make sure to replace `your_bot_token_here` with your actual Telegram Bot Token.

3. To check the running container:
   ```
   docker ps
   ```

4. To view logs:
   ```
   docker logs [CONTAINER_ID]
   ```
   Replace [CONTAINER_ID] with the actual container ID from the `docker ps` command.

5. To stop the bot:
   ```
   docker stop [CONTAINER_ID]
   ```

Note: The `-d` flag runs the container in detached mode, allowing it to run in the background.

## Configuration

- Set the `TELOXIDE_TOKEN` environment variable with your Telegram Bot Token.
- Modify the `data.db` path in `src/models/mod.rs` if needed.

## Versioning

For the versions available, see the [tags on this repository](https://github.com/ju8sho/teloxide-telegram-bot-template/tags).


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[Include your chosen license information]

## Disclaimer

This is a template project and may require modifications to suit specific bot requirements.

