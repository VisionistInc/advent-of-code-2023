import java.io.File;

fun main(args : Array<String>) {
    var times = listOf<Int>()
    var distances = listOf<Int>()

    File(args.first()).readLines().forEach { line ->
        if (line.contains("Time:")) {
            times = line.substring(line.indexOf(":") + 1, line.length).trim().split(" ").filterNot{ it.equals("") }.map{ it.toInt() }
        } else {
            distances = line.substring(line.indexOf(": ") + 1, line.length).trim().split(" ").filterNot{ it.equals("") }.map{ it.toInt() }
        }
    }

    println("Solution 1: ${solution1(times, distances)}")
    println("Solution 2: ${solution2(times, distances)}")
}

// nothing fancy, we'll see if that bites me
private fun solution1(times: List<Int>, distances: List<Int>) :Int {
    var product = 1
    distances.forEachIndexed{ i, distance -> 
        var possibilities = mutableListOf<Int>()
        val time = times.get(i)
        for (speed in 1..time) { // it'll never be 0, so start at 1
            val dist = (time - speed) * speed
            if (dist > distance) {
                possibilities.add(dist)
            }
        }
        product = product * possibilities.size
    }

    return product
}
    
private fun solution2(times: List<Int>, distances: List<Int>) :Int {
    var newTime = times.joinToString(separator = "").toLong()
    var newDistance = distances.joinToString(separator = "").toLong()
    var possibilities = mutableListOf<Long>()
    for (speed in 1..newTime) { // it'll never be 0, so start at 1
        val dist = (newTime - speed) * speed
        if (dist > newDistance) {
            possibilities.add(dist)
        }
    }

    return possibilities.size
}

