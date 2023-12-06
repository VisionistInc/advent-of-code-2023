
import okio.FileSystem
import okio.Path


internal expect val FILE_SYSTEM: FileSystem
expect val INPUT_FILE: String

expect fun getLinesFromFile(path: String): List<String>
internal fun getLinesFromFile(path: Path) = FILE_SYSTEM.read(path) {
    readUtf8()
}.lines()
