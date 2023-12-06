import kotlin.time.measureTime

expect fun getLines(): List<String>

fun main() {
    var lines = getLines()
//    lines = """seeds: 79 14 55 13
//
//seed-to-soil map:
//50 98 2
//52 50 48
//
//soil-to-fertilizer map:
//0 15 37
//37 52 2
//39 0 15
//
//fertilizer-to-water map:
//49 53 8
//0 11 42
//42 0 7
//57 7 4
//
//water-to-light map:
//88 18 7
//18 25 70
//
//light-to-temperature map:
//45 77 23
//81 45 19
//68 64 13
//
//temperature-to-humidity map:
//0 69 1
//1 0 69
//
//humidity-to-location map:
//60 56 37
//56 93 4""".lines()

    lines = lines.filter { it.isNotBlank() }

    day1(lines)
    day2(lines)
}

data class Maps(
    val seedToSoil: MutableMap<LongRange, (Long) -> Long> = mutableMapOf(),
    val soilToFertilizer: MutableMap<LongRange, (Long) -> Long> = mutableMapOf(),
    val fertilizerToWater: MutableMap<LongRange, (Long) -> Long> = mutableMapOf(),
    val waterToLight: MutableMap<LongRange, (Long) -> Long> = mutableMapOf(),
    val lightToTemperature: MutableMap<LongRange, (Long) -> Long> = mutableMapOf(),
    val temperatureToHumidity: MutableMap<LongRange, (Long) -> Long> = mutableMapOf(),
    val humidityToLocation: MutableMap<LongRange, (Long) -> Long> = mutableMapOf(),
) {
    // NOTE if not found, dst is same as source
    fun traverse(seed: Long): Long {
        val soil = seedToSoil.transform(seed)
        val fert = soilToFertilizer.transform(soil)
        val water = fertilizerToWater.transform(fert)
        val light = waterToLight.transform(water)
        val temp = lightToTemperature.transform(light)
        val humid = temperatureToHumidity.transform(temp)
        return humidityToLocation.transform(humid)
    }

    // finds the key that the input is present in and applies the mapping function. Otherwise, returns the input
    fun Map<LongRange, (Long) -> Long>.transform(input: Long) = this.entries.firstOrNull { (range, _) -> range.contains(input) }?.let { (_, toNext) -> toNext(input) } ?: input

}

fun buildMap(lines: List<String>): Maps {
    val maps = Maps()

    var curMap = maps.seedToSoil

    for (line in lines) {
        if (line.first().isDigit()) {
            val (dst, src, len) = line.split(" ").map { it.toLong() }.take(3)
            curMap[src..<src+len] = {
                it - src + dst
            }
        } else curMap = when(line.split("-").first()) {
            "soil" -> maps.soilToFertilizer
            "fertilizer" -> maps.fertilizerToWater
            "water" -> maps.waterToLight
            "light" -> maps.lightToTemperature
            "temperature" -> maps.temperatureToHumidity
            "humidity" -> maps.humidityToLocation
            else -> throw IllegalStateException("Unexpected prefix on line: $line")
        }
    }
    return maps
}

fun day1(lines: List<String>) {
    // these are presumably always in this order

    // first line is seeds
    val seeds = lines.first().removePrefix("seeds: ").split(" ").map { it.toLong() }

    val maps = buildMap(lines.drop(2))

    println(seeds.minOf { seed -> maps.traverse(seed) })
}

// IDEA: take the extremeties of each range and feed that into the next step, accounting for if there are multiple ranges they map to

fun day2(lines: List<String>) = measureTime {

    val seedPairs = lines.first()
        .removePrefix("seeds: ")
        .split(" ")
        .map { it.toLong() }
        .windowed(2, step=2)

    val maps = buildMap(lines.drop(2))

    println(seedPairs)

    println(sequence {
        seedPairs.forEach { (start, len) ->
            for (i in 0 until len) {
                yield(start + i)
            }
        }
    }
        .minOf { seed -> maps.traverse(seed) })
}.also { println("duration: $it ")}