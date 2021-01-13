# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.6] - 2020-01-13
- Update deku to 0.10.0
- Use deku attributes for loading fspec
- Other code improvements

## [0.2.5] - 2020-11-06
- Update/Use deku 0.9.1

## [0.2.4] - 2020-10-03
- Update lib.rs example
- Add CI

## [0.2.3] - 2020-10-03
- Update deku: 0.8.0. Gives some speed improvements with less allocation for writing.

## [0.2.2] - 2020-09-27
- Introduce asterix-derive(UpdateFspec): Automatic update of Fspec from data_items generation.
- Cat34: Complete

## [0.2.1] - 2020-08-30
- Cat48: Add TrackQuality
- Cat48: Add WarningErrorConditionsTargetClass
- Cat48: Add Mode3ACodeConfidenceIndicator
- Cat48: Add ModeCCodeAndConfidenceIndicator
- Cat48: Add HeightMeasuredBy3dRadar
- Cat48: Add RadialDopplerSpeed
- Cat48: Add ACASResolutionAdvisoryReport
- Cat48: Add Mode1CodeOctalRepresentation
- Cat48: Add Mode2CodeOctalRepresentation
- Cat48: Add Mode1CodeConfidenceIndicator
- Cat48: Add Mode2CodeConfidenceIndicator
- Cat34: Add AntennaRotationSpeed
- Update Data Item docs

## [0.2.0] - 2020-08-21
-  add AsterixPacket::finalize() for updating the packet to the correct fspec and len after
   messages are added

## [0.1.1] - 2020-08-16
-  Add License file (MIT) and prepare Cargo.toml file for release

## [0.1.0] - 2020-08-16
-  Initial Release for most of CAT048 and CAT034
