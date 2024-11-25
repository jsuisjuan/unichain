# UniChain

UniChain is a decentralized file management system built on blockchain technology, designed to store, view, update, and manage files in a secure and transparent manner. It leverages smart contracts for decentralized operations, ensuring data integrity and immutability, all while providing a simple CLI (Command Line Interface) for user interaction.

## Features

- **File Storage**: Securely store files on the blockchain.
- **File Viewing**: View files stored on the blockchain.
- **File Updates**: Update existing files and ensure their integrity.
- **File Deletion**: Move files to a trash bin, making it easy to manage your data.
- **Access Control**: Easily manage user access and permissions using blockchain technology.
- **User Management**: Uses system-level user identification (username and email) for a personalized experience.

## Prerequisites

- Rust (version 1.50 or higher)
- `cargo` (Rust’s package manager)
- A basic understanding of blockchain concepts and Rust programming

## Installation

### Step 1: Clone the repository

Clone the project repository to your local machine:

```bash
git clone https://github.com/yourusername/UniChain.git
cd UniChain
```

### Step 2: Build the project
Ensure you have Rust installed, and then use cargo to build the project:

```bash
cargo build --release
```

### Step 3: Run the application
After building, you can run the application with:

```bash
Copiar código
cargo run
```

This will start the command-line interface (CLI) where you can interact with the UniChain system.

### Usage
Once the application is running, you'll be greeted with a prompt that shows your username and email associated with the system. Then, the program will provide you with a menu of options:

- **View list of stored files**: Displays a list of all the files currently stored in the system.
- **View a specific file**: Allows you to view the contents of a specific file by ID.
- **Store a new file**: Upload a new file into the system.
- **Update an existing file**: Update a file already stored on the blockchain.
- **Move a file to trash**: Move a file to a "trash" or inactive state.
- **Exit**: Close the application.
You will be prompted to select an option, and the system will guide you through each of the tasks.

### Example Flow
On startup, the system displays your username and email, along with a menu of options.
- If you choose to store a new file, you will be prompted to upload the file’s ID and other details.
- If you decide to view a file or update it, you will provide the file ID.
- If you wish to exit the program, simply choose option 0.

### Error Handling
The application provides error handling at every step, ensuring that any invalid inputs (e.g., non-numeric file IDs, invalid actions) are caught and displayed to the user with a helpful message. The program continues running until you choose to exit.

### Common errors include:

- Invalid ID number for a file
- Incorrect option chosen in the menu
- File not found during operations
- Unexpected I/O errors

### Contributing
If you want to contribute to the project, feel free to fork the repository, create a new branch, and submit a pull request. We welcome improvements, new features, and bug fixes!

### Steps for Contributing:
- Fork the repository
- Create a new branch (`git checkout -b feature/your-feature`)
- Make your changes
- Commit your changes (`git commit -am 'Add your feature'`)
- Push to the branch (`git push origin feature/your-feature`)
- Open a pull request
### License
This project is licensed under the MIT License - see the LICENSE file for details.
