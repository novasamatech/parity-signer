fastlane documentation
----

# Installation

Make sure you have the latest version of the Xcode command line tools installed:

```sh
xcode-select --install
```

For _fastlane_ installation instructions, see [Installing _fastlane_](https://docs.fastlane.tools/#installing-fastlane)

# Available Actions

## iOS

### ios test_build

```sh
[bundle exec] fastlane ios test_build
```

Tests given scheme

Parameters:

- 'scheme : <value>' to define scheme to test

 

Example usage: fastlane test_build scheme:'PolkadotVault'

### ios build_release

```sh
[bundle exec] fastlane ios build_release
```

Build the iOS app for release

Parameters:

- 'scheme : <value>' defines scheme to use for build phase

- 'target : <value>' defines target to build

- 'configuration : <value>' defines configuration for build

 

Example usage: fastlane build_release scheme:'NativeSigner' target: 'NativeSigner' configuration: 'Release' 

### ios prepare_code_signing

```sh
[bundle exec] fastlane ios prepare_code_signing
```

Prepares certificate and provisioning profile

### ios upload_testflight

```sh
[bundle exec] fastlane ios upload_testflight
```

Submit a new build to Apple TestFlight

### ios load_asc_api_key

```sh
[bundle exec] fastlane ios load_asc_api_key
```

Load ASC API Key information to use in subsequent lanes

### ios run_unit_tests

```sh
[bundle exec] fastlane ios run_unit_tests
```

Runs unit tests for development scheme

Example usage: fastlane run_unit_tests

### ios distribute_production_testflight

```sh
[bundle exec] fastlane ios distribute_production_testflight
```

Distribute new iOS production build through TestFlight

Example usage: fastlane distribute_production_testflight

### ios distribute_qa_testflight

```sh
[bundle exec] fastlane ios distribute_qa_testflight
```

Distribute new iOS QA build through TestFlight

Example usage: fastlane distribute_qa_testflight

----

This README.md is auto-generated and will be re-generated every time [_fastlane_](https://fastlane.tools) is run.

More information about _fastlane_ can be found on [fastlane.tools](https://fastlane.tools).

The documentation of _fastlane_ can be found on [docs.fastlane.tools](https://docs.fastlane.tools).
