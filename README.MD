# Fit a line using linear least squares method
A program that will fit a line using the least squares method for the data points defined in a tab/comma separated file.
By default the program will load the datapoints.txt from current folder and use a tab separatar.

## Usage
Run the program with default arguments (datapoints.txt)
```sh
cargo build
cargo run
```

### Arguments
- filename (override path to data points file)
```sh
cargo run datapoints_file.txt
```

- separator (override default seperator of  data points file)
```sh
cargo run datapoints.txt -s ";"
```
