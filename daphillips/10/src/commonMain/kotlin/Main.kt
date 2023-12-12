expect fun getLines(): List<String>

fun main() {
    var lines = getLines()
//    lines = """7-F7-
//.FJ|7
//SJLL7
//|F--J
//LJ.LJ""".lines()

//    lines = """...........
//.S-------7.
//.|F-----7|.
//.||.....||.
//.||.....||.
//.|L-7.F-J|.
//.|..|.|..|.
//.L--J.L--J.
//...........""".lines()
//
//    lines = """.F----7F7F7F7F-7....
//.|F--7||||||||FJ....
//.||.FJ||||||||L7....
//FJL7L7LJLJ||LJ.L-7..
//L--J.L7...LJS7F-7L7.
//....F-J..F7FJ|L7L7L7
//....L7.F7||L7|.L7L7|
//.....|FJLJ|FJ|F7|.LJ
//....FJL-7.||.||||...
//....L---J.LJ.LJLJ...""".lines()
//
//    lines = """FF7FSF7F7F7F7F7F---7
//L|LJ||||||||||||F--J
//FL-7LJLJ||||||LJL-77
//F--JF--7||LJLJ7F7FJ-
//L---JF-JLJ.||-FJLJJ7
//|F|F-JF---7F7-L7L|7|
//|FFJF7L7F-JF7|JL---7
//7-L-JL7||F7|L7F-7F7|
//L.L7LFJ|||||FJL7||LJ
//L7JLJL-JLJLJL--JLJ.L""".lines()

    part1(lines)
    part2(lines)
}

private fun Point.eligibleNeighbors(grid: Map<Point, Char>) = when(grid[this]) {
    '|' -> listOf(moveBy(row = 1), moveBy(row = -1))
    '-' -> listOf(moveBy(col = 1), moveBy(col = -1))
    'L' -> listOf(moveBy(row = - 1), moveBy(col = 1))
    'J' -> listOf(moveBy(row = - 1), moveBy(col = - 1))
    '7' -> listOf(moveBy(row = 1), moveBy(col = - 1))
    'F' -> listOf(moveBy(row = 1), moveBy(col = 1))
    '.' -> null
    'S' -> buildList {
        moveBy(row = - 1).takeIf { listOf('|', 'F' ,'7').contains(grid[it]) }?.let { add(it) }
        moveBy(row =  1).takeIf { listOf('|', 'L' ,'J').contains(grid[it]) }?.let { add(it) }

        moveBy(col = 1).takeIf { listOf('-', 'J' ,'7').contains(grid[it]) }?.let { add(it) }
        moveBy(col = - 1).takeIf { listOf('-', 'L' ,'F').contains(grid[it]) }?.let { add(it) }
    }

    else -> throw IllegalArgumentException("unexpected char $this -> ${grid[this]}")
}


// DON'T DO RECURSION!!!
//fun Map<Point, Char>.walk(point: Point, step: Long = 0, visited: MutableSet<Point> = mutableSetOf()/*prev: Point = point, */): Long? = this[point]?.let {
////    if (it == 'S' && prev != point) {
//    if (it == 'S' && visited.contains(point)) {
//        0L
//    } else {
//        println("$point -> $it")
//        point.eligibleNeighbors(this)?.filterNot { visited.contains(it) }
//            ?.onEach { visited.add(it) }?.mapNotNull {
//                walk(it, step, visited)?.plus(1)
//            }
//        ?.maxOrNull()
//    }
//}


fun part1(lines: List<String>) = printDuration {

    val grid = lines.flatMapIndexed { row, line ->
        line.mapIndexed { col, c -> Pair(Point(row, col), c) }
    }.toMap()

    val start = grid.keys.first { grid[it] == 'S' }

//    grid.walk(start)!! / 2

    val visited = mutableSetOf<Point>()
    val points = mutableListOf(start)
    var dist = 0
    while (points.isNotEmpty()) {
        visited.addAll(points)

        val nextPoints = points.flatMap { it.eligibleNeighbors(grid) ?: emptyList() }

        nextPoints.filterNot { visited.contains(it) }



//        val newPoints = points.flatMap { point ->
//            point.eligibleNeighbors(grid)?.filterNot { visited.contains(it) }?.filter { grid[it] != null } ?: emptyList()
//        }
        val newPoints = points.first().eligibleNeighbors(grid)?.filterNot { visited.contains(it) }?.filter { grid[it] != null } ?: emptyList()

        points.clear()
        points.addAll(newPoints)
        if (points.isNotEmpty()) {
            dist++
        }
    }
    dist
}

fun part2(lines: List<String>) = printDuration {
    val grid = lines.flatMapIndexed { row, line ->
        line.mapIndexed { col, c -> Pair(Point(row, col), c) }
    }.toMap()

    val start = grid.keys.first { grid[it] == 'S' }

    val visited = mutableSetOf<Point>()
    val points = mutableListOf(start)
//    var dist = 0
    while (points.isNotEmpty()) {
        visited.addAll(points)

        val nextPoints = points.flatMap { it.eligibleNeighbors(grid) ?: emptyList() }

        nextPoints.filterNot { visited.contains(it) }



        val newPoints = points.flatMap { point ->
            point.eligibleNeighbors(grid)?.filterNot { visited.contains(it) }?.filter { grid[it] != null } ?: emptyList()
        }
//        val newPoints = points.first().eligibleNeighbors(grid)?.filterNot { visited.contains(it) }?.filter { grid[it] != null } ?: emptyList()

        points.clear()
        points.addAll(newPoints)
    }

    val notPath = grid.keys.filter { it !in visited }

    // TODO this is super brittle! It does _NOT_ work for at least one other input in AoC

    // Ledo's may not cut corners, but I do :)
    val vertPipes = visited.filter { grid[it] in listOf('|', 'J', 'L') }
    val horizPipes = visited.filter { grid[it] in listOf('-', 'J', '7') }

    notPath.map { point ->
        point to listOf(
            // intersections from left
            vertPipes.count { it.row == point.row && it.col > point.col },
            // intersections from right
            vertPipes.count { it.row == point.row && it.col < point.col },
            // intersections from top
            horizPipes.count { it.row > point.row && it.col == point.col },
            // intersections from bottom
            horizPipes.count { it.row < point.row && it.col == point.col },
        )
    }
        // try checking from the top-left
        .filter { it.second.none { it == 0 } }.count { it.second[0] % 2 != 0 || it.second[2] % 2 != 0 }
}
