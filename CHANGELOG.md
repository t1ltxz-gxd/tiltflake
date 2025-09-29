# Changelog

All notable changes to this project will be documented in this file.

## [0.3.0](https://github.com/t1ltxz-gxd/tiltflake/compare/v0.2.1..0.3.0) - 2025-09-29

### <!-- 0 -->🚀 Features

- Update allowed licenses by removing unnecessary entries. - ([2ed9790](https://github.com/t1ltxz-gxd/tiltflake/commit/2ed9790902a775fe05c081c8cc85fa227cfa773c))
- Install `cargo-deny` and add audit checks in CI workflow. - ([9328e5a](https://github.com/t1ltxz-gxd/tiltflake/commit/9328e5a629843b0a70906456832e68c0951a62d1))
- Enhance CI workflow with new lint, security audit, and dependency checks jobs. - ([333afdf](https://github.com/t1ltxz-gxd/tiltflake/commit/333afdff7ccb994f2cd5cd7bbc7442b1961479a9))
- Add `from_u64` and `as_u64` methods to `TiltflakeId` for value conversion. - ([d93beac](https://github.com/t1ltxz-gxd/tiltflake/commit/d93beacdfd1ca15508bbf3302dbf2d9f1e8e1999))

### <!-- 2 -->🚜 Refactor

- Replace `MPL-2.0` with `Unicode-3.0` in allowed licenses. - ([9273165](https://github.com/t1ltxz-gxd/tiltflake/commit/9273165dc78a1b989bca1d32f52d7e895e9f74c0))
- Upgrade `chrono` dependency from version 0.4.40 to 0.4.41. - ([14f2dab](https://github.com/t1ltxz-gxd/tiltflake/commit/14f2dabbfed0db01e16461ec13e72b95a679e9eb))

### <!-- 7 -->⚙️ Miscellaneous Tasks

- Update conditions for installing cargo-deny and running cargo deny check. - ([769af08](https://github.com/t1ltxz-gxd/tiltflake/commit/769af08126a83385275d414de59d2befd7d5557f))
- Upgrade GitHub actions for Rust setup and change checkout action version. - ([96543cd](https://github.com/t1ltxz-gxd/tiltflake/commit/96543cd743fbd794e6b23c1d6971fdb0ce5094ad))
- Modify conditions for cargo-deny installation and update GitHub actions for Rust setup. - ([7d27f49](https://github.com/t1ltxz-gxd/tiltflake/commit/7d27f49d8a1deaccfd4e87156e449ad6e2908118))

## [0.2.1](https://github.com/t1ltxz-gxd/tiltflake/compare/v0.2.0..v0.2.1) - 2025-04-09

### <!-- 2 -->🚜 Refactor

- Simplify public module imports for `epoch` and `flake`. - ([ec4ac89](https://github.com/t1ltxz-gxd/tiltflake/commit/ec4ac89b774319e50ce28f6d6e9509fd54b7c56a))
- Remove unnecessary attribute for `serde` feature. - ([bcb15da](https://github.com/t1ltxz-gxd/tiltflake/commit/bcb15dae416d4a7bb698e7225386f849edf7f7f8))

### <!-- 7 -->⚙️ Miscellaneous Tasks

- Version 0.2.1 - ([d31a8ba](https://github.com/t1ltxz-gxd/tiltflake/commit/d31a8ba34d748d1496ff61b08b9f0b8567a21049))

## [0.2.0](https://github.com/t1ltxz-gxd/tiltflake/compare/v0.1.3..v0.2.0) - 2025-04-09

### <!-- 0 -->🚀 Features

- Expose `epoch` and `flake` modules for unique identifier management. - ([c1224b6](https://github.com/t1ltxz-gxd/tiltflake/commit/c1224b6c9fd3f1b27d9dd8de8fd28f92e5369e17))

### <!-- 2 -->🚜 Refactor

- Simplify import statements for `EpochType` and `Tiltflake` across multiple files. - ([457f612](https://github.com/t1ltxz-gxd/tiltflake/commit/457f6120bfd2278177d48eb4f5851e977c110bb1))

### <!-- 3 -->📚 Documentation

- Update Rust import statements for clarity in snowflake ID generation examples. - ([d3e739c](https://github.com/t1ltxz-gxd/tiltflake/commit/d3e739c5161073f09eba8dd9768475e36bf49114))

### <!-- 7 -->⚙️ Miscellaneous Tasks

- Version 0.2.0 - ([c6762a0](https://github.com/t1ltxz-gxd/tiltflake/commit/c6762a0376d1f92852bd14ca08f2ba3d793cc869))

## [0.1.3](https://github.com/t1ltxz-gxd/tiltflake/compare/v0.1.2..v0.1.3) - 2025-04-09

### <!-- 0 -->🚀 Features

- Add serde feature conditional documentation attribute. - ([be9e227](https://github.com/t1ltxz-gxd/tiltflake/commit/be9e227d8ef0474c96899f84bdc4bd4e3f80fb0a))

### <!-- 7 -->⚙️ Miscellaneous Tasks

- Configure package metadata for docs.rs with serde features. - ([48defd2](https://github.com/t1ltxz-gxd/tiltflake/commit/48defd2ec08a28b4025daf480e34c7262bfeb918))
- Version 0.1.3 - ([d0c00cf](https://github.com/t1ltxz-gxd/tiltflake/commit/d0c00cf7251f78885e0ad90330ca528c471fd7fd))

## [0.1.2](https://github.com/t1ltxz-gxd/tiltflake/compare/v0.1.1..v0.1.2) - 2025-04-09

### <!-- 7 -->⚙️ Miscellaneous Tasks

- Add documentation URL for the Tiltflake project. - ([e909601](https://github.com/t1ltxz-gxd/tiltflake/commit/e90960123275566e5f999bfb330b9b14f718b92f))
- Version 0.1.2 - ([a56093d](https://github.com/t1ltxz-gxd/tiltflake/commit/a56093d868f6012b42e96d8a35c30a7029d2fa41))

## [0.1.1](https://github.com/t1ltxz-gxd/tiltflake/compare/v0.1.0..v0.1.1) - 2025-04-09

### <!-- 0 -->🚀 Features

- Add HTML logo and favicon URLs to documentation. - ([753468a](https://github.com/t1ltxz-gxd/tiltflake/commit/753468a661817d4aba189e8dbf34a9211894e4ba))

### <!-- 2 -->🚜 Refactor

- Update `TiltflakeError` to derive `Eq` and `PartialEq` for improved error comparison. - ([9c5df67](https://github.com/t1ltxz-gxd/tiltflake/commit/9c5df67bfd673ee9903f8d7a373dc6e6b1c5cef0))

### <!-- 3 -->📚 Documentation

- Fix badge links for forks and stars in the README. - ([55296df](https://github.com/t1ltxz-gxd/tiltflake/commit/55296df157ef15281a0e36c65c3d31395e156095))

### <!-- 7 -->⚙️ Miscellaneous Tasks

- Rename preview image. - ([d89e419](https://github.com/t1ltxz-gxd/tiltflake/commit/d89e419fb62df3eff66430dc1bba9a4c4192c864))
- Version 0.1.1 - ([5594f31](https://github.com/t1ltxz-gxd/tiltflake/commit/5594f3129767ec7ad4a738488cd958f8a8aad92f))

## 0.1.0 - 2025-04-08

### <!-- 7 -->⚙️ Miscellaneous Tasks

- Upgrade Rust version to 1.81.0 in CI configuration. - ([3d8092b](https://github.com/t1ltxz-gxd/tiltflake/commit/3d8092b0e332261049249a81105df5977eb8dbdd))
- Update Rust edition to 2024 and set Rust version to 1.81.0. - ([478ccb0](https://github.com/t1ltxz-gxd/tiltflake/commit/478ccb0ce1a54bbb7fd7b877132bcf734bc78e65))
- Downgrade Rust edition from '2024' to '2021'. - ([4ebb6d9](https://github.com/t1ltxz-gxd/tiltflake/commit/4ebb6d969d92da1ebebf990c3d6bb879d749cc39))
- Add Clippy installation step for Rust version 1.81.0 in CI - ([701a941](https://github.com/t1ltxz-gxd/tiltflake/commit/701a9410fd1c2fb89c424fd85a7d00ca386726f6))
- Add Clippy installation steps for Windows, Ubuntu, and macOS. - ([4ed2604](https://github.com/t1ltxz-gxd/tiltflake/commit/4ed2604b66ef2d7c481fee1a94dac99978fe01db))
- Update Clippy installation for Apple Silicon in CI workflow. - ([cc75994](https://github.com/t1ltxz-gxd/tiltflake/commit/cc75994f16ced79b46bd2cd7e16312bf1c9ca6a4))
- Add initial changelog with notable changes for project version 0.1.0. - ([5fc881b](https://github.com/t1ltxz-gxd/tiltflake/commit/5fc881b9f07b0598365b446de30725236570f0cb))
- Add MIT License to the project. - ([b803eb8](https://github.com/t1ltxz-gxd/tiltflake/commit/b803eb88600ad2c29461c387f68e78e7e6b254ac))
- Update repository URL to include the HTTPS protocol. - ([c50352c](https://github.com/t1ltxz-gxd/tiltflake/commit/c50352c23b6a87276c221c152b80fb0b0ad62b75))
- Version 0.1.0 - ([218b955](https://github.com/t1ltxz-gxd/tiltflake/commit/218b95526973e69e0cf6d7ba2a714b6699d1ac1b))

<!-- generated by git-cliff -->
