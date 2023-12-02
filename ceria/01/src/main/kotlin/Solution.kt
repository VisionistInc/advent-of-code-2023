import java.io.File;

fun main(args : Array<String>) {
    val lines = File(args.first()).readLines()

    println("Solution 1: ${solution1(lines)}")
    println("Solution 2: ${solution2(lines)}")
}

private fun solution1(input: List<String>) :Int {
    return input.map{ line ->
        (line.get(line.indexOfFirst{ it.isDigit() }).toString() + line.get(line.indexOfLast{ it.isDigit() }).toString()).toInt()
    }.sum()
}
    
private fun solution2(input: List<String>) :Int {
    val numbers = mapOf<String, Char>( "zero" to '0', "one" to '1', "two" to '2', "three" to '3', "four" to '4', "five" to '5', "six" to '6', "seven" to '7', "eight" to '8', "nine" to '9')
    var calibrations = mutableListOf<Int>()
    input.forEach{ line ->
        var firstDigitChar: Char = '-'
        var lastDigitChar: Char = '-'
        var firstIndex = line.indexOfFirst{ it.isDigit() }
        if (firstIndex != -1) {
            firstDigitChar = line.get(firstIndex)
        } else {
            // make it impossibly large to represent it doesn't exist
            firstIndex = 500000
        }
        var lastIndex = line.indexOfLast{ it.isDigit() }
        if (lastIndex != -1) {
            lastDigitChar = line.get(lastIndex)
        }
        numbers.keys.forEach{ num ->
            val indexes = line.indexesOf(num)
            if (!indexes.isEmpty()) {
                if (indexes.first() < firstIndex) {
                    firstDigitChar = numbers.get(num)!!
                    firstIndex = indexes.first()
                }
                if (indexes.last() > lastIndex) {
                    lastDigitChar = numbers.get(num)!!
                    lastIndex = indexes.last()
                }
            }
        }
        calibrations.add((firstDigitChar.toString() + lastDigitChar.toString()).toInt())
    }
    return calibrations.sum()
}

fun String?.indexesOf(pattern: String): List<Int> =
    pattern.toRegex().findAll(this?: "").map { it.range.first }.toList()