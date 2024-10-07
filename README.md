# The Fetale Programing Language

Fetale is a new programming language with a simple and intuitive syntax. The language is easy to use for beginners and experienced programmers alike and is suitable for a wide variety of applications.

## Features.

- **Simple syntax**: programs are easy to understand and intuitive to write.
- **Field types**: All operations are based on fields and do not require function definitions.
- **JSON-like syntax**: Inspired by JSON, data representation is easy.

## Installation

To use Fetale, please follow the steps below to install it.

> [!WARNING]
> FETALE is under development and cannot be installed yet.<br>
> If you want to install it, please clone this repository and build it

1. clone the repository.

   Clone the repository.
   ```
   git clone https://github.com/yourusername/fetale.git
   ```
 2. Go to the directory.

   ```bash
   cd fetale
   ```
3. Build the program.

   Build the program: 
   ```bash
   cargo build --release
   ```

## How to use

The basic program is written as follows.

```fetale
# this is comment

field “main”
{
  $text = “hello fetale!”
  puts: $text
}
```

This program outputs ``hello fetale!``.

## Documentation

For detailed documentation, see [here](link_to_your_documentation).

## Contributions

Contributions to Fetale are welcome! We welcome bug reports, suggestions for new features, and pull requests.

1. fork it (``fork``)
2. create a branch (``branch``)
3. Create a branch (`git checkout -b feature/YourFeature`).
4. Commit your changes (`git commit -m 'Add some feature'`).
5. Push (`git push origin feature/YourFeature`).
6. Create a pull request.

## License

This project is licensed under the [MIT License](LICENSE).

---

We hope you enjoy your programming life with Fetale!
