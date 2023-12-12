import kotlin.math.abs

data class Point(val row: Long, val col: Long) {
    constructor(row: Int, col: Int) : this(row.toLong(), col.toLong())

    fun moveBy(row: Int, col: Int) = moveBy(row.toLong(), col.toLong())
    fun moveBy(row: Long = 0, col: Long = 0) = copy(row = this.row + row, col = this.col + col)


    operator fun plus(other: Point) = moveBy(other.row, other.col)

    fun dist(other: Point) = abs(row - other.row) + abs(col - other.col)
}

infix fun Long.by(other: Long) = Point(this, other)
infix fun Int.by(other: Int) = Point(this, other)