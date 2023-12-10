import java.io.File

var possibleCards = mutableListOf<Char>('A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2')
var handsToBids = mutableMapOf<String, Int>()

fun main(args : Array<String>) {
    File(args.first()).readLines().forEach { line ->
        var input = line.split(" ")
        handsToBids.put(input.get(0), input.get(1).toInt())
    }

    println("Solution 1: ${solution1()}")
    println("Solution 2: ${solution2()}")
      
}

private fun solution1() :Int {
    val ranked = rankHands(handsToBids.keys, false).reversed()
    var totalWinnings = 0
    ranked.forEachIndexed{ i, hand ->
        totalWinnings += (handsToBids.get(hand)!! * (i + 1))
    }
    return totalWinnings
}

private fun solution2() :Int {
    possibleCards.remove('J')
    possibleCards.add('J')   

    val ranked = rankHands(handsToBids.keys, true).reversed()
    var totalWinnings = 0
    ranked.forEachIndexed{ i, hand ->
       totalWinnings += (handsToBids.get(hand)!! * (i + 1))
    }
    return totalWinnings
}

private fun rankHands(cards: Set<String>, withJokers: Boolean): List<String> {
    var fiveKind = mutableListOf<String>()
    var fourKind = mutableListOf<String>()
    var fullHouse = mutableListOf<String>()
    var threeKind = mutableListOf<String>()
    var twoPair = mutableListOf<String>()
    var onePair = mutableListOf<String>()
    var highCard = mutableListOf<String>()

    var ranked = mutableListOf<String>()
    cards.forEach{h ->
        var hand = h
        if (withJokers && hand.contains("J")) {
            hand = makeBestHand(hand)
        }

        var countMap = hand.groupingBy { it }.eachCount()
        var type = countMap.values.max()
        if (type == 5) {
            fiveKind.add(h)
        } else if (type == 4) {
            fourKind.add(h)
        } else if (type == 3) {
            if (countMap.size == 2) {
                fullHouse.add(h)
            } else {
                threeKind.add(h)
            }
        } else if (type == 2) {
            if (countMap.size == 3) {
                twoPair.add(h)
            } else {
                onePair.add(h)
            }
        } else {
                highCard.add(h)
        }
    }

    ranked.addAll(fiveKind.sortedWith(handComparator))
    ranked.addAll(fourKind.sortedWith(handComparator))
    ranked.addAll(fullHouse.sortedWith(handComparator))
    ranked.addAll(threeKind.sortedWith(handComparator))
    ranked.addAll(twoPair.sortedWith(handComparator))
    ranked.addAll(onePair.sortedWith(handComparator))
    ranked.addAll(highCard.sortedWith(handComparator))

    return ranked
}

private fun makeBestHand(hand: String): String {
    var bestHand = ""
    var noJokers = hand.replace("J", "")

    if (noJokers.length > 0) {
        var countMap: Map<Char, Int> = noJokers.groupingBy { it }.eachCount()
        var type = countMap.values.max()
        var bestCard = findBestCard(noJokers)

       if (type == 2) {
            if (countMap.size == 3 ) {
                // this would be one pair, with a joker turns into three of a kind 
                countMap.forEach{ k, v ->
                    if (v == 2) {
                        bestCard = k
                    }
                }
            } else if (countMap.size == 2) {
                // this would be one pair with 2 jokers that would become four of a kind 
                if (noJokers.length == 3) {
                    countMap.forEach{ k, v ->
                        if (v == 2) {
                            bestCard = k
                        }
                    }
                }
            }
        } else if (type == 3) {
            if (countMap.size == 2) {
                // this would be a three of a kind with a joker, becomes four of a kind
                countMap.forEach{ k, v ->
                    if (v == 3) {
                        bestCard = k       
                    }
                }
            }
        } 
        
        bestHand = hand.replace('J', bestCard)
    } else {
        // a hand of 5 J's, so make it the strongest five of a kind
        bestHand = possibleCards.get(0).toString().repeat(5)
    }
   
    return bestHand
}

private fun findBestCard(hand: String): Char {
    var bestIdx = 15  // any number more than the list of cards
    hand.forEach{ c ->
        var idx = possibleCards.indexOf(c)
        if (idx < bestIdx) {
            bestIdx = idx
        }
    }

    return possibleCards.get(bestIdx)
}

val handComparator = object : Comparator<String> {

    override fun compare(string1: String, string2: String): Int {
         for (i in 0..string1.length - 1) {
            if (possibleCards.indexOf(string1.get(i)) < possibleCards.indexOf(string2.get(i))) {
                return -1
            } else if (possibleCards.indexOf(string1.get(i)) > possibleCards.indexOf(string2.get(i))) {
                return 1
            }
        }
        return 0
    }

}