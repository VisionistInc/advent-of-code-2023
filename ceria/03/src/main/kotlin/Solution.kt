import java.io.File;

var schematic = mutableListOf<List<Char>>()

fun main(args : Array<String>) {
    File(args.first()).readLines().forEach { line ->
        schematic.add(line.toList())
    }

    println("Solution 1: ${solution1()}")
    println("Solution 2: ${solution2()}")
}

private fun solution1() :Int {
    var partNums = mutableListOf<Int>()

    schematic.forEachIndexed{ y, line ->
        var startNumX  = -1
        var endNumX = -1
        line.forEachIndexed{ x, _ -> 
            if (schematic.get(y).get(x).isDigit()) {
                if (startNumX == -1 && endNumX == -1 ) {
                    startNumX = x
                }
                if (x == line.size - 1) {
                    endNumX = x
                }
            } else {
                if (startNumX != -1 && endNumX == -1) {
                    endNumX = x - 1
                }
            }
                                
            if (startNumX != -1 && endNumX != -1) {
                if (test(startNumX, endNumX, y, false).first) {
                    partNums.add( line.subList(startNumX, endNumX + 1).joinToString(separator = "").toInt() )
                }
                startNumX = -1
                endNumX = -1
            }
        }
    }

    return partNums.sum()
}
    
private fun solution2() :Int {
    var gearCoordsToNums = mutableMapOf<Pair<Int, Int>, MutableList<Int>>()

    schematic.forEachIndexed{ y, line ->
        var startNumX  = -1
        var endNumX = -1
        line.forEachIndexed{ x, _ -> 
            if (schematic.get(y).get(x).isDigit()) {
                if (startNumX == -1 && endNumX == -1 ) {
                    startNumX = x
                }
                if (x == line.size - 1) {
                    endNumX = x
                }
            } else {
                if (startNumX != -1 && endNumX == -1) {
                    endNumX = x - 1
                }
            }
                                
            if (startNumX != -1 && endNumX != -1) {
                val testResult = test(startNumX, endNumX, y, true)
                if (testResult.first) {
                    var nums = mutableListOf<Int>()
                    if (gearCoordsToNums.contains(testResult.second)) {
                        nums = gearCoordsToNums.get(testResult.second)!!
                    }
                    nums.add(line.subList(startNumX, endNumX + 1).joinToString(separator = "").toInt())
                    gearCoordsToNums.put(testResult.second, nums)
                }
                startNumX = -1
                endNumX = -1
            }
        }
    }

    return gearCoordsToNums.values.filter{ it.size == 2 }.map{ it.get(0) * it.get(1) }.sum()
}

fun test(x1: Int, x2: Int, y: Int, forGear: Boolean): Pair<Boolean, Pair<Int, Int>> {
    var up = false
    var down = false
    var left = false
    var right = false
    var diagUpLeft = false
    var diagUpRight = false
    var diagDownLeft = false
    var diagDownRight = false

    if (y > 0) {
        for (x in x1..x2) {
            if (forGear && isGear(x, y - 1) ) {
               return (Pair(true, Pair(x, y - 1)))
            } else {
                up = up || isSymbolOrNumber(x, y - 1)
            }
        }
    }

    if (y < schematic.size - 2) {
        for (x in x1..x2) {
            if (forGear && isGear(x, y + 1) ) {
               return (Pair(true, Pair(x, y + 1)))
            } else {
                down = down || isSymbolOrNumber(x, y + 1)
            }
        }
    }

    if (x1 > 0) {
        if (forGear && isGear(x1 - 1, y) ) {
            return (Pair(true, Pair(x1 - 1, y)))
        } else {
            left = isSymbolOrNumber(x1 - 1, y)
        }
    }

    if (x2 < schematic.get(0).size - 2) {
         if (forGear && isGear(x2 + 1, y) ) {
            return (Pair(true, Pair(x2 + 1, y)))
        } else {
            right = isSymbolOrNumber(x2 + 1, y)
        }
    }

    if (y > 0 && x1 > 0) {
         if (forGear && isGear(x1 - 1, y - 1) ) {
            return (Pair(true, Pair(x1 - 1, y - 1)))
        } else {
            diagUpLeft = isSymbolOrNumber(x1 - 1, y - 1)
        }
    }

    if (y > 0 && x2 < schematic.get(0).size - 2) {
         if (forGear && isGear(x2 + 1, y - 1) ) {
            return (Pair(true, Pair(x2 + 1, y - 1)))
        } else {
            diagUpRight = isSymbolOrNumber(x2 + 1, y - 1)
        }
    }

    if (y < schematic.size - 2 && x1 > 0) {
        if (forGear && isGear(x1 - 1, y + 1) ) {
            return (Pair(true, Pair(x1 - 1, y + 1)))
        } else {
            diagDownLeft =  isSymbolOrNumber(x1 - 1, y + 1)
        }
    }

    if (y < schematic.size - 2 && x2 < schematic.get(0).size - 2) {
        if (forGear && isGear(x2 + 1, y + 1) ) {
            return (Pair(true, Pair(x2 + 1, y + 1)))
        } else {
            diagDownRight =  isSymbolOrNumber(x2 + 1, y + 1)
        }
    }

    if (forGear) {
        return (Pair(false, Pair(-1, -1)))
    }

    return Pair(up || down || left || right || diagUpLeft || diagUpRight || diagDownLeft || diagDownRight, Pair(-1, -1))
}

fun isSymbolOrNumber(x: Int, y: Int): Boolean {
    val candidate = schematic.get(y).get(x)
    return !candidate.equals('.')
}

fun isGear(x:Int, y:Int): Boolean {
    val candidate = schematic.get(y).get(x)
    return candidate.equals('*')
}