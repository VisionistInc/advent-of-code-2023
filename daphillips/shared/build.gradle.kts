
plugins {
    id("aoc-2023-kotlin-conventions")
}

kotlin {
    sourceSets {

        commonMain.dependencies {
            implementation("com.squareup.okio:okio:3.6.0")
        }

        val jsMain by getting {
            dependencies {
                implementation("com.squareup.okio:okio-nodefilesystem:3.6.0")
            }
        }

        val wasmJsMain by getting {
            dependencies {
//                implementation("com.squareup.okio:okio-wasifilesystem:3.6.0")
            }
        }

        val wasmWasiMain by getting {
            dependencies {
//                implementation("com.squareup.okio:okio-wasifilesystem:3.6.0")
            }
        }

    }

}