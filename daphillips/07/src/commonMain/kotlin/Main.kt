import Card.Companion.toCardOrdinal
import Card.Companion.toCardOrdinalPart2

expect fun getLines(): List<String>

fun main() {
    var lines = getLines()
//    lines = """32T3K 765
//T55J5 684
//KK677 28
//KTJJT 220
//QQQJA 483""".lines()

    part1(lines)
    part2(lines)
}

enum class Card(val ch: Char) {
    // ORDINAL ORDER MATTERS!
    Two('2'), Three('3'), Four('4'), Five('5'), Six('6'), Seven('7'), Eight('8'), Nine('9'), Ten('T'), Jack('J'), Queen('Q'), King('K'), Ace('A');

    companion object {
        fun Char.toCard() = when(this) {
            '2' -> Two
            '3' -> Three
            '4' -> Four
            '5' -> Five
            '6' -> Six
            '7' -> Seven
            '8' -> Eight
            '9' -> Nine
            'T' -> Ten
            'J' -> Jack
            'Q' -> Queen
            'K' -> King
            'A' -> Ace
            else -> throw IllegalStateException("invalid char $this")
        }

        fun Char.toCardOrdinal() = toCard().ordinal

        fun Char.toCardOrdinalPart2() = toCard().let {
            if (it == Jack) {
                // jacks are really jokers, and are the worst card for comparison
                -1
            } else it.ordinal
        }
    }
}

// TODO maybe sealed?
enum class Type {
    //ORDINAL ORDER MATTERS!
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    Pair,
    High
}

data class Hand(val hand: String, val bet: Long, val type: Type)

fun part1(lines: List<String>) = printDuration {
    val res = lines.map { line ->
        val (hand, bet) = line.split(" ").take(2)

        val associations = hand.toCharArray().toSet().associateWith { ch -> hand.count { it == ch } }

        val type = when {
            associations.size == 1 -> Type.FiveOfAKind
            associations.values.contains(4) -> Type.FourOfAKind
            associations.values.containsAll(listOf(3,2)) -> Type.FullHouse
            associations.values.contains(3) -> Type.ThreeOfAKind
            associations.values.count { it == 2 } == 2 -> Type.TwoPair
            associations.values.contains(2) -> Type.Pair
            else -> Type.High
        }

        Hand(hand, bet.toLong(), type)
    }.sortedWith(compareByDescending<Hand> { it.type.ordinal }.thenComparator { a, b ->
        a.hand.zip(b.hand).first { it.first != it.second }.let {
            (it.first.toCardOrdinal() compareTo it.second.toCardOrdinal())
        }

    })
//        .onEach { println(it) }
        .mapIndexed { index, hand -> hand.bet * (index+1) }
        .sum()

    println(res)
}

fun part2(lines: List<String>) = printDuration {
    val res = lines.map { line ->
        val (hand, bet) = line.split(" ").take(2)

        val associations = hand.toCharArray().toSet().associateWith { ch -> hand.count { it == ch } }.toMutableMap()

        associations[Card.Jack.ch]?.let {
            // check to make sure we don't empty the list... ture IFF we have all jacks!
            if (associations.size > 1) {
                associations.remove(Card.Jack.ch)
            }

            val best = associations.maxBy { it.value }
            associations[best.key] = it + best.value
        }

        val type = when {
            associations.size == 1 -> Type.FiveOfAKind
            associations.values.contains(4) -> Type.FourOfAKind
            associations.values.containsAll(listOf(3,2)) -> Type.FullHouse
            associations.values.contains(3) -> Type.ThreeOfAKind
            associations.values.count { it == 2 } == 2 -> Type.TwoPair
            associations.values.contains(2) -> Type.Pair
            else -> Type.High
        }

        Hand(hand, bet.toLong(), type)
    }.sortedWith(compareByDescending<Hand> { it.type.ordinal }.thenComparator { a, b ->
        a.hand.zip(b.hand).first { it.first != it.second }.let {
            (it.first.toCardOrdinalPart2() compareTo it.second.toCardOrdinalPart2())
        }

    })
//        .onEach { println(it) }
        .mapIndexed { index, hand -> hand.bet * (index+1) }
        .sum()

    println(res)
}
