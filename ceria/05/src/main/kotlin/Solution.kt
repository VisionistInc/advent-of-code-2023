import java.io.File;

var seeds = listOf<Long>()
val titles = listOf<String>("seed-to-soil map:", "soil-to-fertilizer map:", "fertilizer-to-water map:", "water-to-light map:", "light-to-temperature map:", "temperature-to-humidity map:", "humidity-to-location map:")
val maps = List(titles.size) { mutableMapOf<LongRange, LongRange>() }

fun main(args : Array<String>) {

    val seedsKeyWord = "seeds: "
    val titleChar = ':'
    var parseIdx = -1

    File(args.first()).readLines().forEach { line ->
        if (line.contains(seedsKeyWord)) {
            seeds = line.substring(line.indexOf(titleChar) + 2, line.length).split(" ").map{ it.toLong() }
        } else if (line.contains(titleChar)) {
            parseIdx++
        } else if (!line.equals("")) {
            var ranges = line.split(" ").map{ it.toLong() }
            var mapBeingPopulated = maps.get(parseIdx)
            var sourceRange = LongRange(ranges.get(1), ranges.get(1) + ranges.get(2) - 1)
            var destRange = LongRange(ranges.get(0), ranges.get(0) + ranges.get(2) - 1)
            mapBeingPopulated.put(sourceRange, destRange)
        }
    }

    println("Solution 1: ${solution1()}")
    println("Solution 2: ${solution2()}")
}

private fun solution1() :Long {
    var locations = mutableListOf<Long>()
    seeds.forEach { seed ->
        var newSource = seed
        maps.forEach{lookupMap ->
            var filtered = lookupMap.filterKeys{ it.contains(newSource) }
            filtered.forEach {k, v ->
                newSource = v.endInclusive - (k.endInclusive - newSource)
            }
        }
        locations.add(newSource)
    }
   
    return locations.min()
}

// Just wait for it...  
// Intel(R) Core(TM) i7-6700HQ CPU @ 2.60GHz laptop, plugged in
// (4 cores, but realistically only one running this program) = ~30 min
private fun solution2() :Long {
    var lowestLocation = seeds.get(0)
    for (i in 0..seeds.size - 1 step 2) {
        val start = seeds.get(i)
        val end = start + seeds.get(i + 1)
        // there's probably a trick to avoid going through each seed in the thousands/millions/billions...
        for (seed in start..end) {
            var newSource = seed
            maps.forEach{lookupMap ->
                // this filtering of each map repeatedly probably could be improved - it's hella exepnsive
                var filtered = lookupMap.filterKeys{ it.contains(newSource) }
                filtered.forEach {k, v ->
                    newSource = v.endInclusive - (k.endInclusive - newSource)
                }
            }
            // switch to comparison instead of accumulating a list and calling min() - should be faster
            if (newSource < lowestLocation) {
                lowestLocation = newSource
            }
        }

    }
   
    return lowestLocation
}