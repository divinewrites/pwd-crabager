## pwd-crabager 🦀
A simple and "secure" password manager written in Rust, providing a command-line interface for managing your passwords.

## Features

- **Create Passwords:** Add a new password for a specific site or application.

- **Edit Passwords:** Modify the password for an existing site or application.

- **Generate Secure Passwords:** Automatically generate a secure password.

- **List Sites:** View a comprehensive list of sites and applications with stored passwords.


## Getting Started

### Prerequisites

Ensure you have Rust installed. If not, download it from [rustup](https://rustup.rs/).

### Clone the Repository

```bash
git clone https://github.com/divinewrites/pwd-crabager.git
cd pwd-crabager
```


## Usage/Examples

- **Create Passwords:** Add a new password for a specific site or application.
    ```bash
    cargo run -- create <"site/app">
    ```
    To force editing, you can run: 
    ```bash
     cargo run -- create <"site/app"> true
    ```

- **Edit Passwords:** Modify the password for an existing site or application.
    ```bash
    cargo run -- edit <"site/app">
    ```

- **Generate Secure Passwords:** Automatically generate a secure password.
    ```bash
    cargo run -- generate
    ```

- **List Sites:** View a comprehensive list of sites and applications with stored passwords.
    ```bash
    cargo run -- list
    ```

Follow the on-screen instructions or use the commands mentioned above to interact with the password manager.



## Contributing

Feel free to contribute by opening issues or submitting pull requests. Your feedback and suggestions are valuable.

Enjoy managing your passwords with pwd-crabager! THIS IS NOT SECURE, NO ENCRYPTION, JUST FOR LEARNING! 🦀🔐 🦀🔐

