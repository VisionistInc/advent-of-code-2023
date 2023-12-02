import java.io.File;

fun main(args : Array<String>) {
    // game => list of tuple(b, g, r)
    var games = mutableMapOf<Int, List<Triple<Int, Int, Int>>>()

    File(args.first()).readLines().forEach { line ->
        val game = line.substring(line.indexOf(" ") + 1, line.indexOf(":")).toInt()
        val sets = mutableListOf<Triple<Int, Int, Int>>()
        val setsLine = line.substring(line.indexOf(": ") + 2, line.length).split("; ")

        setsLine.forEach{ s ->
            var blue = 0
            var green = 0
            var red = 0
            val colors = s.split(", ")
            colors.forEach{ colorSet ->
                if (colorSet.indexOf("blue") != -1) {
                    blue = colorSet.substring(0, colorSet.indexOf("blue") - 1).toInt()
                } else if (colorSet.indexOf("green") != -1) {
                    green = colorSet.substring(0, colorSet.indexOf("green") - 1).toInt()
                } else if (colorSet.indexOf("red") != -1) {
                    red = colorSet.substring(0, colorSet.indexOf("red") - 1).toInt()
                } 
            }
            sets.add(Triple<Int, Int, Int>(blue, green, red))
        }
        games.put(game, sets)
    }

    println("Solution 1: ${solution1(games)}")
    println("Solution 2: ${solution2(games)}")
}

private fun solution1(input: Map<Int, List<Triple<Int, Int, Int>>>) :Int {
    val BLUE_MAX = 14
    val GREEN_MAX = 13
    val RED_MAX = 12
    var possibleGames = mutableListOf<Int>()
    input.forEach{ game, sets ->
        var gameIsPossible = true
        sets.forEach{ s ->
            if (s.first > BLUE_MAX || s.second > GREEN_MAX || s.third > RED_MAX) {
                gameIsPossible = false
            }
        }
        if (gameIsPossible) {
            possibleGames.add(game)
        }
    }
    return possibleGames.sum()
}
    
// Would've structured the input differently if I had known that maintaining set information weren't
// going to matter, just max color values...
private fun solution2(input: Map<Int, List<Triple<Int, Int, Int>>>) :Int {
   var gamePowers = mutableListOf<Int>()
   input.forEach{ _, sets ->
        var blueMax = 0
        var greenMax = 0
        var redMax = 0
        sets.forEach{ s ->
            if (s.first > blueMax) {
                blueMax = s.first
            }
            if (s.second > greenMax) {
                greenMax = s.second
            }
            if (s.third > redMax) {
                redMax = s.third
            }
        }
        gamePowers.add(blueMax * greenMax * redMax)    
   }
   return gamePowers.sum()
}

