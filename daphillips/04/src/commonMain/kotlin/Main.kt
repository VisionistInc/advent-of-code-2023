import kotlin.math.pow

expect fun getLines(inputOrPath: String): List<String>

fun main() {
    var lines = getLines(INPUT_FILE)
//    lines = ("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n" +
//            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n" +
//            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n" +
//            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n" +
//            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n" +
//            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").split("\n")
    day1(lines)
    day2(lines)
}

fun String.toNumbers() = split(" ").filter { it.isNotBlank() }.map { it.toLong() }.toSet()

fun day1(lines: List<String>) {
    val prefix = Regex("Card +\\d+: +")
    val res = lines.sumOf { line ->
        val (mine, winning) = line.replace(prefix, "").split("|").zipWithNext()
            .map { scratch ->
                scratch.first.toNumbers() to scratch.second.toNumbers() }.first()
        2.0.pow((mine intersect winning).size - 1.0).toLong()
    }

    println(res)
}

fun day2(lines: List<String>) {
    // hopefully this doesn't blow up
    val cards = LongArray(lines.size) { 1 }


    lines.forEachIndexed { i, line ->
        val (num, line) = line.replace("Card +".toRegex(), "").split(":").zipWithNext { a, b -> a.toLong() to b.replace(": +".toRegex(), "")}.first()

        val (mine, winning) = line.split("|").zipWithNext()
            .map { scratch -> scratch.first.toNumbers() to scratch.second.toNumbers() }.first()

        // future cards grow according to how many cards we have at the current index
        (1..(mine intersect winning).size).forEach { cards[i + it] = cards[i + it] + cards[i] }
    }

//    println(cards.contentToString())
    println(cards.sum())


}