import okio.FileSystem
import okio.Path.Companion.toPath

internal actual val FILE_SYSTEM = FileSystem.SYSTEM
actual val INPUT_FILE = "src/commonMain/resources/input.txt".toPath().toString()

actual fun getLinesFromFile(path: String) = getLinesFromFile(path.toPath())