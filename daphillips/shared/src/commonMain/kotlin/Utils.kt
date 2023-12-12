import kotlin.time.measureTimedValue

inline fun <T> printDuration(body: () -> T) = measureTimedValue {
    body()
}.also {
    println(it.value)
    println("Took ${it.duration}")
}
