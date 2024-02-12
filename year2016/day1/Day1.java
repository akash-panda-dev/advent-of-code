package year2016.day1;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.EnumMap;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

enum Direction {
    R, L, U, D
}

class Location {
    Point point;
    Direction dir;

    public Location(Point point, Direction dir) {
        this.point = point;
        this.dir = dir;
    }
}

class Movement {
    Direction dir;
    int distance;

    public Movement(Direction dir, int distance) {
        this.dir = dir;
        this.distance = distance;
    }
}

class Point {
    int x;
    int y;

    Point(int x, int y) {
        this.x = x;
        this.y = y;
    }
}

public class Day1 {

    private static final Map<Direction, Map<Direction, Direction>> directionMap;

    static {
        directionMap = new EnumMap<Direction, Map<Direction, Direction>>(Direction.class);
        Map<Direction, Direction> rightMap = new EnumMap<Direction, Direction>(Direction.class);
        rightMap.put(Direction.R, Direction.D);
        rightMap.put(Direction.L, Direction.U);
        rightMap.put(Direction.U, Direction.R);
        rightMap.put(Direction.D, Direction.L);

        Map<Direction, Direction> leftMap = new EnumMap<Direction, Direction>(Direction.class);
        leftMap.put(Direction.R, Direction.U);
        leftMap.put(Direction.L, Direction.D);
        leftMap.put(Direction.U, Direction.L);
        leftMap.put(Direction.D, Direction.R);

        directionMap.put(Direction.R, rightMap);
        directionMap.put(Direction.L, leftMap);
    }

    private static Location getDestination(Location start, List<Movement> movements) {
        Location end = new Location(new Point(start.point.x, start.point.y), start.dir);
        Map<String, Integer> visited = new HashMap<String, Integer>();
        visited.put(end.point.x + ", " + end.point.y, 1);

        for (Movement mov : movements) {
            end.dir = directionMap.get(mov.dir).get(end.dir);
            int dx = 0, dy = 0;
            switch (end.dir) {
                case R:
                    dx = 1;
                    break;
                case L:
                    dx = -1;
                    break;
                case U:
                    dy = 1;
                    break;
                case D:
                    dy = -1;
                    break;
            }
            for (int i = 0; i < mov.distance; i++) {
                String newPoint = (end.point.x + dx) + ", " + (end.point.y + dy);
                end.point.x += dx;
                end.point.y += dy;
                visited.compute(newPoint, (k, v) -> v == null ? 1 : v + 1);

                if (visited.get(newPoint) == 2) {
                    return end;
                }
            }
        }

        return end;
    }

    public static void main(String[] args) {
        String file;

        try {
            file = Files.readString(Paths.get("year2016/day1/input.txt"));
        } catch (IOException e) {
            e.printStackTrace();
            throw new RuntimeException(e);
        }

        String[] movInstr = Arrays.stream(file.trim().split(","))
                .map(String::trim)
                .toArray(String[]::new);

        List<Movement> movements = new ArrayList<Movement>();

        Arrays.stream(movInstr).forEach(instr -> {
            Direction dir = instr.charAt(0) == 'R' ? Direction.R : Direction.L;
            int distance = Integer.parseInt(instr.substring(1));
            movements.add(new Movement(dir, distance));
        });

        Location start = new Location(new Point(0, 0), Direction.U);
        Location end = getDestination(start, movements);

        System.out.println("Part 1: " + (Math.abs(end.point.x - start.point.x) + Math.abs(end.point.y - start.point.y)));
    }
}
