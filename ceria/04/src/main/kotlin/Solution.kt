import java.io.File;
import kotlin.math.pow;

var games = mutableListOf<Pair<Set<Int>, Set<Int>>>()

fun main(args : Array<String>) {
    File(args.first()).readLines().forEach { line ->
        val winningNums = line.substring(line.indexOf(": ") + 2, line.indexOf(" |")).trim().split(" ").filterNot{ it.equals("") }.map{ it.trim().toInt() }.toSet()
        val yourNums = line.substring(line.indexOf("| ") + 2, line.length).trim().split(" ").filterNot{ it.equals("") }.map{ it.trim().toInt() }.toSet()
        games.add(Pair(winningNums, yourNums))
    }

    println("Solution 1: ${solution1()}")
    println("Solution 2: ${solution2()}")
}

private fun solution1() :Int {
    var pointTotal = 0
    val multiplier = 2.0
    games.forEach{ game ->
        val intersect = game.first.intersect(game.second)
        if (!intersect.isEmpty()) {
            pointTotal += multiplier.pow((intersect.size - 1).toDouble()).toInt()
        }

    }
    return pointTotal
}
    
private fun solution2() :Int {
    var cardCount = 0
    games.forEachIndexed{idx, game ->
        cardCount++   // the card we are processing, i.e. the original
        val card = game.first.intersect(game.second).size
        if (card > 0) {
           cardCount += recurse(idx, card, 0)
        }
    }
    return cardCount
}

private fun recurse(idx: Int, card: Int, total: Int): Int{
    var newTotal = total + card
    for (i in idx + 1..idx + card) {
        val game = games.get(i)
        val newCard = game.first.intersect(game.second).size
        if (newCard > 0) {
            newTotal = recurse(i, newCard, newTotal)
        }
    }
   
    return newTotal
}
