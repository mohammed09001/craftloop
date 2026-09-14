// Execution 01, Phase 28, Task 203; Execution 02, Phase 03 (Tasks
// 017-021) automates what was a placeholder: the native Rust library
// and Kotlin bindings below are no longer manually copied from
// evidence folders (Article 19's explicit prohibition) -- two Gradle
// `Exec` tasks cross-compile `craftloop-mobile-ffi` via cargo-ndk and
// generate its Kotlin bindings via the crate's own `uniffi-bindgen`
// binary, wired as real inputs to `preBuild` so `assembleDebug` alone
// reproduces the whole chain on a clean checkout.

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("org.jetbrains.kotlin.plugin.compose")
}

// The Craft Loop repository root, one level above `android/`. Every
// path the Rust build tasks below need (Cargo.toml, the mobile-ffi
// crate, cargo/cargo-ndk on PATH) is resolved relative to this, not to
// `android/`, since the Rust workspace lives outside the Gradle
// project entirely (Article 4: shared core is platform-neutral and
// does not live under an Android-specific root).
val repoRoot = rootProject.projectDir.parentFile

// Execution 02, Phase 03/Task 020's pinned physical target -- Article
// 20's own policy ("First physical ABI: arm64-v8a"). x86_64 for
// emulator/CI is deliberately not added yet; no task before Phase 16
// (Android CI) needs it.
val cargoNdkTarget = "arm64-v8a"

// Matches `defaultConfig.minSdk` below -- cargo-ndk's `--platform`
// flag controls which Android API level the cross-compiled native
// library targets, and it must not silently diverge from the API
// level the rest of this module already commits to.
val ndkPlatform = 30

val rustJniLibsDir = layout.buildDirectory.dir("rustJniLibs")
val uniffiGeneratedDir = layout.buildDirectory.dir("generated/uniffi")

val cargoBuildAndroid =
    tasks.register<Exec>("cargoBuildAndroid") {
        description =
            "Cross-compiles craftloop-mobile-ffi for $cargoNdkTarget via cargo-ndk " +
                "(Execution 02, Phase 03, Tasks 017/018/020)."
        workingDir = repoRoot
        commandLine(
            "cargo", "ndk",
            "--target", cargoNdkTarget,
            "--platform", ndkPlatform.toString(),
            "-o", rustJniLibsDir.get().asFile.absolutePath,
            "build", "-p", "craftloop-mobile-ffi",
        )
        // AGP already resolves `ndkVersion` below to a concrete install
        // under the SDK; point cargo-ndk at that exact same NDK rather
        // than trusting an ambient ANDROID_NDK_HOME that could disagree
        // with it.
        environment("ANDROID_NDK_HOME", android.ndkDirectory.absolutePath)
        outputs.dir(rustJniLibsDir)
        // Cargo itself is the real incremental-build authority here
        // (it no-ops a rebuild with nothing changed); this task is
        // cheap to always re-invoke rather than trying to model Rust's
        // own dependency graph as Gradle task inputs.
        outputs.upToDateWhen { false }
    }

val generateUniffiBindings =
    tasks.register<Exec>("generateUniffiBindings") {
        description =
            "Generates Kotlin bindings from the compiled native library via the crate's own " +
                "uniffi-bindgen binary (Execution 02, Phase 03/05, Tasks 019/034)."
        dependsOn(cargoBuildAndroid)
        workingDir = repoRoot
        val soFile = rustJniLibsDir.get().asFile.resolve("$cargoNdkTarget/libcraftloop_mobile_ffi.so")
        commandLine(
            "cargo", "run", "--bin", "uniffi-bindgen", "-p", "craftloop-mobile-ffi", "--",
            "generate", "--library", soFile.absolutePath,
            "--language", "kotlin", "--out-dir", uniffiGeneratedDir.get().asFile.absolutePath,
            "--no-format",
        )
        outputs.dir(uniffiGeneratedDir)
        outputs.upToDateWhen { false }
    }

tasks.named("preBuild") {
    dependsOn(generateUniffiBindings)
}

android {
    namespace = "com.craftloop.shell"
    // Chosen for a tablet-class device (Samsung Galaxy Tab S-series,
    // Task 205's validation target), not a phone-first minimum.
    compileSdk = 35

    // Execution 02, Phase 01, Task 009/019: pinned, not "latest" --
    // matches `scripts/environment-doctor.ps1`'s own installed version.
    ndkVersion = "27.2.12479018"

    defaultConfig {
        applicationId = "com.craftloop.shell"
        minSdk = 30
        targetSdk = 35
        versionCode = 1
        versionName = "0.1.0-placeholder"
    }

    buildFeatures {
        compose = true
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }

    sourceSets {
        getByName("main") {
            jniLibs.srcDir(rustJniLibsDir)
        }
    }
}

kotlin {
    sourceSets {
        getByName("main") {
            kotlin.srcDir(uniffiGeneratedDir)
        }
    }
}

dependencies {
    implementation("androidx.core:core-ktx:1.13.1")
    implementation("androidx.activity:activity-compose:1.9.2")
    implementation(platform("androidx.compose:compose-bom:2024.09.02"))
    implementation("androidx.compose.ui:ui")
    implementation("androidx.compose.material3:material3")
    // UniFFI's generated Kotlin bindings need the JNA runtime to call
    // into the native craftloop_mobile_ffi library.
    implementation("net.java.dev.jna:jna:5.14.0@aar")
}
