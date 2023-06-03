# faserve
Serve files easily (and blazingly fast!) using Rust

# Guide
Faserve accepts 2 optional arguments and requires one argument.

`--port`: Faserve's port (defaults to 8080)

`--host`: Faserve's host IP (defaults to 127.0.0.1)

`<FILE>`: Required argument, It's the file that Faserve is going to serve.

# Example usage
`faserve contents.zip`

`faserve --host 0.0.0.0 image.jpeg`
