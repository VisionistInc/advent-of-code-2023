import java.io.File;

var directions = ""
var mapNodesToDirections = mutableMapOf<String, List<String>>()

fun main(args : Array<String>) {
    var idx = 0

    File(args.first()).readLines().forEach { line ->
      if (idx == 0) {
        directions = line
        directions = directions.replace('L', '0')
        directions = directions.replace('R', '1')
        idx++
      } else if (line.equals("")) {
        idx ++
      } else {
        var node = line.substring(0, line.indexOf(" "))
        var left = line.substring(line.indexOf("(") + 1, line.indexOf(","))
        var right = line.substring(line.indexOf(",") + 2, line.length - 1)
        mapNodesToDirections.put(node, listOf<String>(left, right))
      }
    }

    println("Solution 1: ${solution1()}")
    println("Solution 2: ${solution2()}")
}

private fun solution1() :Int {
    var steps = 0
    var node = "AAA"

    while (!node.equals("ZZZ")) {
        var navigated = navigate(node, true)
        node = navigated.first
        steps += navigated.second
    }

    return steps
}

 /** 
  * This is not my original implementation. Originally I passed the list of start nodes
  * to the navigate function (I had modified that funtion to "simultaniously" traverse
  * all of the paths in the list), but the execution of that code never finished. A little
  * reddit help told me this was LCM. Next year self, look for problems that
  *   - Involve traversing paths "simultaniously"
  *   - Say that the answer for the second part increases "significantly"
  *   - The "obvious" looping implementaiton takes forever, say, more than 5 minutes to execute
  *      (Definitely if it doesn't finish over night)
  * If so, try LCM
  */
private fun solution2() :Long {
    var startingNodes = mapNodesToDirections.keys.filter{ it.last() == 'A'}
    var pathSteps = mutableListOf<Long>()
    startingNodes.forEach{ n ->
        var steps = 0L
        var node = n
        while (node.last() != 'Z') {
            var navigated = navigate(node, false)
            node = navigated.first
            steps += navigated.second.toLong()
        }
        pathSteps.add(steps)
    }
    return findLCMOfList(pathSteps)
}

private fun navigate(node: String, all: Boolean): Pair<String, Int> {
    var steps = 0
    var currNode = node
    for (i in 0..directions.length - 1) {
        currNode = mapNodesToDirections.get(currNode)!!.get(directions.get(i).toString().toInt())
        steps++

        if (all) {
            if (currNode.equals("ZZZ")) {
                break
            }
        } else {
            if (currNode.last() == 'Z') {
                break
            }
        }
    }

    return Pair(currNode, steps)
}

private fun findLCM(a: Long, b: Long): Long {
    val larger = if (a > b) a else b
    val maxLcm = a * b
    var lcm = larger
    while (lcm <= maxLcm) {
        if (lcm % a == 0L && lcm % b == 0L) {
            return lcm
        }
        lcm += larger
    }
    return maxLcm
}

private fun findLCMOfList(steps: List<Long>): Long {
    var result = steps.get(0)
    for (i in 1..steps.size - 1) {
        result = findLCM(result, steps.get(i))
    }
    return result
}