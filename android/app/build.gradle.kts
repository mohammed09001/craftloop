// Execution 01, Phase 28, Task 203. See ../README.md -- placeholder
// shell, not build-validated in this environment.

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "com.craftloop.shell"
    // Chosen for a tablet-class device (Samsung Galaxy Tab S-series,
    // Task 205's validation target), not a phone-first minimum.
    compileSdk = 35

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
