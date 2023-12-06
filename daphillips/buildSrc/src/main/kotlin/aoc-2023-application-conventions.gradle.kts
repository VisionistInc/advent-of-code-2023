plugins {
    id("aoc-2023-kotlin-conventions")
//    application
}

kotlin {
    jvm().mainRun {
        mainClass.set("MainKt")
    }

    js {
        binaries.executable()
    }

    wasmJs {
        binaries.executable()
    }

    wasmWasi {
        binaries.executable()
    }
    // TODO nativeMain

    getNativeHostTarget().apply {
        binaries.executable()
    }

    sourceSets {
        commonMain.dependencies {
            implementation(project(":shared"))
        }
    }
}

tasks.register("runAll") {
    dependsOn("jvmRun")
    dependsOn("jsNodeRun")
    dependsOn("runReleaseExecutableNativeHost")
//    dependsOn("wasmJsNodeRun")
}

//application {
//    mainClass.set("MainKt")
//}

repositories {
    mavenCentral()
}