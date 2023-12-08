expect fun getLines(): List<String>

fun main() {
    var lines = getLines()
    // part 1 test input
//    lines = """RL
//
//AAA = (BBB, CCC)
//BBB = (DDD, EEE)
//CCC = (ZZZ, GGG)
//DDD = (DDD, DDD)
//EEE = (EEE, EEE)
//GGG = (GGG, GGG)
//ZZZ = (ZZZ, ZZZ)""".lines()

    // part 2 test input
//    lines = """LR
//
//11A = (11B, XXX)
//11B = (XXX, 11Z)
//11Z = (11B, XXX)
//22A = (22B, XXX)
//22B = (22C, 22C)
//22C = (22Z, 22Z)
//22Z = (22B, 22B)
//XXX = (XXX, XXX)""".lines()

//    part1(lines)
    part2(lines)
}

data class Node(val parent: String, val left: String, val right: String, val parentOcc: Int = 0, val leftOcc: Int = 0, val rightOcc: Int = 0)

fun part1(lines: List<String>) = printDuration {
    val directions = lines[0]

    val tree = lines.drop(2).map { line ->
        val (parent, children) = line.split(" = ")
        val (left, right) = children.drop(1).dropLast(1).split(", ")
        Node(parent, left, right)
    }.associateBy { it.parent }

    // TODO check for cycle?
    var index = 0
    var steps = 0L
    var cur = "AAA"

    while (cur != "ZZZ") {
        cur = when(directions[index]) {
            'R' -> tree[cur]!!.right
            'L' -> tree[cur]!!.left
            else -> TODO()
        }
        index = (index + 1) % directions.length
        steps++
    }

    println(steps)
}

// https://rosettacode.org/wiki/Least_common_multiple#Kotlin
fun gcd(a: Long, b: Long): Long = if (b == 0L) a else gcd(b, a % b)
fun lcm(a: Long, b: Long): Long = a / gcd(a, b) * b

fun part2(lines: List<String>) = printDuration {
    // I wish coroutines worked in this current setup...
    val directions = lines[0]

    val tree = lines.drop(2).map { line ->
        val (parent, children) = line.split(" = ")
        val (left, right) = children.drop(1).dropLast(1).split(", ")
        Node(parent, left, right)
    }.associateBy { it.parent }

    tree.keys.filter { it.last() == 'A' }.map { start ->
        var index = 0
        var steps = 0L
        var cur = start
        // hopefully no crossover
        while (cur.last() != 'Z') {
            cur = when(directions[index]) {
                'R' -> tree[cur]!!.right
                'L' -> tree[cur]!!.left
                else -> TODO()
            }
            index = (index + 1) % directions.length
            steps++
        }
        steps
    }.reduce { acc, l -> lcm(acc, l) }

}
