## image-server

Project to provide an image-server that allows to serve different images in pre-defined dimensions.


_Mostly a reason to learn and code some Rust_

## Development
### Prerequisites
#### Rust

This is a Rust-application, therefore Rust needs to be installed on your system to build and run this. See
[rustup.rs](https://rustup.rs/) to check out how you should install Rust on your local machine. 

#### Node.js
This project expects Node.js version v20.17.0 or higher. Use e.g. ode Version Manager (NVM) to install and manage
Node.js versions. Find out more about NVM at https://github.com/nvm-sh/nvm.

Install the dependencies:

```bash
npm install
```

#### Husky / Git hooks
I use [husky](https://typicode.github.io/husky/) to manage git-hooks. It will get automatically installed as part of
the `npm install` command and creates a `.husky` folder in the project root in which you can add more git-hooks if
needed. Check out the husky documentation for more information. Adjust for your package manager of choice if you are not
using npm.

### Development

To run the development server, use the following command:

```bash
cargo run
```

Open [http://localhost:8080](http://localhost:3000) with your browser to see the result.

## Deployment
Right now, this project is deployed to Vercel automatically via a hook in vercel which runs the `build` command whenever
there is a change in this repository. Since this is a Rust project, it can be hosted anywhere where else as long as
Rust environment is available.
