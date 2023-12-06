
expect fun getLines(inputOrPath: String): List<String>

fun main() {
    var lines = getLines(INPUT_FILE)
//    lines = ("467..114..\n" +
//            "...*......\n" +
//            "..35..633.\n" +
//            "......#...\n" +
//            "617*......\n" +
//            ".....+.58.\n" +
//            "..592.....\n" +
//            "......755.\n" +
//            "...\$.*....\n" +
//            ".664.598..").split("\n")

    part1(lines)
    part2(lines)
}

// bound by in-memory array...
data class Point(val row: Int, val col: Int) {
    fun moveBy(row: Int = 0, col: Int = 0) = copy(row = this.row + row, col = this.col + col)

    fun neighbors() = listOf(
        copy(row = row - 1),
        copy(row = row + 1),
        copy(col = col - 1),
        copy(col = col + 1),

        copy(row = row - 1, col = col - 1),
        copy(row = row + 1, col = col - 1),
        copy(row = row - 1, col = col + 1),
        copy(row = row + 1, col = col + 1),
        )
}

data class Region(val row: IntRange, val col: IntRange) {
    fun contains(point: Point) = row.contains(point.row) && col.contains(point.col)
}

fun part1(lines: List<String>) {
    val numbers = mutableMapOf<Region, Long>()
    val symbols = mutableMapOf<Point, Char>()

    lines.forEachIndexed {row, line ->
        var buildingNum = false
        var start = 0
        var numStr = ""
        line.forEachIndexed { col, c ->
            if (c.isDigit()) {
                if (!buildingNum) {
                    buildingNum = true
                    start = col
                }
                numStr += c
            } else if (buildingNum) {
                buildingNum = false

                numbers[Region((row-1)..(row+1), (start-1)..(col))] = numStr.toLong()
                numStr = ""

//                numbers[] = numStr.toLong()
            }

            if (!c.isDigit() && c != '.') {
                symbols[Point(row, col)] = c
            }
        }
        // handle end of line edge case
        if (buildingNum) {
            numbers[Region((row-1)..(row+1), (start-1)..(line.length))] = numStr.toLong()
        }
    }

    val foundRegions = mutableSetOf<Region>()

    val answer = symbols.keys.flatMap { point ->
        // n^2 ftw
        numbers.keys.mapNotNull {
            if (!foundRegions.contains(it) && it.contains(point)) {
                foundRegions.add(it)
                numbers[it]
            } else null
        }
    }
        .sum()
    println(answer)
}

fun part2(lines: List<String>) {
    val numbers = mutableMapOf<Region, Long>()
    val symbols = mutableMapOf<Point, Char>()

    lines.forEachIndexed {row, line ->
        var buildingNum = false
        var start = 0
        var numStr = ""
        line.forEachIndexed { col, c ->
            if (c.isDigit()) {
                if (!buildingNum) {
                    buildingNum = true
                    start = col
                }
                numStr += c
            } else if (buildingNum) {
                buildingNum = false

                numbers[Region((row-1)..(row+1), (start-1)..(col))] = numStr.toLong()
                numStr = ""

//                numbers[] = numStr.toLong()
            }

            if (c == '*') {
                symbols[Point(row, col)] = c
            }
        }
        // handle end of line edge case
        if (buildingNum) {
            numbers[Region((row-1)..(row+1), (start-1)..(line.length))] = numStr.toLong()
        }
    }

    val foundRegions = mutableSetOf<Region>()

    // we only want symbols that have exactly 2 adjacent numbers
    val answer = symbols.keys.map { point ->
        // n^2 ftw
        numbers.keys.mapNotNull {
            if (!foundRegions.contains(it) && it.contains(point)) {
                foundRegions.add(it)
                numbers[it]
            } else null
        }
    }.filter { it.size == 2 }.sumOf { it[0] * it[1] }

//        .sum()
    println(answer)
}