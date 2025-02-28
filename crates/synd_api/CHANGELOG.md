# Changelog

All notable changes to this project will be documented in this file.

## [v0.1.8] 2024-03-23

### Miscellaneous Tasks

- Disable cargo-dist due to .cargo/config by [@ymgyt](https://github.com/ymgyt) ([7407a67e](https://github.com/ymgyt/syndicationd/commit/7407a67e1b730e079016e138e58495c75354f456))

## [v0.1.7] - 2024-03-23

### Features

- Use tokio-metrics to get runtime metrics by [@ymgyt](https://github.com/ymgyt) ([ed3b881d](https://github.com/ymgyt/syndicationd/commit/ed3b881d37888a9bdecb7653e6b9a46c5c71bd22))

## [v0.1.6] - 2024-03-17

### Features

- Set graphql depth and complexity limit by [@ymgyt](https://github.com/ymgyt) ([54b44e88](https://github.com/ymgyt/syndicationd/commit/54b44e889ec4c50d78ed1b1142ccdf964b69b056))
- Set graphql depth and complexity limit for introspection by [@ymgyt](https://github.com/ymgyt) ([f6db33dc](https://github.com/ymgyt/syndicationd/commit/f6db33dc4a36390af6d7b573d32f763588a5e516))
- Support google oidc by [@ymgyt](https://github.com/ymgyt) ([c7c81fd8](https://github.com/ymgyt/syndicationd/commit/c7c81fd8e786ea89c977abc107c39fc521135553))

### Miscellaneous Tasks

- Remove unused headers crate by [@ymgyt](https://github.com/ymgyt) ([2eaf00db](https://github.com/ymgyt/syndicationd/commit/2eaf00dbd5ce5984763ae4add18a6150de1c213d))
- Inc gql complexity limit by [@ymgyt](https://github.com/ymgyt) ([f7ac84b5](https://github.com/ymgyt/syndicationd/commit/f7ac84b58a7db7846f84b4e5c88d249b4b2f16c1))

### Refactor

- Use cfg macro to configure gql schema by [@ymgyt](https://github.com/ymgyt) ([d2a1f551](https://github.com/ymgyt/syndicationd/commit/d2a1f551d72d49d68ce288f6c06753aa55b57fdd))

## [v0.1.4] - 2024-02-25

### Features

- Add serve options by [@ymgyt](https://github.com/ymgyt) ([c06cd23b](https://github.com/ymgyt/syndicationd/commit/c06cd23b23ff2e62b57156338330967f59f7b822))
- More verbose at startup by [@ymgyt](https://github.com/ymgyt) ([1e9f6550](https://github.com/ymgyt/syndicationd/commit/1e9f6550f5cea8762633021f181d4dc12c439a63))
- Use health check response type for health check by [@ymgyt](https://github.com/ymgyt) ([ceff27f5](https://github.com/ymgyt/syndicationd/commit/ceff27f5d5d0a1aa0a3a3751335fe57e4f0bfcdb))

### Miscellaneous Tasks

- Configure cargo-dist by [@ymgyt](https://github.com/ymgyt) ([1da44d02](https://github.com/ymgyt/syndicationd/commit/1da44d0261b5266566d8d8c97147a6bb7053305a))
- Temporary disable Result::inspect to compile with 1.75 in macos runner by [@ymgyt](https://github.com/ymgyt) ([88d1634a](https://github.com/ymgyt/syndicationd/commit/88d1634a6d9d0c6c2f92cb358008e76e98c3d6b3))
- Update cargo-dist to 0.11 by [@ymgyt](https://github.com/ymgyt) ([6e75b48d](https://github.com/ymgyt/syndicationd/commit/6e75b48d1b56e08ea8212f297864aa8f7c70d4e6))
- Add homepage to package metadata by [@ymgyt](https://github.com/ymgyt) ([4bfdb49e](https://github.com/ymgyt/syndicationd/commit/4bfdb49e317e18ff6345ce1b8e8071f0497a1a5f))
- Release by [@ymgyt](https://github.com/ymgyt) ([ed5bd533](https://github.com/ymgyt/syndicationd/commit/ed5bd533aeac806fbd047de76ac86920a90ac0e2))

### Refactor

- Refactor flags by [@ymgyt](https://github.com/ymgyt) ([7e2a91d9](https://github.com/ymgyt/syndicationd/commit/7e2a91d93a8ed1afd7955583a2dc9ff9c6289a4f))

## [v0.1.3] - 2024-02-23

### Features

- Handle subscribe feed error by [@ymgyt](https://github.com/ymgyt) ([90c47d3f](https://github.com/ymgyt/syndicationd/commit/90c47d3f8e225cb71f33b1e6d6df0f0735e21f73))
- Use updated if published is none by [@ymgyt](https://github.com/ymgyt) ([9967dc10](https://github.com/ymgyt/syndicationd/commit/9967dc108f7f6602e321808398737f891462ec81))
- Add generator resolver in feed by [@ymgyt](https://github.com/ymgyt) ([f8de4aa4](https://github.com/ymgyt/syndicationd/commit/f8de4aa4a9a4edb8d1f7e8dd31c53b2e66360b18))
- Resolve entry content if there is no summary by [@ymgyt](https://github.com/ymgyt) ([0459e71c](https://github.com/ymgyt/syndicationd/commit/0459e71c38aba96b4d878ce97cd35ed78587032b))

### Miscellaneous Tasks

- Trim prefix from changelog by [@ymgyt](https://github.com/ymgyt) ([95d44877](https://github.com/ymgyt/syndicationd/commit/95d448773ec7ab009fbece0928854364679b6f2c))

## [v0.1.2] - 2024-02-20

### Features

- Raise soft fd limit by [@ymgyt](https://github.com/ymgyt) ([54e7ba3c](https://github.com/ymgyt/syndicationd/commit/54e7ba3c44a4a379e61edea95bc27c487fa0b7d6))
- Instrument kvsd client span name by [@ymgyt](https://github.com/ymgyt) ([bced1b62](https://github.com/ymgyt/syndicationd/commit/bced1b62a52e79e0af70fb2177a2efa940adf91c))
- Use monotonic_counter as feed subscription metrics by [@ymgyt](https://github.com/ymgyt) ([670dc430](https://github.com/ymgyt/syndicationd/commit/670dc4300310695a71ee73db90f066309323ba6b))

### Bug Fixes

- Strict fetch feed in flight limit by [@ymgyt](https://github.com/ymgyt) ([5a2b646e](https://github.com/ymgyt/syndicationd/commit/5a2b646e2d4fa2b24a2f115a27288c922fa87af3))

## [v0.1.1] - 2024-02-19

### Features

- Add o11y crate by [@ymgyt](https://github.com/ymgyt) ([0a50517e](https://github.com/ymgyt/syndicationd/commit/0a50517e0b861973fac95ad5dba6f2c4d5b7270d))
- Add opentelemetry-tracing log bridge layer by [@ymgyt](https://github.com/ymgyt) ([92f22b56](https://github.com/ymgyt/syndicationd/commit/92f22b564357a0d43f8631212cf976338eb05a04))
- Add baggage propagation by [@ymgyt](https://github.com/ymgyt) ([d02e514c](https://github.com/ymgyt/syndicationd/commit/d02e514c8f6e32aa748c10dadb204153cba21ecc))
- Add opentelemetry layers by [@ymgyt](https://github.com/ymgyt) ([4d3f5bf3](https://github.com/ymgyt/syndicationd/commit/4d3f5bf3f45f31cfd014dbdf37a41a31ea0472ca))
- Add palette flag by [@ymgyt](https://github.com/ymgyt) ([04dc486d](https://github.com/ymgyt/syndicationd/commit/04dc486d0ab3043e021e164e70f5fe081e3c464d))
- Impl kvsd client by [@ymgyt](https://github.com/ymgyt) ([6ae6de7a](https://github.com/ymgyt/syndicationd/commit/6ae6de7a2e783417b1a8d5d3c2b450109d83725f))
- Use kvsd by [@ymgyt](https://github.com/ymgyt) ([19eaeada](https://github.com/ymgyt/syndicationd/commit/19eaeadab75be9ea0c7c95e65ca654f9842707af))
- Remove unsubscribed entries by [@ymgyt](https://github.com/ymgyt) ([d29ba92e](https://github.com/ymgyt/syndicationd/commit/d29ba92e929d9d1348fa114ac2bdf210b76c5a1b))
- Serve https by [@ymgyt](https://github.com/ymgyt) ([fbb9011e](https://github.com/ymgyt/syndicationd/commit/fbb9011e86acf6e4cf30f37a74e67d3202bbc5a0))
- Support axum_server graceful shutdown by [@ymgyt](https://github.com/ymgyt) ([880b6d3e](https://github.com/ymgyt/syndicationd/commit/880b6d3e8d0f90b711a1d6e8e1bf6fb1808e5161))
- Instrument request counter metrics by [@ymgyt](https://github.com/ymgyt) ([ac64b3aa](https://github.com/ymgyt/syndicationd/commit/ac64b3aa6880482597e672649de800eb30b3ad56))
- Export basic counter metrics by [@ymgyt](https://github.com/ymgyt) ([13ba79b7](https://github.com/ymgyt/syndicationd/commit/13ba79b7a20f5b9b573e7285a02302d8dc848b03))
- Add fallback handler by [@ymgyt](https://github.com/ymgyt) ([681d0315](https://github.com/ymgyt/syndicationd/commit/681d0315b49c1b2a157d3141f0e45be95e32272e))
- Remove path attribute from http request count metrics by [@ymgyt](https://github.com/ymgyt) ([017470e5](https://github.com/ymgyt/syndicationd/commit/017470e50f38b26270cfa9e3c1d85a588b23e725))

### Miscellaneous Tasks

- Format toml by [@ymgyt](https://github.com/ymgyt) ([36677745](https://github.com/ymgyt/syndicationd/commit/3667774506106fe0f38d77efac9f4b27c70090aa))
- Organize dev files by [@ymgyt](https://github.com/ymgyt) ([4af5df57](https://github.com/ymgyt/syndicationd/commit/4af5df57a38f69b734b3e4ceaf741b3415bed6e1))
- Configure release flow by [@ymgyt](https://github.com/ymgyt) ([855d1063](https://github.com/ymgyt/syndicationd/commit/855d1063f5b476433fe0a7ab352b72d63a749e2e))
- Use hyphen as package name instead of underscore by [@ymgyt](https://github.com/ymgyt) ([0a8ed059](https://github.com/ymgyt/syndicationd/commit/0a8ed05997790f9f05c932c92fa2b2b2d74065a9))
- Instrument by [@ymgyt](https://github.com/ymgyt) ([07839dc1](https://github.com/ymgyt/syndicationd/commit/07839dc10e7c44cae79055eea6103f099f0daf5e))
- Add bin section to Cargo.toml by [@ymgyt](https://github.com/ymgyt) ([9bfd56ef](https://github.com/ymgyt/syndicationd/commit/9bfd56ef41e27f094ef240653da47cdda662d2fb))

### Refactor

- Rename crates by [@ymgyt](https://github.com/ymgyt) ([ce0982e4](https://github.com/ymgyt/syndicationd/commit/ce0982e497647b23dcf07e39d525121bcd9ac1fa))
- Use clippy pedantic by [@ymgyt](https://github.com/ymgyt) ([328ddade](https://github.com/ymgyt/syndicationd/commit/328ddadebbad5381271c5e84cce2d6888252e70c))
- Rename datastore to repository by [@ymgyt](https://github.com/ymgyt) ([969c0052](https://github.com/ymgyt/syndicationd/commit/969c0052164a7719d5c8902a5fd70b40c42faae5))
- Rename to subscription repository by [@ymgyt](https://github.com/ymgyt) ([707ec5f3](https://github.com/ymgyt/syndicationd/commit/707ec5f3197b6079e420a8c5e2dc17c3efd7ed56))
- Rename to repository by [@ymgyt](https://github.com/ymgyt) ([aed9bb48](https://github.com/ymgyt/syndicationd/commit/aed9bb4873c2a286699898e2c37825e292811ee6))
- Fix lint by [@ymgyt](https://github.com/ymgyt) ([aac00b98](https://github.com/ymgyt/syndicationd/commit/aac00b98335bb75cc57fdea0875bfd675bf8f3cc))

### Testing

- Impl device flow test case by [@ymgyt](https://github.com/ymgyt) ([93572902](https://github.com/ymgyt/syndicationd/commit/9357290265a4fbf8d78721e4f9f1904b1cf5b12a))
- Add auth flow case by [@ymgyt](https://github.com/ymgyt) ([6d2b1905](https://github.com/ymgyt/syndicationd/commit/6d2b1905d9b06bd9ed670f210cd590f89405c37c))
- Run integration test by [@ymgyt](https://github.com/ymgyt) ([20c0bc2d](https://github.com/ymgyt/syndicationd/commit/20c0bc2d31a938d3103fafedba5a10b4a9bba9ae))

<!-- generated by git-cliff -->
