import java.util.*;
import java.io.*;

public class Main {
	public static void main(String[] args) {
		String filename = "input.txt";
		if (args.length > 0) {
			filename = args[0];
		}
		File f = new File(filename);
		try (Scanner sc = new Scanner(f)) {
			ArrayList<String> lines = new ArrayList<>();
			while (sc.hasNextLine()) {
				lines.add(sc.nextLine());
			}
			partOne(lines);
			partTwo(lines);
		}
		catch (FileNotFoundException e) {
			System.err.println("Couldn't open the file");
		}
	}
	
	public static void partOne(ArrayList<String> lines) {
		char[][] map = transformToMap(lines);
		int totalSum = 0;
		for (int i = 0; i < map.length; ++i) {
			for (int j = 0; j < map[i].length; ++j) {
				if (map[i][j] == '0') {
					totalSum += countTrailheads(map, i, j, map.length, map[i].length, '1', false);
					map = transformToMap(lines);
				}
			}
		}
		System.out.println(totalSum);
	}
	
	public static void partTwo(ArrayList<String> lines) {
		char[][] map = transformToMap(lines);
		int totalSum = 0;
		for (int i = 0; i < map.length; ++i) {
			for (int j = 0; j < map[i].length; ++j) {
				if (map[i][j] == '0') {
					totalSum += countTrailheads(map, i, j, map.length, map[i].length, '1', true);
					map = transformToMap(lines);
				}
			}
		}
		System.out.println(totalSum);
	}

	public static char[][] transformToMap(ArrayList<String> lines) {
		char[][] map = new char[lines.size()][];
		for (int i = 0; i < lines.size(); ++i) {
			map[i] = lines.get(i).toCharArray();
		}
		return (map);
	}
	
	public static int countTrailheads(char[][] map, int i, int j, int rows, int cols, char next, boolean flag) {
		if (map[i][j] == '9') {
			if (!flag) {
				map[i][j] = 'x';
			}
			return (1);
		}
		int x = 0;
		if (i - 1 >= 0 && next == map[i - 1][j]) {
			x += countTrailheads(map, i - 1, j, rows, cols, (char)(map[i - 1][j] + 1), flag);
		}
		if (i + 1 < rows && next == map[i + 1][j]) {
			x += countTrailheads(map, i + 1, j, rows, cols, (char)(map[i + 1][j] + 1), flag);
		}
		if (j - 1 >= 0 && next == map[i][j - 1]) {
			x += countTrailheads(map, i, j - 1, rows, cols, (char)(map[i][j - 1] + 1), flag);
		}
		if (j + 1 < cols && next == map[i][j + 1]) {
			x += countTrailheads(map, i, j + 1, rows, cols, (char)(map[i][j + 1] + 1), flag);
		}
		return (x);
	}
}
