<p align="center">
    <img src="./Kotlin Full Color Logo Mark RGB.svg" width="250" height="250">
</p>

# AOC 2023 in [Kotlin Multiplatform](https://kotlinlang.org/docs/multiplatform-get-started.html)
Kotlin is a pretty great language. So great, that the devs decided that it should target more than just the JVM or mobile. Kotlin has a bunch of different [platform targets](https://kotlinlang.org/docs/multiplatform-dsl-reference.html#targets), including JVM, Native, JS, and WASM. This year, an attempt is made to make kotlin submissions that can run on _any_ of these platforms.

## Structure
This project has a unique gradle module for each day, numbered `XX`. In addition, the `shared` module contains common library code for each day (namely for file I/O).

This project also uses "modern" gradle features, such as the [library catalog](https://docs.gradle.org/current/userguide/platforms.html)/[`libs.versions.toml`](https://docs.gradle.org/current/userguide/platforms.html#sub:conventional-dependencies-toml) and [conventions plugins](https://docs.gradle.org/current/userguide/sharing_build_logic_between_subprojects.html#sec:convention_plugins) in `buildSrc

## Building/Running
Things _should_ be as simple as navigating to one of the submission days and running `./gradlew runAll`. This will build and run the submission for each platform. 