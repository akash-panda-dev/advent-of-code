package year2016.day3;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

public class Day3 {
    public static void main(String[] args) {
        String input;

        try {
            input = Files.readString(Paths.get("year2016/day3/input.txt"));
        } catch (IOException e) {
            System.out.println("Error reading file");
            e.printStackTrace();
            return;
        }

        // List<List<Integer>> triangles = Arrays.stream(input.split("\n"))
        // .map(String::trim)
        // .map(s -> s.split("\\s+"))
        // .map(Arrays::stream)
        // .map(t -> t.map(Integer::parseInt).collect(Collectors.toList()))
        // .peek(Collections::sort)
        // .collect(Collectors.toList());

        String[] lines = input.split("\n");
        int validTriangles = 0;

        for (int i = 0; i < lines.length; i += 3) {
            for (int j = 0; j < 3; j++) {
                int a = Integer.parseInt(lines[i].trim().split("\\s+")[j]);
                int b = Integer.parseInt(lines[i + 1].trim().split("\\s+")[j]);
                int c = Integer.parseInt(lines[i + 2].trim().split("\\s+")[j]);

                if (a + b > c && a + c > b && b + c > a) {
                    validTriangles++;
                }
            }
        }

        System.out.println(validTriangles);
    }
}
