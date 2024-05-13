
# Encosh
[![Apache License 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://choosealicense.com/licenses/apache-2.0/)

A fast flexible command executor written in pure Rust


## Features

- Base _encosh.yaml_
- Additional _encosh.yaml_ params
- Fullscreen mode
- Cross platform


## Usage/Examples

Encosh.yaml file example:
```yaml
  conf:
    commands: "dir > lsfile.md"
```
Then run the following commands depending on your operating system:

- Linux usage:
```bash
  grenka-finder
```
- Windows usage:
```bash
  grenka-finder.exe
```


## Deployment

To deploy this project run (only for binary file):

 - for Windows
```bash
  grenka-finder.exe
```

- for Linux
```bash
  grenka-finder
```


## Roadmap

- Base **encosh.yaml** implementation

- Additional **encosh.yaml** params

- Implementation of _default_ values

- Implementation of program startup parameters

- Progress displaying

- Display verbose information about the execution


## Authors

- [@Grenka](https://www.github.com/GRHead)


## License

[Apache License 2.0](https://choosealicense.com/licenses/apache-2.0/)

