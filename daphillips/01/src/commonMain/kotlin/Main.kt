expect fun getLines(inputOrPath: String): List<String>

fun main() {
    val lines = getLines(INPUT_FILE)
    part1(lines)
    part2(lines)
}

fun part1(lines: List<String>) {
    println(lines.map { line -> line.toCharArray().filter { it.isDigit() } }
        .sumOf { "${it.first()}${it.last()}".toLong() })
}

fun part2(lines: List<String>) {
    val numbers = mapOf(
        "one" to "1",
        "two" to "2",
        "three" to "3",
        "four" to "4",
        "five" to "5",
        "six" to "6",
        "seven" to "7",
        "eight" to "8",
        "nine" to "9",
    )

    val r = Regex("(?=(one|two|three|four|five|six|seven|eight|nine|[0-9]))")

    println(lines.sumOf { line ->
        // find all the matches that exist in the line
        // since we are using a lookahead, we get nested arrays and blank values
        // flatMap handles the arrays, filter the values
        val matches = r.findAll(line).flatMap { it.groupValues }.filter { it.isNotBlank() }
        val first = matches.first().let { numbers[it] ?: it }
        val last = matches.last().let { numbers[it] ?: it }

        "$first$last".toLong()
    })
}