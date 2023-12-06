import okio.FileSystem

internal actual val FILE_SYSTEM: FileSystem = TODO("Wasm doesn't work with file systems at the moment")
actual val INPUT_FILE: String = TODO("Wasm doesn't work with file systems at the moment")

actual fun getLinesFromFile(path: String): List<String> = TODO("Wasm doesn't work with file systems at the moment")