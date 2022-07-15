![Riverbend High School](https://www.spotsylvania.k12.va.us/cms/lib/VA01918722/Centricity/Template/GlobalAssets/images///logos/RHS.png)

# **RHS Gabe VS Gavin** Rust Rocket REST API

![Rust 2021](https://img.shields.io/badge/Rust-2021-%232D44A4?style=flat)
![GPLv3 License](https://img.shields.io/badge/License-GPLv3-%232D44A4?style=flat)
![Version 0.1.0](https://img.shields.io/badge/Version-v0.1.0-%232D44A4?style=flat)

This repository contains the backend server code for a student voting system used by Gabe and Gavin on the Gabe and Gavin Show. This api is used to record votes on polls generated on the show.

This backend is running on Rust using [Rocket v0.5.0-rc1](https://rocket.rs/). It is designed to be used with the frontend component found in [the frontend component folder](/frontend). Thee frontend included with this api is implemented in Vue.

Created by [Gabriel Hogan](https://gabrielhogan.com) and [Samuel Rembisz](https://stappsworld.com).

## Installation

All commands assume that you are at the root of the `gabe_versus_gavin` repository that you cloned.

1. Ensure you have both [Cargo and Rustup](https://rustup.rs/) to date
   ```
   rustup update
   ```
2. You will also need to build the frontend located in [the frontend component folder](/frontend)

   ```
   npm i
   npm run build
   ```

3. Then move the dist folder to the root directory of the project and rename the folder to `static`

   ```
   mv dist/ ../static
   ```

4. From there, the program can be run
   ```
   cargo r --release
   ```

## Authentication

Authentication with the API is currently done through **randomly generated ID's** supplied by the server.
(More Info Soon)

## License

Distributed under the GPLv3 License. See [LICENSE](LICENSE) for more information.

## Contact

RHS Library - rhs@spotsylvania@k12.va.us
