expect fun getLines(): List<String>

fun main() {
    var lines = getLines()
//    lines = """...#......
//.......#..
//#.........
//..........
//......#...
//.#........
//.........#
//..........
//.......#..
//#...#.....""".lines()


    part1(lines)
    part2(lines)
}

fun part1(lines: List<String>) = printDuration {
    val universe = lines.flatMapIndexed { row, line ->
        line.mapIndexed { col, c -> Point(row, col) to c }.filter { (_, c) -> c == '#' }
    }.map { it.first }.toSet()

    val rows = universe.map { it.row }
    val emptyRows = (0L..<lines.size).asSequence().filterNot { it in rows }.toList()

    val cols = universe.map{ it.col }
    val emptyCols = (0L..<lines.first().length).asSequence().filterNot { it in cols }.toList()


    val expandedUniverse = universe.map { (row, col) ->
        Point(emptyRows.count { it < row } + row, emptyCols.count { it < col } + col)
    }

    expandedUniverse.flatMapIndexed { index, galaxy ->
        expandedUniverse.drop(index + 1).associateWith { galaxy }.map { (other, galaxy) -> galaxy.dist(other) }
    }.sum()

}

fun part2(lines: List<String>) = printDuration {
    val universe = lines.flatMapIndexed { row, line ->
        line.mapIndexed { col, c -> Point(row, col) to c }.filter { (_, c) -> c == '#' }
    }.map { it.first }.toSet()

    val rows = universe.map { it.row }
    val emptyRows = (0L..<lines.size).asSequence().filterNot { it in rows }.toList()

    val cols = universe.map{ it.col }
    val emptyCols = (0L..<lines.first().length).asSequence().filterNot { it in cols }.toList()


    val expandedUniverse = universe.map { galaxy ->
        galaxy.moveBy((1_000_000L-1) * emptyRows.count { it < galaxy.row }, (1_000_000L-1) * emptyCols.count { it < galaxy.col })
    }

    expandedUniverse.flatMapIndexed { index, galaxy ->
        expandedUniverse.drop(index + 1).associateWith { galaxy }.map { (other, galaxy) -> galaxy.dist(other) }
    }.sum()
}
