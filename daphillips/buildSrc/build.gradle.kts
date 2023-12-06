plugins {
    `kotlin-dsl`
}

dependencies {
    implementation(libs.plugins.kotlin.mpp)
}

// This is a future Gradle feature to make declaring dependencies to plugins simpler
// See an upvote https://docs.google.com/document/d/1P7aTeeVNhkhwxcS5sQNFrSsmqJOhDo3G8kUdhtp_vyM
fun DependencyHandler.implementation(pluginDependency: Provider<PluginDependency>): Dependency? =
    add("implementation", pluginDependency.map {
        "${it.pluginId}:${it.pluginId}.gradle.plugin:${it.version.requiredVersion}"
    }.get())
