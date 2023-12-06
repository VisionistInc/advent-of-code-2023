import org.gradle.api.Project
import org.gradle.api.artifacts.VersionCatalog
import org.gradle.api.artifacts.VersionCatalogsExtension
import org.gradle.kotlin.dsl.the

internal
val Project.libs: VersionCatalog
    get() = the<VersionCatalogsExtension>().named("libs")

internal
val VersionCatalog.jvm: Int
    get() = findVersion("jvm").get().requiredVersion.toInt()
