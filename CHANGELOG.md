# Changelog

## [0.1.0] - 2024-04-20

### Features

- change port to 8537, write readme, get + post request ([df2657c](https://github.com/tippfehlr/composehook/commit/df2657c6fc037c1ca2ae197d74ac03870ab827eb))
- allow only one concurrent update ([1525103](https://github.com/tippfehlr/composehook/commit/15251038ea36497bf1526d53a66fc59302138b33))
- timeout of 10 seconds ([57567b9](https://github.com/tippfehlr/composehook/commit/57567b9029f97ac6dab840e7e1d0c209435933fd))
- set TIMEOUT as env var, default to 10 seconds ([f3937e4](https://github.com/tippfehlr/composehook/commit/f3937e4b0789c650bcfd4bcca7f4ce6265887df0))
- add simple info at / ([aa2907b](https://github.com/tippfehlr/composehook/commit/aa2907b2500fff1ebfa46f9e0ae9e94f7aa41ff9))

### Bug Fixes

- change port in code ([a1a0080](https://github.com/tippfehlr/composehook/commit/a1a008007a1124205ab3187e8f16e6431c5c1bda))
- wrap HashMap in Mutex ([5cad92e](https://github.com/tippfehlr/composehook/commit/5cad92e2c318e8784d556278a838aca7adfc52e3))
- use Arc to mutate the same Hashmap ([78ea4d9](https://github.com/tippfehlr/composehook/commit/78ea4d9eef08250b4fe15346da1591f506b3192e))

### Performance

- remove extra check for update since assigning returns the old value ([57474f7](https://github.com/tippfehlr/composehook/commit/57474f702a1f27f5f5a9a2e0feae60ed5e4b692e))

### Documentation

- explain the port number ([e4bedee](https://github.com/tippfehlr/composehook/commit/e4bedeedf7ee4076b5dd3052303b7118fcce1dd2))
- explain details and limit to post requests ([ca71d35](https://github.com/tippfehlr/composehook/commit/ca71d351e702e64f1b5fe9c9b4c3329bef543644))

<!-- generated by git-cliff -->
