// Execution 01, Phase 28, Task 203. See README.md — placeholder shell,
// not build-validated in this environment (no Android SDK/Gradle here).

pluginManagement {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}

dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        mavenCentral()
    }
}

rootProject.name = "craftloop-android-shell"
include(":app")
