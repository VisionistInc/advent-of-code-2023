expect fun getLines(): List<String>

fun main() {
    var lines = getLines()
//    lines = """0 3 6 9 12 15
//1 3 6 10 15 21
//10 13 16 21 30 45""".lines()


    part1(lines)
    part2(lines)
}
fun part1(lines: List<String>) = printDuration {
    lines.sumOf { line ->
        val iterations = mutableListOf<List<Int>>()
        iterations.add(line.split(" ").map { it.toInt() })

        while (iterations.last().any { it != 0 }) {
            iterations.add(iterations.last().zipWithNext().map { it.second - it.first })
        }

        // this originally had reversed(), but addition doesn't matter, so removing it saves the cost of reversing
        iterations.sumOf { it.last() }
    }
}

fun part2(lines: List<String>) = printDuration {
    lines.sumOf { line ->
        val iterations = mutableListOf<List<Int>>()
        iterations.add(line.split(" ").map { it.toInt() })

        while (iterations.last().any { it != 0 }) {
            iterations.add(iterations.last().zipWithNext().map { it.second - it.first })
        }

        // make sure we go bottom-up since we are subtracting a particular order
        iterations.reversed().map { it.first().toLong() }.reduce { acc, i -> i - acc }
    }

}
