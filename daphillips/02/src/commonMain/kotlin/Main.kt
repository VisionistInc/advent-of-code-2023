expect fun getLines(inputOrPath: String): List<String>

fun main() {
    val lines = getLines(INPUT_FILE)
//    val lines = ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n" +
//            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n" +
//            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n" +
//            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n" +
//            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").split("\n")
    part1(lines)
    part2(lines)
}

const val MAX_RED = 12L
const val MAX_GREEN = 13L
const val MAX_BLUE = 14L


fun part1(lines: List<String>) {
    println(lines.map { line ->
        line.split(": ").let {
            val id = it.first().removePrefix("Game ")
            id.toLong() to it.last().split("; ")
        }
    }
        .filter { (id, games) ->
//            print(id)
            games.forEach { game ->
                val remaining = mutableMapOf("red" to MAX_RED, "green" to MAX_GREEN, "blue" to MAX_BLUE)
                game.split(", ").forEach { cubeAndColor ->
                    cubeAndColor.split(" ").let {
                        val color = it.last()
                        remaining[color]?.let { count -> remaining[color] = count - it.first().toLong() }
                        if (remaining.values.any { it < 0L }) {
                            // boo
                            return@filter false//.also { println(" no")}
                        }
                    }
                }
            }
            true//.also { println("")}
        }
        .sumOf { (id, _) -> id })
}

fun part2(lines: List<String>) {
    println(lines.map { line ->
        line.split(": ").let {
            val id = it.first().removePrefix("Game ")
            id.toInt() to it.last().split("; ")
        }
    }
        .map { (id, games) ->
//            print(id)
            val most = mutableMapOf<String, Long>()
            games.forEach { game ->
//                val remaining = mutableMapOf("red" to MAX_RED, "green" to MAX_GREEN, "blue" to MAX_BLUE)
                game.split(", ").forEach { cubeAndColor ->
                    cubeAndColor.split(" ").let {
                        val color = it.last()
                        val count = it.first().toLong()

                        most.getOrPut(color) { count }.run {
                            if (count > this) {
                                most[color] = count
                            }
                        }

                    }
                }
            }

            most.values.reduce { acc, l -> acc * l }
//            true.also { println("")}
        }
        .sum())
}