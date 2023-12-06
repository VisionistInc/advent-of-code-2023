import org.gradle.api.GradleException
import org.jetbrains.kotlin.gradle.dsl.KotlinMultiplatformExtension
import org.jetbrains.kotlin.gradle.plugin.mpp.KotlinNativeTarget

fun KotlinMultiplatformExtension.getNativeHostTarget(): KotlinNativeTarget {
    val hostOs = System.getProperty("os.name")
    val isArm64 = System.getProperty("os.arch") == "aarch64"
    val isMingwX64 = hostOs.startsWith("Windows")
    return when {
        hostOs == "Mac OS X" && isArm64 -> macosArm64("nativeHost")
        hostOs == "Mac OS X" && !isArm64 -> macosX64("nativeHost")
        hostOs == "Linux" && isArm64 -> linuxArm64("nativeHost")
        hostOs == "Linux" && !isArm64 -> linuxX64("nativeHost")
        isMingwX64 -> mingwX64("nativeHost")
        else -> throw GradleException("Host OS is not supported in Kotlin/Native.")
    }
}