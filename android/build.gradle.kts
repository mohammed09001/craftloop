// Execution 01, Phase 28, Task 203. Root build script -- plugin version
// pinning only, all real configuration lives in app/build.gradle.kts.

plugins {
    id("com.android.application") version "8.6.1" apply false
    id("org.jetbrains.kotlin.android") version "2.0.21" apply false
    // Execution 02, Phase 02, Task 014. Kotlin 2.0 decoupled the Compose
    // compiler from the Kotlin compiler itself -- as of Kotlin 2.0, this
    // plugin must be applied explicitly wherever `buildFeatures.compose =
    // true` is set, or Gradle project configuration fails outright. Found
    // by actually running a Gradle build for the first time in this
    // execution (Execution 01 never build-validated this project at all).
    id("org.jetbrains.kotlin.plugin.compose") version "2.0.21" apply false
}
