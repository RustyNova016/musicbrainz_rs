# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [0.8.0](https://github.com/RustyNova/musicbrainz_rs_nova/compare/195a6cd51b7f702a4936da578483e5f31fef5bba..0.8.0) - 2024-10-07
#### Bug Fixes
- **(cdstub)** Make Barcode field optional - ([cefceb2](https://github.com/RustyNova/musicbrainz_rs_nova/commit/cefceb2f90cc737854520c3636de289a8a94a20c)) - Jan Holthuis
- **(discid)** Fix serialization for `Disc` and `Discid` - ([286e7f0](https://github.com/RustyNova/musicbrainz_rs_nova/commit/286e7f0b4979d624dd71a3f91a958869ac354a7e)) - Jan Holthuis
- **(entity)** Fix serialization of `BrowseResult` type - ([1ed0497](https://github.com/RustyNova/musicbrainz_rs_nova/commit/1ed04976347358108e0f5fd85aa11c53ca918b66)) - Jan Holthuis
- **(event)** Make `time` field optional - ([a89bb6b](https://github.com/RustyNova/musicbrainz_rs_nova/commit/a89bb6bdb57c398918f75d7b7b4e2a8d817b1aab)) - Jan Holthuis
- **(genre)** Make `count` field optional - ([ab94cb5](https://github.com/RustyNova/musicbrainz_rs_nova/commit/ab94cb56e85ee165a1dcc1c6a55d0652aaf882f4)) - Jan Holthuis
- **(place)** Fix (de-)serialization of string coordinates - ([96f207f](https://github.com/RustyNova/musicbrainz_rs_nova/commit/96f207f59d7105bdc0a6ac59797b0123f5bcc202)) - Jan Holthuis
- **(relations)** Use correct case for `RelationContent` - ([6128b64](https://github.com/RustyNova/musicbrainz_rs_nova/commit/6128b6459f637a530b8aece20f5a4e9d34b32fdb)) - Jan Holthuis
- **(search)** Fix serialization of `SearchResult` type - ([fd1433d](https://github.com/RustyNova/musicbrainz_rs_nova/commit/fd1433d034b9c51d3c16fb7322562317f80f86c4)) - Jan Holthuis
- **(tag)** Make `count` field optional - ([adf7e8e](https://github.com/RustyNova/musicbrainz_rs_nova/commit/adf7e8e70161595439de1a2cd6671abe52334b6d)) - Jan Holthuis
- **(work)** Fix serialization of `MusicalKey` - ([2b8d84f](https://github.com/RustyNova/musicbrainz_rs_nova/commit/2b8d84f648c3b7b024a5171898599edf7aa7e1bc)) - Jan Holthuis
- musicbrainz logo in readme - ([af18ea0](https://github.com/RustyNova/musicbrainz_rs_nova/commit/af18ea0baa0dfe2f8fefb918f44a01c84aa2d71e)) - [@RustyNova](https://github.com/RustyNova)
- add track offset to media - ([e62d78c](https://github.com/RustyNova/musicbrainz_rs_nova/commit/e62d78c057285cc88a7beb445508e519cc59fc36)) - [@RustyNova](https://github.com/RustyNova)
- Added artist credits to tracks - ([c547cc1](https://github.com/RustyNova/musicbrainz_rs_nova/commit/c547cc11a125ac9925a9e36cf8443c52d43e4202)) - [@RustyNova](https://github.com/RustyNova)
- Set track's recording as optional as it can be undefined - ([8845408](https://github.com/RustyNova/musicbrainz_rs_nova/commit/8845408e936d5d19b53b3e03fe39956a3e3e250e)) - [@RustyNova](https://github.com/RustyNova)
- missing media inc querry - ([378255b](https://github.com/RustyNova/musicbrainz_rs_nova/commit/378255bfdde6a74f5ae49a4a85c720ac048440f2)) - [@RustyNova](https://github.com/RustyNova)
- Update once_cell for new minimum requirements - ([b81df71](https://github.com/RustyNova/musicbrainz_rs_nova/commit/b81df7199161ea0ea5df3acc9be5eca1e6052625)) - [@RustyNova](https://github.com/RustyNova)
- Mirror Serialization - ([195a6cd](https://github.com/RustyNova/musicbrainz_rs_nova/commit/195a6cd51b7f702a4936da578483e5f31fef5bba)) - [@RustyNova](https://github.com/RustyNova)
#### Continuous Integration
- Add minimum crate version check - ([22dd8d7](https://github.com/RustyNova/musicbrainz_rs_nova/commit/22dd8d71d9b9c0b1e43542f5a510946b69298105)) - [@RustyNova](https://github.com/RustyNova)
#### Documentation
- documentation additions - ([6c18b0b](https://github.com/RustyNova/musicbrainz_rs_nova/commit/6c18b0be190123e9d9dd33d5e1a3e9a5d149f03c)) - [@RustyNova](https://github.com/RustyNova)
- Added contribution documentation - ([5b894cc](https://github.com/RustyNova/musicbrainz_rs_nova/commit/5b894cce313650dbe2f57969dd74ec594e8c9e9e)) - [@RustyNova](https://github.com/RustyNova)
#### Features
- **(cdstub)** Add missing disambiguation field for `CDStub` - ([c5974c8](https://github.com/RustyNova/musicbrainz_rs_nova/commit/c5974c8b9453832ca28061529a704541bc1e3971)) - Jan Holthuis
- **(genre)** Add missing optional fields - ([d7b0f91](https://github.com/RustyNova/musicbrainz_rs_nova/commit/d7b0f91387d3c6b9270d99d93bc1c14b6a10009c)) - Jan Holthuis
- **(recording)** Add support for `first-release-date` field - ([c228ef3](https://github.com/RustyNova/musicbrainz_rs_nova/commit/c228ef3ee1671615e70475301ffdc1b6b38a46c3)) - Jan Holthuis
- **(release)** Add helpers to get script/language as name/code strings - ([e1290d3](https://github.com/RustyNova/musicbrainz_rs_nova/commit/e1290d31cc5a4e56d2e4427a14cbfb4136a05e53)) - Jan Holthuis
- **(release)** Add support for `asin` field - ([c18566c](https://github.com/RustyNova/musicbrainz_rs_nova/commit/c18566ca9bc8b189afd7a20aa22ce6c3a7723f49)) - Jan Holthuis
- **(release)** Add support for `text-representation` field - ([e9fb639](https://github.com/RustyNova/musicbrainz_rs_nova/commit/e9fb6390aa2834237afa21d29168f381aa09cb34)) - Jan Holthuis
- **(search)** Implement `Searchable` for `Place`/`Tag`/`Url` - ([4655b9b](https://github.com/RustyNova/musicbrainz_rs_nova/commit/4655b9b1c1c778552ce3cbb5962a4c27e0bb55b6)) - Jan Holthuis
- **(tag)** Add missing `score` field - ([224bbe5](https://github.com/RustyNova/musicbrainz_rs_nova/commit/224bbe5c1f63dabfd39d8c872ae151d509dd1e8d)) - Jan Holthuis
#### Miscellaneous Chores
- add legacy_serialize to readme - ([2daa198](https://github.com/RustyNova/musicbrainz_rs_nova/commit/2daa198b3fccbda9c123f1bc342f3264acc08d2f)) - [@RustyNova](https://github.com/RustyNova)
- version bump - ([4787002](https://github.com/RustyNova/musicbrainz_rs_nova/commit/4787002c065b1c94a71c206bcf658171e943e0e8)) - [@RustyNova](https://github.com/RustyNova)
- formating - ([fd0d26d](https://github.com/RustyNova/musicbrainz_rs_nova/commit/fd0d26dc15bf8b243cfd7c7d5d4aebd2e1ffa0bd)) - [@RustyNova](https://github.com/RustyNova)
- formating - ([ce9a3de](https://github.com/RustyNova/musicbrainz_rs_nova/commit/ce9a3de84315537a2b6c862f4ee814734fb75f64)) - [@RustyNova](https://github.com/RustyNova)

- - -

## [0.7.0](https://github.com/RustyNova/musicbrainz_rs_nova/compare/64668e301b4782f885091b561e90cd6a4c9f3418..0.7.0) - 2024-09-04
#### Build system
- **(deps)** bump softprops/action-gh-release from 1 to 2 - ([8a8903c](https://github.com/RustyNova/musicbrainz_rs_nova/commit/8a8903c5be7728ede59d126bf78b41953c2f8bcc)) - dependabot[bot]
- **(deps)** bump actions/checkout from 2 to 4 - ([64668e3](https://github.com/RustyNova/musicbrainz_rs_nova/commit/64668e301b4782f885091b561e90cd6a4c9f3418)) - dependabot[bot]
- Bump MSRV to 1.70 - ([fab5c38](https://github.com/RustyNova/musicbrainz_rs_nova/commit/fab5c386134256cfd4db1095e7593347df943d87)) - [@RustyNova](https://github.com/RustyNova)
#### Continuous Integration
- Reduced number of tests - ([f2fdb3b](https://github.com/RustyNova/musicbrainz_rs_nova/commit/f2fdb3b2a112724e88206fd97cb5b4a0b825a822)) - [@RustyNova](https://github.com/RustyNova)
- restore tests - ([30e585b](https://github.com/RustyNova/musicbrainz_rs_nova/commit/30e585b79f73b6a59c24812ef8bb9114e158571b)) - [@RustyNova](https://github.com/RustyNova)
#### Miscellaneous Chores
- Update version in readme - ([5d93f6a](https://github.com/RustyNova/musicbrainz_rs_nova/commit/5d93f6aa88e7642f264cefb5c0d701cb82099a07)) - [@RustyNova](https://github.com/RustyNova)
- Update MSRV in readme - ([57d6044](https://github.com/RustyNova/musicbrainz_rs_nova/commit/57d6044bf9ff23ca0d3cbfe0830eeaa27e297f4d)) - [@RustyNova](https://github.com/RustyNova)
#### Style
- format - ([cd27f0a](https://github.com/RustyNova/musicbrainz_rs_nova/commit/cd27f0acc949b6423b2321d01d35d1b59c076945)) - [@RustyNova](https://github.com/RustyNova)
#### Tests
- Fixed async test with serialization - ([e6fb58c](https://github.com/RustyNova/musicbrainz_rs_nova/commit/e6fb58c9aa648e14dbfd5089a63e64b4879e1332)) - [@RustyNova](https://github.com/RustyNova)

- - -

## [0.5.1](https://github.com/RustyNova/musicbrainz_rs_nova/compare/4f284f3adf4c9f95fb219442ac7a833fc2946fc5..0.5.1) - 2024-05-30
#### Miscellaneous Chores
- Fixed broken test data - ([611dd1f](https://github.com/RustyNova/musicbrainz_rs_nova/commit/611dd1f166dc25b8bcec01cace158a2dd7eb8d83)) - [@RustyNova](https://github.com/RustyNova)
- Changed crate name - ([6179b31](https://github.com/RustyNova/musicbrainz_rs_nova/commit/6179b314f892a5a3f8f207888fada4fb09d1414b)) - [@RustyNova](https://github.com/RustyNova)
- Changed CI for fork - ([e62dee2](https://github.com/RustyNova/musicbrainz_rs_nova/commit/e62dee2a299a4b14f012c51600152ac937aef85c)) - [@RustyNova](https://github.com/RustyNova)

- - -

## [0.5.0](https://github.com/oknozor/musicbrainz_rs/compare/0.4.0..0.5.0) - 2023-01-22
#### Bug Fixes
- **(artist)** fix artist fuzzy search - ([b737202](https://github.com/oknozor/musicbrainz_rs/commit/b73720213e3cd131ac9271b15d7cdb3adf761dc7)) - [@ritiek](https://github.com/ritiek)
- **(ci)** add build and test to new config - ([7a1fd3d](https://github.com/oknozor/musicbrainz_rs/commit/7a1fd3d8503ec9645e811bbc4ab9a982433e620a)) - [@oknozor](https://github.com/oknozor)
- **(test)** update assertions as per api response - ([7ad1deb](https://github.com/oknozor/musicbrainz_rs/commit/7ad1debac35f506398728f97aa2923a5b596fae4)) - [@ritiek](https://github.com/ritiek)
- **(test)** temporary fix for musicbrainz api rate limit - ([34192e0](https://github.com/oknozor/musicbrainz_rs/commit/34192e052199c258eed442e865af06fe4afd7c98)) - [@oknozor](https://github.com/oknozor)
- clippy errors added in 1.63.0 - ([b584d24](https://github.com/oknozor/musicbrainz_rs/commit/b584d24b2abc1c34bf96fb7d5fb0686bb9c2fd72)) - [@Yesterday17](https://github.com/Yesterday17)
- clippy errors and warnings - ([4e0d40a](https://github.com/oknozor/musicbrainz_rs/commit/4e0d40ab3284599577b059239955f0db86bdd1e7)) - [@Yesterday17](https://github.com/Yesterday17)
- Serialize/Deserialize impls for WorkAttribute - ([aca97ab](https://github.com/oknozor/musicbrainz_rs/commit/aca97ab901de8f1a5fa2df1bcd65ebafd8adc5fa)) - Roger Filmyer
- Serialize/Deserialize impls for work.rs enums - ([4d24609](https://github.com/oknozor/musicbrainz_rs/commit/4d246094cebb01fb6d0b994facd607717c90e848)) - Roger Filmyer
- use serde default on release group to avoid deserialization panic - ([df846a0](https://github.com/oknozor/musicbrainz_rs/commit/df846a0684dc33be6651a8e44a052d0938521c15)) - [@oknozor](https://github.com/oknozor)
- attempt to fix hyper 'incomplete message' error on coverart' - ([d1940dd](https://github.com/oknozor/musicbrainz_rs/commit/d1940dd3e27d562b7fde3d0b140c7f11d865dcca)) - [@oknozor](https://github.com/oknozor)
- bump lucene query builder to get query 'type' (rust reserved word) - ([51fe4d2](https://github.com/oknozor/musicbrainz_rs/commit/51fe4d210cf371b551bc57fc9dcb8ed9cc9313f3)) - [@oknozor](https://github.com/oknozor)
- fix release date deserialization - ([9accadd](https://github.com/oknozor/musicbrainz_rs/commit/9accadde32b47d7e1a9fc5dabcb929eb8df582e6)) - [@oknozor](https://github.com/oknozor)
- fix created date format in search response - ([d3d7c08](https://github.com/oknozor/musicbrainz_rs/commit/d3d7c08f7c57ef0f016af72c129b549d60718fea)) - [@oknozor](https://github.com/oknozor)
- no sonar for rustacean - ([28df07d](https://github.com/oknozor/musicbrainz_rs/commit/28df07da6670c57aea6f79a527cb4953cd08f9f1)) - [@oknozor](https://github.com/oknozor)
- typo - ([497a0e0](https://github.com/oknozor/musicbrainz_rs/commit/497a0e0936320bb0369547bef90b42f21908e1b0)) - [@oknozor](https://github.com/oknozor)
#### Build system
- add release action - ([42943ed](https://github.com/oknozor/musicbrainz_rs/commit/42943edbb778cba1a4e884e4b2951265b738cc80)) - [@oknozor](https://github.com/oknozor)
- update CI and release build - ([48e6a8b](https://github.com/oknozor/musicbrainz_rs/commit/48e6a8b6c9044c1ffb97f0736a7aea88cde81837)) - [@oknozor](https://github.com/oknozor)
#### Continuous Integration
- fix cargo-edit usage in cog.toml - ([f43108b](https://github.com/oknozor/musicbrainz_rs/commit/f43108b5e44796e875940b3fe2fd82948159a2cf)) - [@oknozor](https://github.com/oknozor)
- update cog release config - ([2351bd7](https://github.com/oknozor/musicbrainz_rs/commit/2351bd7308dc9d591b465ef44c1a7d56a13e37d4)) - [@oknozor](https://github.com/oknozor)
- remove duplicate test/coverage in CI - ([dc953af](https://github.com/oknozor/musicbrainz_rs/commit/dc953af32d224f60553ff636d36ad2fb10689243)) - [@oknozor](https://github.com/oknozor)
- switch default branch to 'main' - ([808c7c3](https://github.com/oknozor/musicbrainz_rs/commit/808c7c30206f55ee2bd98bbe42b2a0f430d65115)) - [@oknozor](https://github.com/oknozor)
- add cocogitto - ([bc158a7](https://github.com/oknozor/musicbrainz_rs/commit/bc158a7d0c6d614979e99dacba46c6d3699361b0)) - [@oknozor](https://github.com/oknozor)
- move to github action - ([9bbd646](https://github.com/oknozor/musicbrainz_rs/commit/9bbd6468c12963da38492c95f198bc4f767cda5f)) - [@oknozor](https://github.com/oknozor)
- add sonnar - ([a2cb80e](https://github.com/oknozor/musicbrainz_rs/commit/a2cb80eae861497615d6f79c167765034903aa33)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- **(coverart)** add examples - ([6b6fa6e](https://github.com/oknozor/musicbrainz_rs/commit/6b6fa6e2fd558950e57e6dad76a55151d2d74e2e)) - [@ritiek](https://github.com/ritiek)
- **(release-group)** update readme - ([0959d11](https://github.com/oknozor/musicbrainz_rs/commit/0959d117dd27f239c078d003fa73c3516dde25b4)) - [@oknozor](https://github.com/oknozor)
- fix feature flag for async examples - ([d656793](https://github.com/oknozor/musicbrainz_rs/commit/d6567939b314a3d218ad482a66c4229bc276eb53)) - [@oknozor](https://github.com/oknozor)
- improve entity docs - ([ea69701](https://github.com/oknozor/musicbrainz_rs/commit/ea697015b3ec193cadb01aa64f61698a5eb3d09c)) - [@ritiek](https://github.com/ritiek)
- README replace model with entity in imports - ([9de09ba](https://github.com/oknozor/musicbrainz_rs/commit/9de09ba9808ef7f903ae03d50edae2b78ad41065)) - Nick Zana
- add crate level doc and fix visibility - ([c89ad4c](https://github.com/oknozor/musicbrainz_rs/commit/c89ad4c84b08da03f8ea9a5e01238ffe6c3f3c46)) - [@oknozor](https://github.com/oknozor)
- add search example - ([5d40cef](https://github.com/oknozor/musicbrainz_rs/commit/5d40cef3864ecc164e546123bdf24d35ac0d8c20)) - [@oknozor](https://github.com/oknozor)
- update README - ([d141ef6](https://github.com/oknozor/musicbrainz_rs/commit/d141ef67101f8755ebe92d71847e80ad0aa2574b)) - [@oknozor](https://github.com/oknozor)
- update doc for browse query - ([2f37133](https://github.com/oknozor/musicbrainz_rs/commit/2f371332f554a01857983790eff1230f386812e5)) - [@oknozor](https://github.com/oknozor)
- update examples and readme - ([5ad4e86](https://github.com/oknozor/musicbrainz_rs/commit/5ad4e86892c0a345952a260d3681314a8ffa30a5)) - [@oknozor](https://github.com/oknozor)
- move model status to github milestone - ([a02e419](https://github.com/oknozor/musicbrainz_rs/commit/a02e419fbe59c0f7e7176974f7f0e0fe454f4e56)) - [@oknozor](https://github.com/oknozor)
- update README - ([91ad822](https://github.com/oknozor/musicbrainz_rs/commit/91ad82293b231cf91990a92004a5b6de9cfe7f1e)) - [@oknozor](https://github.com/oknozor)
- update manifest - ([d22e6d8](https://github.com/oknozor/musicbrainz_rs/commit/d22e6d82d688b3ee4a10e28b3293862452cab51d)) - [@oknozor](https://github.com/oknozor)
- update manifest - ([8d635b2](https://github.com/oknozor/musicbrainz_rs/commit/8d635b2d8107721f836a305dc167679e43ae5c06)) - [@oknozor](https://github.com/oknozor)
#### Features
- **(*)** add the ability to use inc params - ([3bbe25d](https://github.com/oknozor/musicbrainz_rs/commit/3bbe25dc46c7325adc6b27755f3ed44c52d0b4d5)) - [@oknozor](https://github.com/oknozor)
- **(annotation)** implement search - ([33aff7a](https://github.com/oknozor/musicbrainz_rs/commit/33aff7acafb9fea8a4308eda81fa74469ebe3dfd)) - [@ritiek](https://github.com/ritiek)
- **(api)** add a generic way to build request with fluent syntax - ([aa2dbec](https://github.com/oknozor/musicbrainz_rs/commit/aa2dbec98acf107d135f2f96f29232c5ab57462e)) - [@oknozor](https://github.com/oknozor)
- **(area)** implement search - ([da56b14](https://github.com/oknozor/musicbrainz_rs/commit/da56b1460230a631e6092db69788fe29824f9fb8)) - [@ritiek](https://github.com/ritiek)
- **(area)** support additional relation incs - ([c80ab31](https://github.com/oknozor/musicbrainz_rs/commit/c80ab3137248949aa337661fca06821e89d25268)) - [@ritiek](https://github.com/ritiek)
- **(artist)** support additional relation incs - ([2379178](https://github.com/oknozor/musicbrainz_rs/commit/2379178c1bd627d00d27f4bc750848f67f429f28)) - [@ritiek](https://github.com/ritiek)
- **(artists)** add recording and releases-groups incs for artists - ([36f45c7](https://github.com/oknozor/musicbrainz_rs/commit/36f45c71be1e3cb635648639a1c4b2ca1461ab0c)) - [@oknozor](https://github.com/oknozor)
- **(browse)** add a deserialize impl for generic browse result - ([46d4dc5](https://github.com/oknozor/musicbrainz_rs/commit/46d4dc5d4b6f79f86c1d25e01a5a21e1d05a41c2)) - [@oknozor](https://github.com/oknozor)
- **(cdstub)** implement search - ([373ba50](https://github.com/oknozor/musicbrainz_rs/commit/373ba50c12e390084f5d9e563cfc945de96e774b)) - [@ritiek](https://github.com/ritiek)
- **(ci)** add rustfmt - ([3cdad3c](https://github.com/oknozor/musicbrainz_rs/commit/3cdad3c1735255ab675232d35f7d41bac2b99863)) - [@oknozor](https://github.com/oknozor)
- **(ci)** add build status image - ([59fd690](https://github.com/oknozor/musicbrainz_rs/commit/59fd69013101ef1870ee86b18d343eb775e4a351)) - [@oknozor](https://github.com/oknozor)
- **(ci)** add travis configuration - ([6e30dda](https://github.com/oknozor/musicbrainz_rs/commit/6e30dda72b36c984b347945a587e8d0bdedb8b4d)) - [@oknozor](https://github.com/oknozor)
- **(config)** make http client configurable - ([ddbbcef](https://github.com/oknozor/musicbrainz_rs/commit/ddbbcefc8026c67bd7ea5c7cf07152dfdfc86497)) - [@oknozor](https://github.com/oknozor)
- **(coverart)** coverart builder to make specific queries - ([e478d4c](https://github.com/oknozor/musicbrainz_rs/commit/e478d4cd42c8e08246d62d83fa6277a7cd08f939)) - [@ritiek](https://github.com/ritiek)
- **(coverart)** implement get_coverart method - ([380b1e0](https://github.com/oknozor/musicbrainz_rs/commit/380b1e098c9768bc66c006deed3fc0c384046d47)) - [@ritiek](https://github.com/ritiek)
- **(event)** implement search - ([e1f2d00](https://github.com/oknozor/musicbrainz_rs/commit/e1f2d009fda1a8455052d208c5883df3ec579ac8)) - [@ritiek](https://github.com/ritiek)
- **(event)** support additional relation incs - ([509e43a](https://github.com/oknozor/musicbrainz_rs/commit/509e43ae37ff1e8e3aba3788289a3234f2b33242)) - [@ritiek](https://github.com/ritiek)
- **(includes)** support level relations - ([a161479](https://github.com/oknozor/musicbrainz_rs/commit/a1614793e5e8fb1e0b1b4b8c1dec21419549e34b)) - [@ritiek](https://github.com/ritiek)
- **(includes)** support additional relation incs - ([3183386](https://github.com/oknozor/musicbrainz_rs/commit/31833868479f2c16a0972954cbace8eacb84efc4)) - [@ritiek](https://github.com/ritiek)
- **(includes)** add url-rel inc on artist, release - ([e423685](https://github.com/oknozor/musicbrainz_rs/commit/e4236850740eca2fb2e367392e9d4460d147578d)) - [@ritiek](https://github.com/ritiek)
- **(includes)** add artist include on release - ([073f1de](https://github.com/oknozor/musicbrainz_rs/commit/073f1de0cffd5e3847c4cd0d35df773f5db4e7a8)) - [@ritiek](https://github.com/ritiek)
- **(includes)** partially add annotations - ([a952e3c](https://github.com/oknozor/musicbrainz_rs/commit/a952e3c1c214a0afadc7e8ecd8a27f10146d7116)) - [@oknozor](https://github.com/oknozor)
- **(includes)** add genres - ([0bf2bf9](https://github.com/oknozor/musicbrainz_rs/commit/0bf2bf97534b6e26a85bff074de1277a5909e7ab)) - [@oknozor](https://github.com/oknozor)
- **(includes)** add rating everywhere - ([d68bd39](https://github.com/oknozor/musicbrainz_rs/commit/d68bd3949c9267d3b530623f846c9150132d15d1)) - [@oknozor](https://github.com/oknozor)
- **(includes)** add aliases everywhere - ([abc0146](https://github.com/oknozor/musicbrainz_rs/commit/abc01462d6b2a98de2afad7a0c103b77c15ae7be)) - [@oknozor](https://github.com/oknozor)
- **(includes)** add tags - ([71a64a7](https://github.com/oknozor/musicbrainz_rs/commit/71a64a78402232e0af537492a1ef044e702dd8b7)) - [@oknozor](https://github.com/oknozor)
- **(includes)** add release group includes - ([ca0e4f5](https://github.com/oknozor/musicbrainz_rs/commit/ca0e4f5340427d76112c7ea946c4ca9f36d1ecd8)) - [@oknozor](https://github.com/oknozor)
- **(includes)** add release includes - ([efcdb94](https://github.com/oknozor/musicbrainz_rs/commit/efcdb94b06a99e1ac4d06ab9f9f8f3b6754df2d7)) - [@oknozor](https://github.com/oknozor)
- **(includes)** add recording incs & label aliases - ([050efc1](https://github.com/oknozor/musicbrainz_rs/commit/050efc14d39a9a8ca82cc02e7a29525751a7adb4)) - [@oknozor](https://github.com/oknozor)
- **(includes)** add label releases - ([4e5c927](https://github.com/oknozor/musicbrainz_rs/commit/4e5c927142b8e9fbda464dc5578271f3e3cc295f)) - [@oknozor](https://github.com/oknozor)
- **(instrument)** implement search - ([36a1130](https://github.com/oknozor/musicbrainz_rs/commit/36a113021fac4433224c9f605834dfda483be6b5)) - [@ritiek](https://github.com/ritiek)
- **(instrument)** support additional relation incs - ([041b287](https://github.com/oknozor/musicbrainz_rs/commit/041b2874839d92934d5d4e1869047608f3f22951)) - [@ritiek](https://github.com/ritiek)
- **(label)** implement search - ([620b36f](https://github.com/oknozor/musicbrainz_rs/commit/620b36f630e52b2b7767928e349d5960bc7026c3)) - [@ritiek](https://github.com/ritiek)
- **(label)** support additional relation incs - ([1734a58](https://github.com/oknozor/musicbrainz_rs/commit/1734a581a482e4839d7afe873ba24f2440d32c74)) - [@ritiek](https://github.com/ritiek)
- **(model)** add media - ([60e2acf](https://github.com/oknozor/musicbrainz_rs/commit/60e2acf10f56f101eb01152fc2c8a9de991625d8)) - [@oknozor](https://github.com/oknozor)
- **(model)** add url - ([012b7ec](https://github.com/oknozor/musicbrainz_rs/commit/012b7ecaf5eb00c794d3909d7cbff618145ad642)) - [@oknozor](https://github.com/oknozor)
- **(model)** add serie - ([d99c67c](https://github.com/oknozor/musicbrainz_rs/commit/d99c67c092b521aa22912a83068feda0037e1c87)) - [@oknozor](https://github.com/oknozor)
- **(model)** add place - ([91150eb](https://github.com/oknozor/musicbrainz_rs/commit/91150eb4672745a23f479e9785ab89daad3b1278)) - [@oknozor](https://github.com/oknozor)
- **(model)** add instrument - ([5ba85e0](https://github.com/oknozor/musicbrainz_rs/commit/5ba85e0755f0803d8902e8cc799884f7bd8fcf26)) - [@oknozor](https://github.com/oknozor)
- **(model)** add event - ([40ce9df](https://github.com/oknozor/musicbrainz_rs/commit/40ce9df1c12ac36b19fea695c162affde112dc54)) - [@oknozor](https://github.com/oknozor)
- **(model)** bound include to each entity separatly - ([6e266af](https://github.com/oknozor/musicbrainz_rs/commit/6e266af7bb930ff8aa5310108f83f297324bfc3f)) - [@oknozor](https://github.com/oknozor)
- **(model)** add aliases and artist alias relation - ([d64d34c](https://github.com/oknozor/musicbrainz_rs/commit/d64d34ca8231b873e29adf7dd726e8e15c13b4a6)) - [@oknozor](https://github.com/oknozor)
- **(model)** add area: GET, move area and lifespan to dedicated modules - ([6f7259e](https://github.com/oknozor/musicbrainz_rs/commit/6f7259eb1d769e416ef152d76005d7fa282ba8d1)) - [@oknozor](https://github.com/oknozor)
- **(model)** add label - ([239df35](https://github.com/oknozor/musicbrainz_rs/commit/239df35518908c590019b3b8721ad96cf04894dc)) - [@oknozor](https://github.com/oknozor)
- **(model)** add work - ([ffe4e85](https://github.com/oknozor/musicbrainz_rs/commit/ffe4e850f378ca15ce40952bda5e8592a91b9201)) - [@oknozor](https://github.com/oknozor)
- **(model)** add release - ([22ae49f](https://github.com/oknozor/musicbrainz_rs/commit/22ae49fb40c4433ef4c8f5c30ff597ae3769f061)) - [@oknozor](https://github.com/oknozor)
- **(model)** add release group and refactor doc comments - ([0c2884b](https://github.com/oknozor/musicbrainz_rs/commit/0c2884baffd7b31fdedf77e685fecd9a0639a6a5)) - [@oknozor](https://github.com/oknozor)
- **(place)** support additional relation incs - ([b9c9a62](https://github.com/oknozor/musicbrainz_rs/commit/b9c9a62c656c330f6b2691815c135bb7d0bc3bb4)) - [@ritiek](https://github.com/ritiek)
- **(recording)** implement search - ([d324652](https://github.com/oknozor/musicbrainz_rs/commit/d324652ba1220e1f0281d0477eb8e4cadc96d881)) - [@ritiek](https://github.com/ritiek)
- **(recording)** add work level rel-incs - ([03a19e5](https://github.com/oknozor/musicbrainz_rs/commit/03a19e58bd06d3efe920fbfb9fbadb0c3909baad)) - [@ritiek](https://github.com/ritiek)
- **(recording)** support additional relation incs - ([e87076a](https://github.com/oknozor/musicbrainz_rs/commit/e87076a2e501cad4d37914c4171498935e2b4b2a)) - [@ritiek](https://github.com/ritiek)
- **(recordings)** add url-relations incs for recordings - ([90e3f03](https://github.com/oknozor/musicbrainz_rs/commit/90e3f03ceb6c0d26bd854758014cd428d23a231e)) - [@ritiek](https://github.com/ritiek)
- **(recordings)** add isrcs incs for recordings - ([380dc0b](https://github.com/oknozor/musicbrainz_rs/commit/380dc0b41df8c206e2991f8001f0d2fe0c9fbe3c)) - [@ritiek](https://github.com/ritiek)
- **(relations)** expose target type - ([eb1a425](https://github.com/oknozor/musicbrainz_rs/commit/eb1a425156501dba01b74f5dc2bd46fcc90ac24d)) - [@ritiek](https://github.com/ritiek)
- **(relations)** add relations to model - ([ff0e62e](https://github.com/oknozor/musicbrainz_rs/commit/ff0e62e5cd89f049546b6672228980dbd23f6098)) - [@oknozor](https://github.com/oknozor)
- **(release)** support `artist_credits` subquery - ([93a2535](https://github.com/oknozor/musicbrainz_rs/commit/93a2535fb76e596bed3f9c76b0879912c8c19b79)) - [@snylonue](https://github.com/snylonue)
- **(release)** implement search - ([f3cf2a5](https://github.com/oknozor/musicbrainz_rs/commit/f3cf2a5514ac8381b2242563bfde66a3ec5372fe)) - [@ritiek](https://github.com/ritiek)
- **(release)** add work & recording level rel-incs - ([aec4908](https://github.com/oknozor/musicbrainz_rs/commit/aec49089070fcd172c588b0ccfd550a04a17a039)) - [@ritiek](https://github.com/ritiek)
- **(release)** add artist-rel inc on release - ([3a25742](https://github.com/oknozor/musicbrainz_rs/commit/3a25742997e8da3a55aae13c0c9f3d2604f38aaa)) - [@ritiek](https://github.com/ritiek)
- **(release-group)** support additional relation incs - ([5367ca6](https://github.com/oknozor/musicbrainz_rs/commit/5367ca68356cddb3c910d08fae51aeadc0788855)) - [@ritiek](https://github.com/ritiek)
- **(search)** add search query - ([92bf978](https://github.com/oknozor/musicbrainz_rs/commit/92bf978c98056073876b12431042f3ad5441dcca)) - [@oknozor](https://github.com/oknozor)
- **(search)** add search entity - ([0a28110](https://github.com/oknozor/musicbrainz_rs/commit/0a2811001d59dc289391e9f5f56b0a8883caaa47)) - [@oknozor](https://github.com/oknozor)
- **(series)** implement search - ([7a6673c](https://github.com/oknozor/musicbrainz_rs/commit/7a6673c74b4d1b5ed13f76a9c813ba174aab4570)) - [@ritiek](https://github.com/ritiek)
- **(series)** support additional relation incs - ([dbdda03](https://github.com/oknozor/musicbrainz_rs/commit/dbdda03419d2ef00b01cca8ad252b12c55851f06)) - [@ritiek](https://github.com/ritiek)
- **(work)** implement search - ([6910694](https://github.com/oknozor/musicbrainz_rs/commit/69106945358cbfab655d0f93a6c154b77ebf4c95)) - [@ritiek](https://github.com/ritiek)
- **(work)** support additional relation incs - ([1926b34](https://github.com/oknozor/musicbrainz_rs/commit/1926b34e89e1ed0e8ce45f6ed3bc369d8696f179)) - [@ritiek](https://github.com/ritiek)
- introduce offset and limit on BrowseQuery, - ([bd901bc](https://github.com/oknozor/musicbrainz_rs/commit/bd901bcf65c29b1966bf46872dce12893eaf5812)) - Marco Napetti
- add async api - ([269fb92](https://github.com/oknozor/musicbrainz_rs/commit/269fb9258320668e38f6c7292711b9fec1794a24)) - [@oknozor](https://github.com/oknozor)
- implement search for release-groups - ([b23759a](https://github.com/oknozor/musicbrainz_rs/commit/b23759a92df0aef58b9ce0542033caed456a0a55)) - [@oknozor](https://github.com/oknozor)
- implement auto-retries - ([0fc1fce](https://github.com/oknozor/musicbrainz_rs/commit/0fc1fce200b5b70b7a9dc9ae7cbb0d87df1659fa)) - [@ritiek](https://github.com/ritiek)
- fetch coverart images for releases - ([3e8c607](https://github.com/oknozor/musicbrainz_rs/commit/3e8c6078848ef73b33967f4f25a94da6abf6b17e)) - [@ritiek](https://github.com/ritiek)
- add all browses for works - ([1266e25](https://github.com/oknozor/musicbrainz_rs/commit/1266e259c3111eb8049c2af07147669ef28fa4e3)) - [@oknozor](https://github.com/oknozor)
- add all browses for release_groups - ([c963cf7](https://github.com/oknozor/musicbrainz_rs/commit/c963cf764c3d6ecd0d801a59984c4b1d76df3a37)) - [@oknozor](https://github.com/oknozor)
- add all browses for releases - ([6bb19b3](https://github.com/oknozor/musicbrainz_rs/commit/6bb19b35aea8b6462d54c931cf32dfcd3d9fd4e3)) - [@oknozor](https://github.com/oknozor)
- add all browses for recordings - ([f14de18](https://github.com/oknozor/musicbrainz_rs/commit/f14de1858b56f7f5d14f8b6ca76b1241c877b63e)) - [@oknozor](https://github.com/oknozor)
- add all browses for places - ([aa7d08d](https://github.com/oknozor/musicbrainz_rs/commit/aa7d08dc897914fa8afb3cbc2d2b152bd5c3569f)) - [@oknozor](https://github.com/oknozor)
- add all browses for labels - ([222464b](https://github.com/oknozor/musicbrainz_rs/commit/222464b6f14c52d3552afaa9a7f9e5bde436464d)) - [@oknozor](https://github.com/oknozor)
- add all browses for events - ([2feb79f](https://github.com/oknozor/musicbrainz_rs/commit/2feb79fbd0fef36af6af5a5ecb6fa01ad55aa90f)) - [@oknozor](https://github.com/oknozor)
- add all browses for artists - ([9e347f4](https://github.com/oknozor/musicbrainz_rs/commit/9e347f4066625dc4e1e6b6d20f9aa6f9a82b0507)) - [@oknozor](https://github.com/oknozor)
#### Miscellaneous Chores
- **(version)** 0.4.0 - ([039a7c4](https://github.com/oknozor/musicbrainz_rs/commit/039a7c475450d1608b75a929d0a8d05c4b340a4b)) - [@oknozor](https://github.com/oknozor)
- **(version)** 0.3.0 - ([0928b82](https://github.com/oknozor/musicbrainz_rs/commit/0928b821c4ebe88ccdeb9e5609896317daea4d56)) - [@oknozor](https://github.com/oknozor)
- add nappa85 to contributors list - ([fde464b](https://github.com/oknozor/musicbrainz_rs/commit/fde464be3189ffa2b715569481521a4ef32223d8)) - [@oknozor](https://github.com/oknozor)
- remove debug statements - ([3df884c](https://github.com/oknozor/musicbrainz_rs/commit/3df884c5b691e57c2752f68c969946fcc95c5f2b)) - [@ritiek](https://github.com/ritiek)
- rename "serie" -> "series" - ([da1f776](https://github.com/oknozor/musicbrainz_rs/commit/da1f776fd28c20f1edebbdc7a6894ca9dcc637c0)) - [@ritiek](https://github.com/ritiek)
- test recording-rels on different area mbid - ([9112df7](https://github.com/oknozor/musicbrainz_rs/commit/9112df7a7439ed1ffcf8d02e9a0c01775b9c358a)) - [@ritiek](https://github.com/ritiek)
- rename artist-rel inc for consistency - ([92f5c8a](https://github.com/oknozor/musicbrainz_rs/commit/92f5c8aaa8490ef6f774428db4ecbd5b4bbfecd2)) - [@ritiek](https://github.com/ritiek)
- fix clippy lints - ([f114d27](https://github.com/oknozor/musicbrainz_rs/commit/f114d275c0bbd04dd7401743fa4c0f0df49a7136)) - [@oknozor](https://github.com/oknozor)
- fix typo in cog config - ([d0d1a20](https://github.com/oknozor/musicbrainz_rs/commit/d0d1a207385a666983f24b53eed7a2e7c2b92780)) - [@oknozor](https://github.com/oknozor)
- bump lucene query builder to 0.2.4 - ([75d7ef2](https://github.com/oknozor/musicbrainz_rs/commit/75d7ef2dd6d6b5f87551f58e950f9eb06e9e7240)) - [@oknozor](https://github.com/oknozor)
- lucene query builder to 0.2.3 - ([43bc39e](https://github.com/oknozor/musicbrainz_rs/commit/43bc39e5eb7146d23224aaa47c1143178c4cb73c)) - [@oknozor](https://github.com/oknozor)
- fmt all - ([ef9d166](https://github.com/oknozor/musicbrainz_rs/commit/ef9d16666e056da267e7b6321380d08ef8fadc89)) - [@oknozor](https://github.com/oknozor)
- disable places test until next MB release - ([4afe247](https://github.com/oknozor/musicbrainz_rs/commit/4afe2478a18e9480aa1400eedefc94a0227597e4)) - [@oknozor](https://github.com/oknozor)
- version 0.1.4 - ([913718c](https://github.com/oknozor/musicbrainz_rs/commit/913718cd1e04b83e7b0d11db36f789d7e0bd9320)) - [@oknozor](https://github.com/oknozor)
- version 0.1.3 - ([19ae281](https://github.com/oknozor/musicbrainz_rs/commit/19ae2819e730c3ab3054caa706a88991de0b3ddf)) - [@oknozor](https://github.com/oknozor)
- release 0.1.2 - ([9dd2a70](https://github.com/oknozor/musicbrainz_rs/commit/9dd2a7034b7ee74976357475e3fd075155288c56)) - [@oknozor](https://github.com/oknozor)
- remove useless print - ([a7b9de4](https://github.com/oknozor/musicbrainz_rs/commit/a7b9de466de2888e8934545cd30a26803261ec76)) - [@oknozor](https://github.com/oknozor)
- remove useless bin folder - ([54193cc](https://github.com/oknozor/musicbrainz_rs/commit/54193cc60e560407c7c41f119ef81af5b784d751)) - [@oknozor](https://github.com/oknozor)
- alpha 0.1.1 - ([be5d54c](https://github.com/oknozor/musicbrainz_rs/commit/be5d54c242001ea78eedf5192c7ffeefa506d324)) - [@oknozor](https://github.com/oknozor)
- Create LICENSE - ([d89a21b](https://github.com/oknozor/musicbrainz_rs/commit/d89a21b0e57f8317b1077e1ca0a7e3b0a5160906)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- **(*)** rename package to according to MusicBrainz guidelines - ([ce03e9a](https://github.com/oknozor/musicbrainz_rs/commit/ce03e9a2cc339d9e2cdc1f31aa362f6a382f7fc5)) - [@oknozor](https://github.com/oknozor)
- **(*)** clean imports - ([f5b8f54](https://github.com/oknozor/musicbrainz_rs/commit/f5b8f5406132e05ade7549b01ae08cc25c79dc3b)) - [@oknozor](https://github.com/oknozor)
- **(CI)** correct yaml lints - ([c7d4e5d](https://github.com/oknozor/musicbrainz_rs/commit/c7d4e5dc6374d536ea1ad08e43c8c043df357029)) - [@oknozor](https://github.com/oknozor)
- **(http)** extract http param literals to consts - ([5510cb2](https://github.com/oknozor/musicbrainz_rs/commit/5510cb2ea413662b9668ec5a49be2f355010027f)) - [@oknozor](https://github.com/oknozor)
- **(http)** use lazy static http client - ([edf729a](https://github.com/oknozor/musicbrainz_rs/commit/edf729a4b40daebc28173630fee76914dc6764be)) - [@oknozor](https://github.com/oknozor)
- **(include)** make artists include tests more explicit - ([92707f8](https://github.com/oknozor/musicbrainz_rs/commit/92707f830d93c01acf8d4d3920f534d486d19caa)) - [@oknozor](https://github.com/oknozor)
- **(include)** make label include tests more explicit - ([abc10b3](https://github.com/oknozor/musicbrainz_rs/commit/abc10b3603748f99a0f6fb4abc6cf5ff68a95176)) - [@oknozor](https://github.com/oknozor)
- **(includes)** split incs into variants - ([7cb860d](https://github.com/oknozor/musicbrainz_rs/commit/7cb860d24409cd5e0eb67e87c8f71a63bc642f0f)) - [@ritiek](https://github.com/ritiek)
- **(model)** use serde global deserialization policy to force kebab-case globally - ([78731b5](https://github.com/oknozor/musicbrainz_rs/commit/78731b5495dc944a40d61bd016047715224674e2)) - [@oknozor](https://github.com/oknozor)
- **(search)** use a dedicated entity for search queries - ([1b33963](https://github.com/oknozor/musicbrainz_rs/commit/1b339636681bef18447a6c4d877a2ff119dc55c4)) - [@oknozor](https://github.com/oknozor)
- remove extern crate imports - ([7c0aa0b](https://github.com/oknozor/musicbrainz_rs/commit/7c0aa0b57540811ce9c30f8645f8bffb607d152f)) - [@oknozor](https://github.com/oknozor)
- remove macro uses and replace lazy_static with once_cell - ([32cb0ca](https://github.com/oknozor/musicbrainz_rs/commit/32cb0caed97920261c3bc963be8fd0299f885573)) - [@oknozor](https://github.com/oknozor)
- upgrade reqwest, add rustls feature flag, fix tests - ([e928d4f](https://github.com/oknozor/musicbrainz_rs/commit/e928d4f8c3d89cfe72628797d32b974d632b76d3)) - [@Yesterday17](https://github.com/Yesterday17)
- enums for most entity types - ([e0cd9a7](https://github.com/oknozor/musicbrainz_rs/commit/e0cd9a7cd7a4457c36072cfe52fa0afaf31f4e1f)) - Roger Filmyer
- Enums for Work Attributes & Types - ([3a5b5ef](https://github.com/oknozor/musicbrainz_rs/commit/3a5b5efd3d94a3c70d656938362891839629097a)) - Roger Filmyer
- remove duplicate code - ([3ecc144](https://github.com/oknozor/musicbrainz_rs/commit/3ecc1441277f5d210390e828dba8031aafdf660f)) - [@ritiek](https://github.com/ritiek)
- add macro generated builder functions for browse query - ([a9ea224](https://github.com/oknozor/musicbrainz_rs/commit/a9ea224335fe5f493c369230d5021a7e01bcba82)) - [@oknozor](https://github.com/oknozor)
- add macro generated builder functions for includes - ([92a2cf5](https://github.com/oknozor/musicbrainz_rs/commit/92a2cf5dc4dd7913b8c50bda93bce73a99656a41)) - [@oknozor](https://github.com/oknozor)
- use Default impl instead of custom ones - ([7d17d56](https://github.com/oknozor/musicbrainz_rs/commit/7d17d5671eca7695263666441c71638414210f36)) - [@oknozor](https://github.com/oknozor)
- rename test files - ([1700457](https://github.com/oknozor/musicbrainz_rs/commit/1700457c1b57d2c6096b227ece0b963cea7a383b)) - [@oknozor](https://github.com/oknozor)
- split query trait into specialized fetch and browse trait - ([4ff3438](https://github.com/oknozor/musicbrainz_rs/commit/4ff3438d34781d749322dd932170f0c92b4254b7)) - [@oknozor](https://github.com/oknozor)
#### Style
- fix indentation/spacing with rustfmt - ([d0f3533](https://github.com/oknozor/musicbrainz_rs/commit/d0f3533a88769b0aacb9a5e155ae4c0f1cdf038e)) - Roger Filmyer
- rustfmt all - ([e7977c7](https://github.com/oknozor/musicbrainz_rs/commit/e7977c783769fefb2255d655691907f152e87863)) - [@oknozor](https://github.com/oknozor)
#### Tests
- **(includes)** tests for relation includes - ([caabf44](https://github.com/oknozor/musicbrainz_rs/commit/caabf44d24eb11033cb0378ae20cb221244e6214)) - [@ritiek](https://github.com/ritiek)
- fix feature flag for doc tests - ([9c5327d](https://github.com/oknozor/musicbrainz_rs/commit/9c5327d449eb77432ca1f538629ec43372f02724)) - [@oknozor](https://github.com/oknozor)
- fetch relation fields - ([6477ae8](https://github.com/oknozor/musicbrainz_rs/commit/6477ae8a4cec710a4f85b2fa1c1030dad8a98c82)) - [@ritiek](https://github.com/ritiek)
- make use of auto-retries - ([283fe3e](https://github.com/oknozor/musicbrainz_rs/commit/283fe3e1ec53c7ab3d4ef59288a0c1477d4cffe0)) - [@ritiek](https://github.com/ritiek)
- entity annotations - ([d5537a4](https://github.com/oknozor/musicbrainz_rs/commit/d5537a469852f862d52b779b9340bd43dfa57093)) - [@ritiek](https://github.com/ritiek)
- execute area unit-tests - ([afdafc2](https://github.com/oknozor/musicbrainz_rs/commit/afdafc260ce1456e4e3085257bc632fce1b212a6)) - [@ritiek](https://github.com/ritiek)
- refactor integration test organisation - ([04dc815](https://github.com/oknozor/musicbrainz_rs/commit/04dc815d7c39836089916566cf7ff9e9f99a5c57)) - [@oknozor](https://github.com/oknozor)
- - -

## 0.4.0 - 2021-05-28


### Miscellaneous Chores

[b7ab90](https://github.com/oknozor/musicbrainz_rs/commit/b7ab90799302dac61a5e299851ed20d91c5f08bd) - fix clippy lints - [oknozor](https://github.com/oknozor)


### Bug Fixes

[9699a3](https://github.com/oknozor/musicbrainz_rs/commit/9699a3b4a6593f12e911f80a9fc184fc58d03842) - fix release date deserialization - [oknozor](https://github.com/oknozor)


### Features

[8d2222](https://github.com/oknozor/musicbrainz_rs/commit/8d2222a469f8b27c717f7da72dc439dfa7578c47) - fetch coverart images for releases - [ritiek](https://github.com/ritiek)


- - -
## 0.3.0 - 2021-03-23


### Continuous Integration

[5de00e](https://github.com/oknozor/musicbrainz_rs/commit/5de00e01ec28078511fc4142c38399f0e38d1328) - update cog release config - [oknozor](https://github.com/oknozor)

[ff8b41](https://github.com/oknozor/musicbrainz_rs/commit/ff8b4147817e0755f8c3f473412372dab10dab91) - remove duplicate test/coverage in CI - [oknozor](https://github.com/oknozor)

[567523](https://github.com/oknozor/musicbrainz_rs/commit/567523c20d312d0e4cc8e0396907a42623fb1c12) - switch default branch to 'main' - [oknozor](https://github.com/oknozor)

[02e33e](https://github.com/oknozor/musicbrainz_rs/commit/02e33e95f1a7b5e14e8aa49fa07aa03d2058ae39) - add cocogitto - [oknozor](https://github.com/oknozor)

[b75647](https://github.com/oknozor/musicbrainz_rs/commit/b756478b8215742e8cf1e18140b7a7017d97aece) - move to github action - [oknozor](https://github.com/oknozor)

[0fb73c](https://github.com/oknozor/musicbrainz_rs/commit/0fb73c463a2af35140640453fe055aaf71505688) - add sonnar - [oknozor](https://github.com/oknozor)


### Tests

[8c510e](https://github.com/oknozor/musicbrainz_rs/commit/8c510e62449e47ee1f4cbde4ae377cca217b0383) - entity annotations - [ritiek](https://github.com/ritiek)

[96a8f8](https://github.com/oknozor/musicbrainz_rs/commit/96a8f8833818e5d99f9803a567a2b29012f30db4) - execute area unit-tests - [ritiek](https://github.com/ritiek)

[f85ef7](https://github.com/oknozor/musicbrainz_rs/commit/f85ef7279e3cc352d351370142d0069e18a8e1d8) - refactor integration test organisation - [oknozor](https://github.com/oknozor)


### Miscellaneous Chores

[1af42a](https://github.com/oknozor/musicbrainz_rs/commit/1af42a6bbd825d6650f92122eefdfdeb61f18df5) - fix typo in cog config - [oknozor](https://github.com/oknozor)

[0f461f](https://github.com/oknozor/musicbrainz_rs/commit/0f461fae8f73c0e33cf91447733caea7794ab530) - bump lucene query builder to 0.2.4 - [oknozor](https://github.com/oknozor)

[04fbd2](https://github.com/oknozor/musicbrainz_rs/commit/04fbd288eae042ba943d89dddf85bd7fdff27e00) - lucene query builder to 0.2.3 - [oknozor](https://github.com/oknozor)

[927f60](https://github.com/oknozor/musicbrainz_rs/commit/927f60ff6f9f18eb6ec6759ae214c30d3e0202e5) - fmt all - [oknozor](https://github.com/oknozor)

[34d99a](https://github.com/oknozor/musicbrainz_rs/commit/34d99a37b8c2f39075b7ceb964de45a2f4c649db) - disable places test until next MB release - [oknozor](https://github.com/oknozor)

[1a3e6e](https://github.com/oknozor/musicbrainz_rs/commit/1a3e6ea2a0fa7ef1fabcbeff9e7536cbaed8e188) - version 0.1.4 - [oknozor](https://github.com/oknozor)

[c2c6ce](https://github.com/oknozor/musicbrainz_rs/commit/c2c6ce842e2537e2d67e3b844ce0e9738c661975) - version 0.1.3 - [oknozor](https://github.com/oknozor)

[05da0b](https://github.com/oknozor/musicbrainz_rs/commit/05da0bdc992bb44a43dbd346b3d8a6437152c5aa) - release 0.1.2 - [oknozor](https://github.com/oknozor)

[f38a0a](https://github.com/oknozor/musicbrainz_rs/commit/f38a0a798cbae8ba8c4f81dc97aebab3dcb055c1) - remove useless print - [oknozor](https://github.com/oknozor)

[1c8e11](https://github.com/oknozor/musicbrainz_rs/commit/1c8e11192707a3a279f8e23e32b3244abd6fc692) - remove useless bin folder - [oknozor](https://github.com/oknozor)

[27c8f4](https://github.com/oknozor/musicbrainz_rs/commit/27c8f40861a9aa18e395c16a5ec29b43d34d33dc) - alpha 0.1.1 - [oknozor](https://github.com/oknozor)

[75c7e2](https://github.com/oknozor/musicbrainz_rs/commit/75c7e250bf6297fc141ee22772c39a827ab081cb) - Create LICENSE - [oknozor](https://github.com/oknozor)


### Refactoring

[763f3b](https://github.com/oknozor/musicbrainz_rs/commit/763f3b1d7d87f9d10927abed8f36e81d75d8fb43) - add macro generated builder functions for browse query - [oknozor](https://github.com/oknozor)

[92ef0d](https://github.com/oknozor/musicbrainz_rs/commit/92ef0ddac8d30398995fa8c4fbcc3755847556b6) - add macro generated builder functions for includes - [oknozor](https://github.com/oknozor)

[5bbea4](https://github.com/oknozor/musicbrainz_rs/commit/5bbea4b2ace03f39750ea7593f34b8e0e3b48eef) - use Default impl instead of custom ones - [oknozor](https://github.com/oknozor)

[ff8931](https://github.com/oknozor/musicbrainz_rs/commit/ff8931eead07a5fbb0db83b497f017e36c54f4f8) - rename test files - [oknozor](https://github.com/oknozor)

[0f9e2e](https://github.com/oknozor/musicbrainz_rs/commit/0f9e2e6ea871b4ed5dd4b8794b79b8d199656834) - split query trait into specialized fetch and browse trait - [oknozor](https://github.com/oknozor)

[58c45c](https://github.com/oknozor/musicbrainz_rs/commit/58c45c5b27ea9c2b05ffae4846e70c274037761b) - extract http param literals to consts - [oknozor](https://github.com/oknozor)

[9123d7](https://github.com/oknozor/musicbrainz_rs/commit/9123d7348dff7c2a287704ecac6f9da8c7904e1e) - use lazy static http client - [oknozor](https://github.com/oknozor)

[ca430a](https://github.com/oknozor/musicbrainz_rs/commit/ca430aeb36c842498c43a4df73c1509912ae2ff0) - make artists include tests more explicit - [oknozor](https://github.com/oknozor)

[6f8e03](https://github.com/oknozor/musicbrainz_rs/commit/6f8e0321c0748f6a6cea58c957652443ea8f1561) - make label include tests more explicit - [oknozor](https://github.com/oknozor)

[1e74cb](https://github.com/oknozor/musicbrainz_rs/commit/1e74cbc8ff0971242e5c0b0bc7f1def6d36e1932) - correct yaml lints - [oknozor](https://github.com/oknozor)

[1d3efa](https://github.com/oknozor/musicbrainz_rs/commit/1d3efae1a82d0d69912a74e999b966724da3c605) - use serde global deserialization policy to force kebab-case globally - [oknozor](https://github.com/oknozor)

[f16872](https://github.com/oknozor/musicbrainz_rs/commit/f168723789f0b4d82dd4fe2fa2d00eb3b96defb9) - rename package to according to MusicBrainz guidelines - [oknozor](https://github.com/oknozor)

[7b51b3](https://github.com/oknozor/musicbrainz_rs/commit/7b51b373dfdcbb11ae2551552b0e83d52e624db3) - clean imports - [oknozor](https://github.com/oknozor)

[032528](https://github.com/oknozor/musicbrainz_rs/commit/032528415658e8e82012997b7c037455467ed9bb) - generify http query - [oknozor](https://github.com/oknozor)

[387d78](https://github.com/oknozor/musicbrainz_rs/commit/387d786e8a2c5cc8346ae259868115e7a9214d17) - clean imports - [oknozor](https://github.com/oknozor)


### Features

[15cca3](https://github.com/oknozor/musicbrainz_rs/commit/15cca3ca4160cb1b372592dc321eae5296a79055) - add search query - [oknozor](https://github.com/oknozor)

[7b726c](https://github.com/oknozor/musicbrainz_rs/commit/7b726c0d34c53d6e76889ef3eec2a02525688e3d) - add search entity - [oknozor](https://github.com/oknozor)

[932f75](https://github.com/oknozor/musicbrainz_rs/commit/932f75117be29068ac4ea9925686f95c8f2ec216) - add all browses for works - [oknozor](https://github.com/oknozor)

[32593f](https://github.com/oknozor/musicbrainz_rs/commit/32593f2d7a60468c0f4703d96c5b768de19d3f97) - add all browses for release_groups - [oknozor](https://github.com/oknozor)

[939180](https://github.com/oknozor/musicbrainz_rs/commit/939180aa9b1e94350ab9a80472eddefb98ea8df1) - add all browses for releases - [oknozor](https://github.com/oknozor)

[00ffbb](https://github.com/oknozor/musicbrainz_rs/commit/00ffbbfdf0fa514212f02628e2fced5c644addb3) - add all browses for recordings - [oknozor](https://github.com/oknozor)

[32f528](https://github.com/oknozor/musicbrainz_rs/commit/32f5289d9cb33746718efe9bec4c68a47a6965d7) - add all browses for places - [oknozor](https://github.com/oknozor)

[c1f6e1](https://github.com/oknozor/musicbrainz_rs/commit/c1f6e1fe356a08b62b4c4a427f93cfc196d529df) - add all browses for labels - [oknozor](https://github.com/oknozor)

[49754f](https://github.com/oknozor/musicbrainz_rs/commit/49754ffb6c5454c4e2fd0262786069f442b3a4b5) - add all browses for events - [oknozor](https://github.com/oknozor)

[1f1257](https://github.com/oknozor/musicbrainz_rs/commit/1f1257fd5a369dd9cb9bc752c36b4d16a9caf10e) - add all browses for artists - [oknozor](https://github.com/oknozor)

[3b5ecd](https://github.com/oknozor/musicbrainz_rs/commit/3b5ecd748ee8042ebe95310847ee6df7a4442a41) - add a deserialize impl for generic browse result - [oknozor](https://github.com/oknozor)

[bb569d](https://github.com/oknozor/musicbrainz_rs/commit/bb569d056a9103adc653e08b3bd8ffbd71a4560a) - make http client configurable - [oknozor](https://github.com/oknozor)

[ca3b42](https://github.com/oknozor/musicbrainz_rs/commit/ca3b427ab48b49e4afbd5fb84d4c1977e4341681) - partially add annotations - [oknozor](https://github.com/oknozor)

[f5ba96](https://github.com/oknozor/musicbrainz_rs/commit/f5ba964c3eba81e41e68e3e74199afab10dcb038) - add genres - [oknozor](https://github.com/oknozor)

[72f5f6](https://github.com/oknozor/musicbrainz_rs/commit/72f5f6547178ffa4ce000497c8529d9f5015ee5e) - add rating everywhere - [oknozor](https://github.com/oknozor)

[9b3bb2](https://github.com/oknozor/musicbrainz_rs/commit/9b3bb25764ac1b8013f1e7917e05d1cd17cf9b9e) - add aliases everywhere - [oknozor](https://github.com/oknozor)

[14069d](https://github.com/oknozor/musicbrainz_rs/commit/14069d4863c1fb3f55ac9b7f2378b6818aa2dc43) - add tags - [oknozor](https://github.com/oknozor)

[fb8c8d](https://github.com/oknozor/musicbrainz_rs/commit/fb8c8d64498adce20de654e62bd4b8eb70309953) - add media - [oknozor](https://github.com/oknozor)

[ce22b1](https://github.com/oknozor/musicbrainz_rs/commit/ce22b1b979011471942f6a16381d26d16a6107bc) - add release group includes - [oknozor](https://github.com/oknozor)

[e70a7f](https://github.com/oknozor/musicbrainz_rs/commit/e70a7fc03809537f0e330c084cadfd0def02a7e0) - add release includes - [oknozor](https://github.com/oknozor)

[965426](https://github.com/oknozor/musicbrainz_rs/commit/965426f8ff31b48a49fe7dd1ffeb87d5818f3ff3) - add recording incs & label aliases - [oknozor](https://github.com/oknozor)

[03a4d8](https://github.com/oknozor/musicbrainz_rs/commit/03a4d81c1511d5d1d5252c4c1bbc4eca28cc1e1c) - add label releases - [oknozor](https://github.com/oknozor)

[201953](https://github.com/oknozor/musicbrainz_rs/commit/201953bd943a6637155f18be896cc2938687fbe8) - add url - [oknozor](https://github.com/oknozor)

[397b49](https://github.com/oknozor/musicbrainz_rs/commit/397b497a18e89048b7348c3c100ef6cade4b3ec3) - add serie - [oknozor](https://github.com/oknozor)

[20774d](https://github.com/oknozor/musicbrainz_rs/commit/20774d58794f9b0ac6afa4f56d5ec4a8c1afc574) - add place - [oknozor](https://github.com/oknozor)

[3f9ed7](https://github.com/oknozor/musicbrainz_rs/commit/3f9ed73e39871d586bca5687c04d360d7dd05fcd) - add instrument - [oknozor](https://github.com/oknozor)

[bed043](https://github.com/oknozor/musicbrainz_rs/commit/bed043d70e1b36fc00204b138947079468426147) - add event - [oknozor](https://github.com/oknozor)

[7925d1](https://github.com/oknozor/musicbrainz_rs/commit/7925d1a1b4b9a78321470721f185c81b711b116b) - bound include to each entity separatly - [oknozor](https://github.com/oknozor)

[1a4841](https://github.com/oknozor/musicbrainz_rs/commit/1a4841ea02517a3f454859537f378f8ebd113a8f) - add aliases and artist alias relation - [oknozor](https://github.com/oknozor)

[a18c9a](https://github.com/oknozor/musicbrainz_rs/commit/a18c9af78ef7234d1a0cb9bf12db00d37f86ae78) - add recording and releases-groups incs for artists - [oknozor](https://github.com/oknozor)

[571d78](https://github.com/oknozor/musicbrainz_rs/commit/571d7851c735a96b2f47cd11521eaa57f5065024) - add rustfmt - [oknozor](https://github.com/oknozor)

[ef79a8](https://github.com/oknozor/musicbrainz_rs/commit/ef79a87c6be9b797cf2762c6895b627fd5cd045e) - add the ability to use inc params - [oknozor](https://github.com/oknozor)

[6de2c9](https://github.com/oknozor/musicbrainz_rs/commit/6de2c9202a24187618c3b9a0b62475784e3ccb11) - add a generic way to build request with fluent syntax - [oknozor](https://github.com/oknozor)

[b426b8](https://github.com/oknozor/musicbrainz_rs/commit/b426b87ccae54b364b58795e8b67c8425891a7a7) - add relations to model - [oknozor](https://github.com/oknozor)

[55d33a](https://github.com/oknozor/musicbrainz_rs/commit/55d33aeb1ef839fefef6c686bd16c0bb289bb3c7) - add area: GET, move area and lifespan to dedicated modules - [oknozor](https://github.com/oknozor)

[020ee4](https://github.com/oknozor/musicbrainz_rs/commit/020ee40569a9019c47b62684ca775876d488a149) - add label - [oknozor](https://github.com/oknozor)

[ce2021](https://github.com/oknozor/musicbrainz_rs/commit/ce2021f24ed9f073dc29349f2a6449cf49c55799) - add work - [oknozor](https://github.com/oknozor)

[4bddf6](https://github.com/oknozor/musicbrainz_rs/commit/4bddf64116dd150824612b1921e6ec401a237438) - add release - [oknozor](https://github.com/oknozor)

[0274c9](https://github.com/oknozor/musicbrainz_rs/commit/0274c910019a950169de8c2890d54b33113d8935) - add release group and refactor doc comments - [oknozor](https://github.com/oknozor)

[72c2f6](https://github.com/oknozor/musicbrainz_rs/commit/72c2f660e3f5bbdd8e0b405cf0fea8a4f7998fa2) - add build status image - [oknozor](https://github.com/oknozor)

[b1dda8](https://github.com/oknozor/musicbrainz_rs/commit/b1dda857809b3d3b59f5f483265e1242879d6e83) - add travis configuration - [oknozor](https://github.com/oknozor)

[2d66df](https://github.com/oknozor/musicbrainz_rs/commit/2d66df62a528e4231123b952af8550d3a146ec9c) - add recording - [oknozor](https://github.com/oknozor)

[be48bf](https://github.com/oknozor/musicbrainz_rs/commit/be48bfcc2dc27620530c2ba0368362c67ea6576e) - split model in a dedicated module - [oknozor](https://github.com/oknozor)

[3bf93f](https://github.com/oknozor/musicbrainz_rs/commit/3bf93f1222b3e1a2c61351eec5096f569c069381) - add optional life time to areas, for search route - [oknozor](https://github.com/oknozor)


### Documentation

[045b8c](https://github.com/oknozor/musicbrainz_rs/commit/045b8c4f0f5c49498fef1db914c0c33ca10999f6) - add crate level doc and fix visibility - [oknozor](https://github.com/oknozor)

[7295f3](https://github.com/oknozor/musicbrainz_rs/commit/7295f3a2180a321e7f7cc1b1769f517b72428dbb) - add search example - [oknozor](https://github.com/oknozor)

[6983f4](https://github.com/oknozor/musicbrainz_rs/commit/6983f4c0948dc76d1c65d702b4a02124751f5906) - update README - [oknozor](https://github.com/oknozor)

[cc872b](https://github.com/oknozor/musicbrainz_rs/commit/cc872b1b087da9517d30e58b7900a6905126e4bf) - update doc for browse query - [oknozor](https://github.com/oknozor)

[114129](https://github.com/oknozor/musicbrainz_rs/commit/114129cd7a2eb2ee5b6b805135eeea87bd27f006) - update examples and readme - [oknozor](https://github.com/oknozor)

[c06897](https://github.com/oknozor/musicbrainz_rs/commit/c068973e50951666ab394c0ec0c202993a53d452) - move model status to github milestone - [oknozor](https://github.com/oknozor)

[30db59](https://github.com/oknozor/musicbrainz_rs/commit/30db592cd40852932d29e6cf927e0a8d587f0c2f) - update README - [oknozor](https://github.com/oknozor)

[bc3269](https://github.com/oknozor/musicbrainz_rs/commit/bc326978bf36dbe62d24995323b1b7d61db69a1f) - update manifest - [oknozor](https://github.com/oknozor)

[3284e4](https://github.com/oknozor/musicbrainz_rs/commit/3284e498b469aa540ba5f19090884f7864cf7dda) - update readme - [oknozor](https://github.com/oknozor)

[22ca63](https://github.com/oknozor/musicbrainz_rs/commit/22ca6333a63b79cf6ca279fb213afe59d798fe3e) - update manifest - [oknozor](https://github.com/oknozor)

[19d317](https://github.com/oknozor/musicbrainz_rs/commit/19d3176c8a8063a684eeff6c86a00ec7d6780955) - update README - [oknozor](https://github.com/oknozor)

[7a6ce1](https://github.com/oknozor/musicbrainz_rs/commit/7a6ce1f62587645999e4a940700424dfd516bcc7) - add begin area - [oknozor](https://github.com/oknozor)

[8ba26a](https://github.com/oknozor/musicbrainz_rs/commit/8ba26af15902efd47c7f671f28a3d7fe841a2667) - add info about life span add format doc - [oknozor](https://github.com/oknozor)


### Bug Fixes

[74304c](https://github.com/oknozor/musicbrainz_rs/commit/74304c4ef04314b2ea19c870e4aa4cc947f92673) - fix created date format in search response - [oknozor](https://github.com/oknozor)

[fbb8fc](https://github.com/oknozor/musicbrainz_rs/commit/fbb8fc6f8ce991426ec14936ccebfa7caa47ad70) - no sonar for rustacean - [oknozor](https://github.com/oknozor)

[49a4a7](https://github.com/oknozor/musicbrainz_rs/commit/49a4a711bcea56773d1005eeb6112363f32976c9) - temporary fix for musicbrainz api rate limit - [oknozor](https://github.com/oknozor)

[9cdaba](https://github.com/oknozor/musicbrainz_rs/commit/9cdaba602bceb2f676acbc8ef845c95bd0a6e3fe) - add build and test to new config - [oknozor](https://github.com/oknozor)

[e8b419](https://github.com/oknozor/musicbrainz_rs/commit/e8b419c287e5c794fcbd29eae89248ae9b3fd385) - typo - [oknozor](https://github.com/oknozor)


### Style

[e84d35](https://github.com/oknozor/musicbrainz_rs/commit/e84d35b8dfc676f1cb99bb8663d2ee5c25bd0031) - rustfmt all - [oknozor](https://github.com/oknozor)


- - -

This changelog was generated by [cocogitto](https://github.com/oknozor/cocogitto).