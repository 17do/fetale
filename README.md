# The Fet Programming Language

Fet is a new programming language with a simple and intuitive syntax. It is easy to use for beginners and experienced programmers alike, making it suitable for a wide variety of applications.

## Features:

- **Simple syntax**: Programs are easy to understand and intuitive to write.
- **Field types**: All operations are based on fields and do not require function definitions.
- **JSON-like syntax**: Inspired by JSON, data representation is straightforward.

## Installation

To use Fetale, please follow the steps below to install it.

> [!WARNING]
> FET is currently under development and cannot be installed yet.<br>
> If you want to install it, please clone this repository and build it.


1. Clone the repository:
   ```bash
   git clone https://github.com/17do/fet.git
   ```
2. Go to the directory:
   ```bash
   cd fet
   ```
3. Build the program:
   ```bash
   cargo build --release
   ```

## How to Use

The basic program is written as follows:
> [!IMPORTANT]
> Note that the following syntax is still being created and may change


```fet
# this is a comment

field "main"
{
  $text = "hello fet!"
  puts: $text
}
```

This program outputs `hello fet!`.

## Contributions

Contributions to Fet are welcome! We welcome bug reports, suggestions for new features, and pull requests.

1. Fork it (`fork`).
2. Create a branch (`branch`).
3. Create a branch (`git checkout -b feature/YourFeature`).
4. Commit your changes (`git commit -m 'Add some feature'`).
5. Push (`git push origin feature/YourFeature`).
6. Create a pull request.

## License

This project is licensed under the [MIT License](LICENSE).

---

We hope you enjoy your programming journey with Fet!
