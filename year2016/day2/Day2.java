package year2016.day2;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

enum Direction {
    R, L, U, D
}

class Keypad {
    private final String[][] keypad;
    private int row;
    private int col;

    public Keypad(int startRow, int startCol) {
        this.keypad = new String[][] {
                { null, null, "1", null, null },
                { null, "2", "3", "4", null },
                { "5", "6", "7", "8", "9" },
                { null, "A", "B", "C", null },
                { null, null, "D", null, null }
        };
        this.row = startRow;
        this.col = startCol;
    }

    public String getNumber() {
        return keypad[row][col];
    }

    public void move(Direction dir) {
        int newRow = row, newCol = col;
        switch (dir) {
            case R:
                newCol++;
                break;
            case L:
                newCol--;
                break;
            case U:
                newRow--;
                break;
            case D:
                newRow++;
                break;
        }
        if (newRow >= 0 && newRow < keypad.length &&
                newCol >= 0 && newCol < keypad[newRow].length &&
                keypad[newRow][newCol] != null) {
            row = newRow;
            col = newCol;
        }
    }
}

public class Day2 {

    public static void main(String[] args) {
        String file;

        try {
            file = Files.readString(Paths.get("year2016/day2/input.txt"));
        } catch (IOException e) {
            e.printStackTrace();
            return;
        }

        Keypad keypad = new Keypad(1, 1);

        String[] lines = file.split("\n");
        String bathroomCode = "";

        for (String line : lines) {
            for (char c : line.toCharArray()) {
                keypad.move(Direction.valueOf(Character.toString(c)));
            }
            bathroomCode += keypad.getNumber();
        }

        System.out.println(bathroomCode);
    }
}
