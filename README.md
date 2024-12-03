# HiddN Website

please use rust typeshare cli

## Getting Started

This section explains how to set up, run, and develop on this project.

### Prerequisites

- **Rust**: Ensure you have Rust installed. You can install Rust using [rustup](https://rustup.rs/).
  - **`typeshare-cli`**: Install the `typeshare-cli` crate using Cargo.

    ```sh
    cargo install typeshare-cli
    ```

- **Node.js**: Ensure you have Node.js installed. You can install Node.js from [nodejs.org](https://nodejs.org/).
- **Docker**: Ensure you have Docker and Docker Compose installed. You can install Docker from [docker.com](https://www.docker.com/).

### Dependencies

- **Rust**: Install Rust using rustup.

  ```sh
  # For Linux, macOS, and WSL
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

  # For Windows, download the installer from rustup.rs
  ```

- **Node.js**: Install Node.js.

  ```sh
  # For Linux and macOS
  wget -qO- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash

  export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "${HOME}/.nvm" || printf %s "${XDG_CONFIG_HOME}/nvm")"
  [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh" # This loads nvm

  # For Windows, download the installer from nodejs.org
  ```

- **Docker**: Install Docker and Docker Compose.

  ```sh
  # For Linux (Ubuntu)
  sudo apt-get update
  sudo apt-get install -y docker.io docker-compose

  # For other Linux distributions, follow the instructions from docker.com

  # For macOS, download Docker Desktop from docker.com
  # For Windows, download Docker Desktop from docker.com
  ```

### Environment Setup

1. **Clone the repository**:

   ```sh
   git clone https://github.com/on9au/hiddn-website2.git
   cd hiddn-website2
   ```

2. **Create a `.env` file**:

   ```sh
   cp .env.example .env
   ```

3. **Fill in the required values in the `.env` file**.

### Running the Application

#### Using Docker

1. **Build and start the containers**:

   ```sh
   sudo docker compose up --build
   ```

2. **Access the application**:
   - The application will be accessible at `http://localhost:3000`.

3. **Manage the database**:
   - You can connect to the MySQL database using any MySQL client with the credentials provided in the `.env` file.

#### Without Docker

1. **Build or Run the project**:

    ```sh
    # To build the project
    make build

    # To run the project in rust debug mode
    make run

    # To run the project in rust release mode
    make preview
    ```

### Development

#### Frontend Development

1. **Navigate to the client directory**:

   ```sh
   cd client
   ```

2. **Install dependencies**:

   ```sh
   npm install
   ```

3. **Run the development server**:

   ```sh
   npm run dev
   ```

#### Backend Development

1. **Run the backend server**:

   ```sh
   RUST_LOG=debug cargo run
   ```

### Platform-Specific Instructions

#### Windows

- Ensure you have WSL2 installed and set up. Follow the instructions from [Microsoft's documentation](https://docs.microsoft.com/en-us/windows/wsl/install).

#### macOS

- Ensure you have Homebrew installed. You can install Homebrew from [brew.sh](https://brew.sh/).

#### Linux

- Ensure you have the necessary build tools installed.

  ```sh
  sudo apt-get update
  sudo apt-get install -y build-essential
  ```

### Additional Information

- **TypeShare CLI**: This project uses the TypeShare CLI for generating TypeScript bindings from Rust. Ensure you have it installed.

  ```sh
  cargo install typeshare-cli
  ```

- **Environment Variables**: The `.env` file contains configuration for the application. Ensure it is correctly set up before running the application.

### Contributing

Feel free to open issues or submit pull requests. For major changes, please open an issue first to discuss what you would like to change.

### License

This project is proprietary and not licensed for public use.
