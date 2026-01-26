# Changelog

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

---
## [0.2.0](https://github.com/inblossoms/rox/compare/v0.1.0..v0.2.0) - 2026-01-26

### Bug Fixes

- **(.gitignore)** resolve merge conflict and clean up ignored files - ([d1bd88a](https://github.com/inblossoms/rox/commit/d1bd88ae703766c0b29e040c21485d3390f8ceba)) - inblossoms
- **(pre-commit)** correct cargo deny command order - ([236cfeb](https://github.com/inblossoms/rox/commit/236cfebbd921fc3653f4141581c799b0eb3fdf92)) - inblossoms
- add dead_code allowance for unused Error struct - ([18db84f](https://github.com/inblossoms/rox/commit/18db84fbbf7fbc5d88eb8a46e4dca1a387de44d2)) - inblossoms

### Documentation

- **(README)** update build status badge to CI workflow - ([a458abb](https://github.com/inblossoms/rox/commit/a458abb775b6ccd050151823e025cb3ad541e013)) - inblossoms
- **(interpreter)** updated some comments. - ([13c5552](https://github.com/inblossoms/rox/commit/13c555201468809b10898e943d55066903e87028)) - inblossoms
- **(readme)** enhance documentation with comprehensive language features overview - ([1ad5cd1](https://github.com/inblossoms/rox/commit/1ad5cd14c791bf2149b16a4049ca1f66926d719c)) - inblossoms
- add contribution guides and setup scripts - ([961eb8e](https://github.com/inblossoms/rox/commit/961eb8ec1c09ed61fc67a8a370b033738d6b363a)) - inblossoms
- add contribution guide and setup scripts - ([c943f57](https://github.com/inblossoms/rox/commit/c943f57c531e48bc45e4a1acac5782a91a13097d)) - inblossoms
- update CONTRIBUTING.md and README.md for better navigation - ([b59972c](https://github.com/inblossoms/rox/commit/b59972c6f0460e72dab044ed01349d404dc88b17)) - inblossoms
- add contribution guide and setup scripts - ([c8f5ea7](https://github.com/inblossoms/rox/commit/c8f5ea7ee466a1d309d3912376bd474515cafa1e)) - inblossoms

### Features

- **(ast)** add support for array/object indexing expressions - ([7a069c3](https://github.com/inblossoms/rox/commit/7a069c310a1b6e1a1eca1531ef691089136ff051)) - inblossoms
- **(dict)** add dictionary methods implementation - ([3ff6625](https://github.com/inblossoms/rox/commit/3ff6625a83d990d66cf83742fb6897cdf2439687)) - inblossoms
- **(evaluate)** add string concatenation support for mixed types - ([dc555f8](https://github.com/inblossoms/rox/commit/dc555f8d2d1d35d7d5efc54db1dd1835cee30869)) - inblossoms
- **(interpreter)** add global native functions support - ([79b0150](https://github.com/inblossoms/rox/commit/79b01508c5faa9996bad2a1674c0888517716c72)) - inblossoms
- **(interpreter)** enhance property access for dictionary objects - ([f40007e](https://github.com/inblossoms/rox/commit/f40007eff8dcefa6bcd836c207f262935c9b5cfa)) - inblossoms
- **(interpreter)** add module system with import functionality - ([5914721](https://github.com/inblossoms/rox/commit/5914721f11e6379c968e92b81928d4b2ea88871d)) - inblossoms
- **(list)** add map and filter methods to list prototype - ([991a3af](https://github.com/inblossoms/rox/commit/991a3af5702e04a384e5f9f01251e4b5b5ead7f3)) - inblossoms
- **(parser)** add support for list and dictionary literals - ([8a034d1](https://github.com/inblossoms/rox/commit/8a034d149ea20e4758c60124d8ed7fbb6a2a7862)) - inblossoms
- **(parser)** add support for dictionary and list parsing - ([e0a7d71](https://github.com/inblossoms/rox/commit/e0a7d7101fc1c2169ef3853e244eebc8a66564f7)) - inblossoms
- **(repl)** add readline support with history and error handling - ([8838ae1](https://github.com/inblossoms/rox/commit/8838ae15088b5b182e08f07e1bddbd0d7b3f043d)) - inblossoms
- **(std-lib)** add file system module - ([5978601](https://github.com/inblossoms/rox/commit/59786011e71ba2b354b57322d6def90b2b721e96)) - inblossoms
- **(std_lib)** implement standard library method lookup system - ([f320147](https://github.com/inblossoms/rox/commit/f320147c082bc8ad18843c215023855acb27ba15)) - inblossoms
- **(std_lib)** add utility functions for type checking - ([ad3d3da](https://github.com/inblossoms/rox/commit/ad3d3da7787085e74df388b9937996be3adea8b3)) - inblossoms
- **(std_lib)** add new list methods and improve existing ones - ([cdab4a5](https://github.com/inblossoms/rox/commit/cdab4a5d8e6b25742637951828ed9dc02b87df9f)) - inblossoms
- **(string)** add new string methods and improve existing implementation - ([5e2f67f](https://github.com/inblossoms/rox/commit/5e2f67f091ea959a12d08aa208fcf09c8b34df4a)) - inblossoms
- **(tokenizer)** extend scanner with bracket and colon tokens - ([03b9541](https://github.com/inblossoms/rox/commit/03b95411efc105c9c2d56461d6acc1844af0e6cb)) - inblossoms
- add new error types and None value type - ([73ddd79](https://github.com/inblossoms/rox/commit/73ddd79dd11337019919504b4b3351dce7dafb23)) - inblossoms
- add lambda expression support to the language - ([562acab](https://github.com/inblossoms/rox/commit/562acab7961ecf445f029877d261d02916e9853b)) - inblossoms
- add try-catch statement support to the language - ([093f503](https://github.com/inblossoms/rox/commit/093f50378da9014416c274e9e0b7484eef261fdb)) - inblossoms
- enhance module import path resolution and remove test file - ([2b4146b](https://github.com/inblossoms/rox/commit/2b4146b4a9a8790ddd3872df227aeb6a17c2fb2b)) - inblossoms
- add export statement support to Rox language - ([fd84de9](https://github.com/inblossoms/rox/commit/fd84de9a9c0e3f41ffd143a14de7166807e14085)) - inblossoms
- add math module with mathematical functions and constants - ([1ca1a68](https://github.com/inblossoms/rox/commit/1ca1a6840d00bb3b131ea4e8a806741a7f37dbad)) - inblossoms
- add throw statement support to the language - ([779c59f](https://github.com/inblossoms/rox/commit/779c59f3dadc8555d3dfb3452e00e82be6b768cf)) - inblossoms
- add codespan-reporting and termcolor dependencies - ([32bce2e](https://github.com/inblossoms/rox/commit/32bce2eb6755e487bb3d3e4c3042efeae9f956dd)) - inblossoms
- add diagnostic error reporting with syntax highlighting - ([da2f4c7](https://github.com/inblossoms/rox/commit/da2f4c7a1877d52f040c09c2cccbe71192b432b7)) - inblossoms

### Miscellaneous Chores

- **(arch)** setup directory structure for standard library - ([b318810](https://github.com/inblossoms/rox/commit/b318810754c7c9061d4a691b96bdf4eb54b35182)) - inblossoms
- **(build)** update git ignore - ([85037e9](https://github.com/inblossoms/rox/commit/85037e91db4d0caf4d3ba47ebcfb3b434a7f1b74)) - inblossoms
- **(deps)** update cargo deny configuration - ([b38a648](https://github.com/inblossoms/rox/commit/b38a648978e91c44397e968eab89c3f176791b5c)) - inblossoms

### Other

- **(deps)** update deny.toml configuration - ([dee99f3](https://github.com/inblossoms/rox/commit/dee99f3dbb48bcb75689949f37977637858bfb0f)) - inblossoms
- update Cargo.lock and add dirs and rustyline dependencies - ([2c205e9](https://github.com/inblossoms/rox/commit/2c205e95e6db8e16de5989a43eb67d089efb50d3)) - inblossoms
- git commit -m "chore(release): prepare for v0.2.0" - ([c66f014](https://github.com/inblossoms/rox/commit/c66f01427d652ebea476d72a9044fd2849b978ca)) - inblossoms

### Refactoring

- **(evaluate/interpreter)** avoid long borrowing of RefCell in instance property lookup - ([0a49a38](https://github.com/inblossoms/rox/commit/0a49a38fb05e546d8a512936e104e9b3b4918974)) - inblossoms
- **(interpreter)** extract call_value method - ([1a8f7fb](https://github.com/inblossoms/rox/commit/1a8f7fb8eb5ad57bdbd8d64604dc8d1337489407)) - inblossoms
- **(mod)** export ScanError along with Error from tokenizer module - ([65d6628](https://github.com/inblossoms/rox/commit/65d6628d064b73738b963c9d13caa6fe1e7de9db)) - inblossoms
- **(std_lib)** update module structure and native function registration - ([ef185dc](https://github.com/inblossoms/rox/commit/ef185dc552a5aedc43f323d96d721bfc730683b9)) - inblossoms
- **(std_lib)** remove IO system library TODO comments - ([166a291](https://github.com/inblossoms/rox/commit/166a29181e535578f6417c981968a06e85b54371)) - inblossoms
- **(tests)** add pretty_assertions import for better test output - ([0a1596e](https://github.com/inblossoms/rox/commit/0a1596eb5514e450a03ae2ea02b76005b2d7fd03)) - inblossoms
- rename AST to Ast for consistency and clarity - ([50cca67](https://github.com/inblossoms/rox/commit/50cca67ad889aaec7ac4e7848e0d1f9ab589f71e)) - inblossoms

### Style

- address multiple Clippy lints across codebase - ([7760988](https://github.com/inblossoms/rox/commit/7760988bff2ef18a04b83669ecb0d82ecdd5a78b)) - inblossoms

### Tests

- **(ci)** update smoke test file extension from .lox to .rox - ([c56b8fb](https://github.com/inblossoms/rox/commit/c56b8fb8fccd427363f760091bd85dd3c2377e68)) - inblossoms

---
## [0.1.0] - 2026-01-09

### Bug Fixes

- **(tokenizer)** correct decimal number parsing logic - ([ef67d8b](https://github.com/inblossoms/rox/commit/ef67d8b245fa060615233ae48eeb4541330e0ad9)) - inblossoms
- **(tokenizer)** fix logical operators mapping - ([e76e622](https://github.com/inblossoms/rox/commit/e76e62285fb42774bc00ed585bb0e4b4d8afe6af)) - inblossoms
- **(tokenizer)** correct token types for keywords and operators - ([b154482](https://github.com/inblossoms/rox/commit/b1544823c6e1b09cd00362a586ddf0b0b7f12bb2)) - inblossoms
- Rename 'content' to 'contents' for consistency - ([524ba8a](https://github.com/inblossoms/rox/commit/524ba8ac9e53bed41270eb1a49fe42e2324e58de)) - inblossoms
- Update expression formatting for consistency and readability - ([60ba600](https://github.com/inblossoms/rox/commit/60ba600206d498ee8a649522f8d75af8db7e3413)) - inblossoms

### Documentation

- **(README)** enhance project description with detailed features - ([115bf0f](https://github.com/inblossoms/rox/commit/115bf0f2db01b8dff75668aef5809ba12271a869)) - inblossoms
- **(README)** format key features list - ([e9be9e7](https://github.com/inblossoms/rox/commit/e9be9e7e5260a6a6c6c6795c83f8135547d6d7b2)) - inblossoms
- **(README)** update documentation with detailed project overview - ([9978126](https://github.com/inblossoms/rox/commit/99781261c55fbaa3e003ed81ff5b2987c1f5c5c7)) - inblossoms
- **(environment)** add comprehensive documentation for variable operations - ([abb6c99](https://github.com/inblossoms/rox/commit/abb6c999b279a4e80c1271d0f62ed699d544d8c8)) - inblossoms
- **(interpreter)** add documentation for Resolver and Side Table relationship - ([27fa377](https://github.com/inblossoms/rox/commit/27fa37772099460e90471efe6bf125410a677175)) - inblossoms
- **(interpreter)** add detailed comments explaining dual environment pointers and side table pattern - ([5111590](https://github.com/inblossoms/rox/commit/51115907413a684289c1e1e932526d055ba30ff7)) - inblossoms
- **(parser)** add comprehensive documentation for parser methods - ([ff95c88](https://github.com/inblossoms/rox/commit/ff95c8880fcd4f2811a75e5fe86c9e251a0d4536)) - inblossoms

### Features

- **(ast)** implement for loop formatting in AST formatter - ([ce08235](https://github.com/inblossoms/rox/commit/ce082356feec3ee32078b4ccdee8d2d4c0cb1188)) - inblossoms
- **(ast)** refactor AST structure with separate statement and expression types - ([9707411](https://github.com/inblossoms/rox/commit/970741157ac8f371378be8f4260fcf657708204b)) - inblossoms
- **(evaluator)** decision logic for updating truth value of is_truthful - ([854d1bd](https://github.com/inblossoms/rox/commit/854d1bd55d8a0472b2364544507aac3a7f646a2c)) - inblossoms
- **(interpreter)** enhance main function with proper error handling - ([db15fbd](https://github.com/inblossoms/rox/commit/db15fbd422357c58856c7f1ee841ea6a33fe2f13)) - inblossoms
- **(interpreter)** modify print statement behavior - ([5d1d1b1](https://github.com/inblossoms/rox/commit/5d1d1b1718ed8853ba50b07543621cb7eea4907a)) - inblossoms
- **(interpreter)** add resolver module and improve REPL functionality - ([268f490](https://github.com/inblossoms/rox/commit/268f4904febe4a391fc3c83b4b9b18645613c39c)) - inblossoms
- **(interpreter)** implement class instance initialization with init method support - ([c78e91b](https://github.com/inblossoms/rox/commit/c78e91bf15de3143bb21b66a8b1a113f8b63bfa0)) - inblossoms
- **(lang)** add class inheritance support with super keyword - ([54d8e88](https://github.com/inblossoms/rox/commit/54d8e8866fe06349625c48b8cd16014da166e851)) - inblossoms
- **(parser)** implement comprehensive expression and statement parsing - ([65c0bf6](https://github.com/inblossoms/rox/commit/65c0bf6ef0fe329d9df2beb97061090ccbc86598)) - inblossoms
- **(parser)** add bitwise operators support with proper precedence - ([19be9d7](https://github.com/inblossoms/rox/commit/19be9d7e33882c79caac8b78dbd8da3666229e59)) - inblossoms
- **(parser)** implement for loop statement parsing and execution - ([9ef1a24](https://github.com/inblossoms/rox/commit/9ef1a24a9e50efd266508fee959c032d66a62149)) - inblossoms
- **(parser)** add loop depth tracking for break/continue validation - ([61805f3](https://github.com/inblossoms/rox/commit/61805f3d9ca9c6b8ad960829e281999f1cd62148)) - inblossoms
- **(parser)** add return statement support with proper token tracking - ([64bd034](https://github.com/inblossoms/rox/commit/64bd03414499974abb3791d9970bbeb75a9644d5)) - inblossoms
- **(parser)** add support for keyword operators and improve token mapping - ([c457c3e](https://github.com/inblossoms/rox/commit/c457c3e93b08597d1c51367aaafdae48b4cd12aa)) - inblossoms
- **(parser)** enhance expression parsing with unique IDs and statement separation - ([2182238](https://github.com/inblossoms/rox/commit/2182238cc4f67238840efa54ac5ea3a74c80d40a)) - inblossoms
- **(reader)** add file extension validation - ([273449a](https://github.com/inblossoms/rox/commit/273449ad0f0d8480e81dbc720b8a473295b0cc41)) - inblossoms
- **(tests)** update return statement test with token information - ([40fa3eb](https://github.com/inblossoms/rox/commit/40fa3eb08c886a7f87fd06092430a47785703f0c)) - inblossoms
- **(tokenizer)** add new literal types to support complex data structures - ([4f3e4fd](https://github.com/inblossoms/rox/commit/4f3e4fd6fa7f721d94e28fbcabe56e22a229a297)) - inblossoms
- Some error improvements - ([9a05e92](https://github.com/inblossoms/rox/commit/9a05e92d448deecc480cc1a34b5667eebbd12fe2)) - inblossoms
- Support '+=' syntax - ([e9f4753](https://github.com/inblossoms/rox/commit/e9f4753a09a32c909afe01c5e7386cb10028e01e)) - inblossoms
- implement interpreter with environment and value system - ([ae2e1af](https://github.com/inblossoms/rox/commit/ae2e1af7d4fd6c77eaea194e05354fdbc08d025b)) - inblossoms
- add variable declaration and compound assignment operators support - ([962f115](https://github.com/inblossoms/rox/commit/962f11573068efb27612f04a0ee0c0b0b106966d)) - inblossoms
- add support for for loops, break, continue, and print statements - ([89326bb](https://github.com/inblossoms/rox/commit/89326bbd5447eb85c03b1ac878975f353ce54020)) - inblossoms
- add comprehensive test cases - ([5651c53](https://github.com/inblossoms/rox/commit/5651c53a71de2c705051a1fb4d058a1337259f63)) - inblossoms
- add bitwise XOR operator support - ([527b794](https://github.com/inblossoms/rox/commit/527b7949363968cda31fdc47f6ee303c808a051f)) - inblossoms
- add modulo operator support - ([0a2494c](https://github.com/inblossoms/rox/commit/0a2494c146a462180302ebeacda8aea95c11b89d)) - inblossoms
- add comprehensive error handling system - ([ee8aa7d](https://github.com/inblossoms/rox/commit/ee8aa7d4a5aae1a1ba71e5f26f7557c07437789c)) - inblossoms
- add resolver module for semantic analysis - ([395d4d4](https://github.com/inblossoms/rox/commit/395d4d4782c442d8cfff673c4009a3808445a255)) - inblossoms
- add class declaration and instantiation support - ([17008c9](https://github.com/inblossoms/rox/commit/17008c9266ccab0ef792fed14f52c95f615b7870)) - inblossoms
- add object property access and assignment support - ([f1675d3](https://github.com/inblossoms/rox/commit/f1675d331275eede033f2b85398afc66f5aa190a)) - inblossoms
- add 'this' keyword support and method binding functionality - ([6148771](https://github.com/inblossoms/rox/commit/6148771ba85d25682a594a79fa4367b4595f178d)) - inblossoms

### Miscellaneous Chores

- **(deps)** add pretty_assertions and related dependencies - ([b079f9c](https://github.com/inblossoms/rox/commit/b079f9cfe88409970837910d99ee856a7f5d8f0c)) - inblossoms
- **(project)** rename project from lox to rox - ([3f4cadb](https://github.com/inblossoms/rox/commit/3f4cadbb13fd5ac59175fe2b7986d106932d2a75)) - inblossoms
- **(project)** add initial project files and configuration - ([fe1fafc](https://github.com/inblossoms/rox/commit/fe1fafc37f82d4f56698e4ed898d2a0f41bb4cff)) - inblossoms
- **(project)** update project metadata to Cargo.toml - ([d9cad1e](https://github.com/inblossoms/rox/commit/d9cad1e87350d5505212ac354ef56d4d69a7bf28)) - inblossoms
- Starting out - ([7e26b8c](https://github.com/inblossoms/rox/commit/7e26b8c2c298e0281aad25a7e5b13580424194f2)) - inblossoms
- Initialize project - ([7d9019f](https://github.com/inblossoms/rox/commit/7d9019f691605307d953dccb188c4b419a9a7812)) - inblossoms
- Working in progress - ([26c6e0a](https://github.com/inblossoms/rox/commit/26c6e0a11d2b79cbcfbeeb3ce25a5bf12288259e)) - inblossoms
- Added some test stubs - ([d4c917d](https://github.com/inblossoms/rox/commit/d4c917de1b7232e8b03742e41668caa289793d0d)) - inblossoms
- Stubbed err handling - ([f7d7f16](https://github.com/inblossoms/rox/commit/f7d7f16a8ac73942dfacfd2f5edce9b8cdf0fa11)) - inblossoms
- Turned errors into structs/enums - ([b67c3af](https://github.com/inblossoms/rox/commit/b67c3af92e3298fcf81af6ded7c214618e21b572)) - inblossoms
- Work on errors - ([9b167ab](https://github.com/inblossoms/rox/commit/9b167ab5b3ce3b1522364dd1f0c656f16bf34c01)) - inblossoms
- Added REPL loop - ([7e4a939](https://github.com/inblossoms/rox/commit/7e4a939e2992d0ddbae636cb6235487ee53ca96d)) - inblossoms
- Some initial work on scanning - ([6f2c917](https://github.com/inblossoms/rox/commit/6f2c9176ea8b20aee4b3a1c3eea958a796729f01)) - inblossoms
- Work in progress - ([b994d79](https://github.com/inblossoms/rox/commit/b994d798df92149378edc7edba0f59e1ab54b4d0)) - inblossoms
- Refactored the scanner and fixed the issue of pointer not advancing after peek - ([6b0a9b5](https://github.com/inblossoms/rox/commit/6b0a9b556914b720a3865a9f67f8ea4ec1d3e321)) - inblossoms
- Enhance tokenizer to recognize whitespace characters as tokens - ([4219ac3](https://github.com/inblossoms/rox/commit/4219ac3ee31554d4226943f88720f5ed7d5f6332)) - inblossoms
- Refactor scanner methods for clarity - ([69d7a60](https://github.com/inblossoms/rox/commit/69d7a6032178379e671f7826a3936e0245317191)) - inblossoms
- Remove unused method - ([1f74105](https://github.com/inblossoms/rox/commit/1f74105ce643b27d800e66ca3ffb712c84819082)) - inblossoms
- Rename scanner methods for consistency and clarity - ([297e2e2](https://github.com/inblossoms/rox/commit/297e2e2e8f48eaaa63426c6e301c764dba8c2c88)) - inblossoms
- Rename scanner handling methods for clarity and consistency - ([d4331a5](https://github.com/inblossoms/rox/commit/d4331a5957ed861b76ca3c946bc988671540a716)) - inblossoms
- Improve comments for clarity - ([1bb1d7f](https://github.com/inblossoms/rox/commit/1bb1d7f27edd28fb13a4e773aadf86b91b80614e)) - inblossoms
- Implement AST structure and expression formatting - ([0612fe0](https://github.com/inblossoms/rox/commit/0612fe023c1d536419880af278805ecab3f6db74)) - inblossoms
- add CI/CD workflows and development tools - ([efb241a](https://github.com/inblossoms/rox/commit/efb241a095264dcf62c744551a53c2df6e32dc2b)) - inblossoms

### Other

- **(workflow)** rename workflow directory to workflows - ([9a7b7b2](https://github.com/inblossoms/rox/commit/9a7b7b2264a53dcb4aa79685b6ad7a06b81c744d)) - inblossoms
- Work in progress - ([888fa25](https://github.com/inblossoms/rox/commit/888fa257c2845f58a0aeea57b6fbfd897c092bc3)) - inblossoms
- Initial commit - ([610cb36](https://github.com/inblossoms/rox/commit/610cb3692f049f4a4d89c41538ccbc724f067512)) - Ray

### Refactoring

- **(ast)** move tests to separate modules - ([41c4ed1](https://github.com/inblossoms/rox/commit/41c4ed1f000b85ff748072c54a15a339f43d7847)) - inblossoms
- **(ast)** restructure AST module into separate files - ([d1b77f7](https://github.com/inblossoms/rox/commit/d1b77f74164b5005920314f7a60843113061302d)) - inblossoms
- **(environment)** update doc comments for Environment struct - ([4343129](https://github.com/inblossoms/rox/commit/4343129292b9de621132be930b5f17674a27e25d)) - inblossoms
- **(environment, value)** consolidate use statements for clarity - ([2616169](https://github.com/inblossoms/rox/commit/2616169967b1427b69b1eb1074b55265d77b780e)) - inblossoms
- **(evaluate)** Improved interpreter behavior through integration with Resolver static parser - ([4cd58cd](https://github.com/inblossoms/rox/commit/4cd58cd0a48f69581c358f5489f0ffa5ab690214)) - inblossoms
- **(interpreter)** move interpreter instance to outer scope for context consistency - ([d4336aa](https://github.com/inblossoms/rox/commit/d4336aa8afb43b30e8720d91eb7a14f23fbcbc2f)) - inblossoms
- **(interpreter)** restructure imports and add comprehensive documentation - ([0605872](https://github.com/inblossoms/rox/commit/06058722b9c1eaa7da919d043d9674e1ecbc9651)) - inblossoms
- **(interpreter)** remove redundant error documentation and comments - ([7fdd757](https://github.com/inblossoms/rox/commit/7fdd757cd012272e4c2e4ef5591013d93c2114bd)) - inblossoms
- **(interpreter)** rename variable scope distance table to side table - ([11ac134](https://github.com/inblossoms/rox/commit/11ac13410e4d580f382c59f5ca4fc1f86cec856e)) - inblossoms
- **(parser)** remove unused imports and add dead code allowance - ([5022000](https://github.com/inblossoms/rox/commit/5022000e3e7ffcebc58707e49a2ed7340640e349)) - inblossoms
- **(parser)** move compound statement parsing to separate module - ([1b9e66f](https://github.com/inblossoms/rox/commit/1b9e66f840a6e8ea3037cc4b96e2f3f73ef3f792)) - inblossoms
- **(parser)** restructure test modules and improve test infrastructure - ([67925b0](https://github.com/inblossoms/rox/commit/67925b0d3488e72036c75dfee48a861c7668d65c)) - inblossoms
- **(parser)** add comments to ParseHelper struct fields - ([667395a](https://github.com/inblossoms/rox/commit/667395a7c45825826cbeea0a9ac15b174f59322d)) - inblossoms
- **(reader)** remove unnecessary dead code annotations - ([6d88cdf](https://github.com/inblossoms/rox/commit/6d88cdf2d928da9d55e974cb04c858e904b4608c)) - inblossoms
- **(reader)** restructure reader module - ([f58baf9](https://github.com/inblossoms/rox/commit/f58baf97053027381f8057eabf3e812faf35aa70)) - inblossoms
- **(reader)** remove unused test module - ([103f388](https://github.com/inblossoms/rox/commit/103f388b239d307cd617856262e33b081d8b3b53)) - inblossoms
- **(resolver)** simplify comments - ([61cdc2b](https://github.com/inblossoms/rox/commit/61cdc2b3beb4096d1ab3f8523866825924356687)) - inblossoms
- **(tests)** update logical operator enum variants in test cases - ([f9ae96c](https://github.com/inblossoms/rox/commit/f9ae96cd1307ae0b9c3f57304c17b5cb37b3ade3)) - inblossoms
- **(tests)** remove obsolete test files - ([34a7e18](https://github.com/inblossoms/rox/commit/34a7e181c0475be3252ff54b7f318b14bbb1949d)) - inblossoms
- **(tests)** improve test helper documentation and cleanup - ([711e58c](https://github.com/inblossoms/rox/commit/711e58c2abc3ed133fffcb4d87306e1e796167c9)) - inblossoms
- **(tokenizer)** extract block comment scanning logic - ([354d773](https://github.com/inblossoms/rox/commit/354d77393f7960dc4da2bfc8ef5d1f04b797b1cc)) - inblossoms
- **(tokenizer)** restructure tokenizer module - ([cf2e1b6](https://github.com/inblossoms/rox/commit/cf2e1b6c5c54c2af2e1ea6ffc80b0c73d08db92b)) - inblossoms
- **(tokenizer)** move Tokens struct to token module and add Display impl - ([c2d27fe](https://github.com/inblossoms/rox/commit/c2d27fee52571251dcf3ecbfa1d0d06c821cc513)) - inblossoms
- Added test stubs and err stubs - ([812baef](https://github.com/inblossoms/rox/commit/812baef6022371f44082172b857207897dfc967e)) - inblossoms
- Changed output types to proper structs - ([c5ebf77](https://github.com/inblossoms/rox/commit/c5ebf778fd7ae4ff9374c13353430b282f7ade87)) - inblossoms
- Simplify lexeme extraction in Scanner - ([99706a9](https://github.com/inblossoms/rox/commit/99706a9449e8ab864e3186557b15e4e91e45266d)) - inblossoms
- Simplify whitespace processing - ([dc5fcc6](https://github.com/inblossoms/rox/commit/dc5fcc6371428ca9a3d624625d1bf8e18b0b1c5b)) - inblossoms
- Replace Token with Operator in expression handling and update number type to String - ([1818932](https://github.com/inblossoms/rox/commit/181893223fda479e382e3b282f606b110f07e28b)) - inblossoms
- Enhance error handling for unterminated strings - ([8341ad7](https://github.com/inblossoms/rox/commit/8341ad79ec07f37703c63616b791a569fad69922)) - inblossoms
- Correct struct naming to Output - ([190f2f9](https://github.com/inblossoms/rox/commit/190f2f9b13cebae9f5c79154fb8a9f0c3eb1857d)) - inblossoms

### Tests

- **(class)** add tests for class property access and undefined properties - ([32c2cd5](https://github.com/inblossoms/rox/commit/32c2cd5b4ffbddf0a387286c2b51d266a92cf90a)) - inblossoms
- **(evaluate)** add class instantiation tests - ([80e7439](https://github.com/inblossoms/rox/commit/80e743977cf8f2fb9b323358a3cb70a58055cae4)) - inblossoms
- **(evaluate)** add class method binding and field access tests - ([45c9180](https://github.com/inblossoms/rox/commit/45c9180161e1f13be782bcc4338073d0df9651a8)) - inblossoms
- **(evaluate)** add comprehensive OOP functionality tests - ([eb19958](https://github.com/inblossoms/rox/commit/eb199588e300b2b2b75190634347073262c581e8)) - inblossoms
- **(evaluator)** add inheritance and super call test cases - ([f8af25d](https://github.com/inblossoms/rox/commit/f8af25d564a49221a6e38da0dbd6dcc617cb5c6a)) - inblossoms
- **(interpreter)** add comprehensive test suite for evaluation module - ([5e78606](https://github.com/inblossoms/rox/commit/5e78606decc0382b593018a5ff2f658f07af0924)) - inblossoms
- **(parser)** add comprehensive parser tests and reorganize test structure - ([acb09bd](https://github.com/inblossoms/rox/commit/acb09bddb4b361d023b8525b853ab59ff3de9b93)) - inblossoms
- **(tokenizer)** add comprehensive test suite for tokenizer functionality - ([cf63596](https://github.com/inblossoms/rox/commit/cf6359659ae35c0c5b4c4c97d2a78ffff380bbf1)) - inblossoms
- **(tokenizer)** add pretty_assertions for better test output - ([9374acb](https://github.com/inblossoms/rox/commit/9374acb3b50b392651795a1a3d8061675981e610)) - inblossoms
- Add tests for two_character、keywords、literals、whitespace handling in scanner - ([50d7300](https://github.com/inblossoms/rox/commit/50d7300f001c18c19b462d53d3e0a3b8b8e8d033)) - inblossoms
- Add identifier token for test case - ([281190e](https://github.com/inblossoms/rox/commit/281190ed4ecf1b38605f0d183128c98bea312e2a)) - inblossoms
- add comprehensive parser test suite - ([d942b76](https://github.com/inblossoms/rox/commit/d942b76c5dc279e82adb8435f8c76f7398d3de02)) - inblossoms
- add comprehensive test suite for evaluate module - ([341c0e6](https://github.com/inblossoms/rox/commit/341c0e69ef44ff5832579cc70d645d375ae67f1c)) - inblossoms

<!-- generated by git-cliff -->
