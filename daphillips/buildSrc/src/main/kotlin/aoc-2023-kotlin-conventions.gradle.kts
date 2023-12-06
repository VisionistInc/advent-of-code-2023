plugins {
    kotlin("multiplatform")
}

kotlin { // Extension for easy setup
    jvmToolchain(libs.jvm)

    jvm()
    js().nodejs()
    getNativeHostTarget()
    wasmJs {
        nodejs()
    }
    wasmWasi {
        nodejs()
    }

    // TODO consider hierarchical source sets
    sourceSets {
        val nativeHostMain by getting
        val nativeHostTest by getting
    }
    
}

repositories {
    mavenCentral()
}


//rootProject.the<NodeJsRootExtension>().apply {
//    // this version was able to work with a "hello world" wasm target
////    nodeVersion = "21.0.0-v8-canary202309143a48826a08"
//    nodeVersion = "22.0.0-v8-canary202311294a3904c9c5"
//    nodeDownloadBaseUrl = "https://nodejs.org/download/v8-canary"
//}

//rootProject.plugins.withType<org.jetbrains.kotlin.gradle.targets.js.nodejs.NodeJsRootPlugin> {
//    rootProject.the<org.jetbrains.kotlin.gradle.targets.js.nodejs.NodeJsRootExtension>().download = false
//    // "true" for default behavior
//}

rootProject.tasks.withType<org.jetbrains.kotlin.gradle.targets.js.npm.tasks.KotlinNpmInstallTask>().configureEach {
    args.add("--ignore-engines")
}
