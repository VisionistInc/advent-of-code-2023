import java.io.File;

fun main(args : Array<String>) {

    var input = mutableListOf<List<Int>>()
    File(args.first()).readLines().forEach { line ->
      input.add(line.split(" ").map{ it.toInt() })
    }

    println("Solution 1: ${solution1(input)}")
    println("Solution 2: ${solution2(input)}")
}

private fun solution1(input: List<List<Int>>) :Long {
    var sum = 0L
    input.forEach{ seq ->
        var history = mutableListOf<Int>()
        history.add(seq.last())
        var historicalSeq = seq
        var newHistoryElement = seq
        while (!newHistoryElement.all{ it == 0 }) {
            newHistoryElement = findHistory(historicalSeq)
            history.add(newHistoryElement.last())
            historicalSeq = newHistoryElement
        }
        
        sum += predict(history.reversed())
    }    

    return sum
}
    
private fun solution2(input: List<List<Int>>) :Int {
   var sum = 0
    input.forEach{ seq ->
        var history = mutableListOf<Int>()
        history.add(seq.first())
        var historicalSeq = seq
        var newHistoryElement = seq
        while (!newHistoryElement.all{ it == 0 }) {
            newHistoryElement = findHistory(historicalSeq)
            history.add(newHistoryElement.first())
            historicalSeq = newHistoryElement
        }
        
        sum += predictReverse(history.reversed())
    }    

    return sum
}

// not going to try to figure out the pattern of each sequence since it varies
// I'm sure that's the trick though
private fun findHistory(seq: List<Int>): List<Int> {
    var diffs = mutableListOf<Int>()
    for (i in 1..seq.size - 1) {
        diffs.add(seq.get(i) - seq.get(i - 1))
    }
    return diffs
}

private fun predict(hist: List<Int>): Long {
    // in case we become a Long at some point
    var history = hist.map{ it.toLong() }
    
    var prev = history.get(0)
    for (i in 1..history.size - 1) {
        prev = history.get(i) + prev
    }
    return prev
}

private fun predictReverse(history: List<Int>): Int {
    var prev = history.get(0)
    for (i in 1..history.size - 1) {
        prev = history.get(i) - prev
    }
    return prev
}