import java.util.*;
import java.io.*;

public class Main2 {
	public static void main(String[] args) {
		File f = new File("input.txt");

		try (Scanner sc = new Scanner(f)) {
			ArrayList<Integer> left = new ArrayList<>();
			ArrayList<Integer> right = new ArrayList<>();
			while (sc.hasNextLine()) {
				String[] nums = sc.nextLine().replaceAll("\\s+", " ").split(" ");
				left.add(Integer.parseInt(nums[0]));
				right.add(Integer.parseInt(nums[1]));
			}
			int totalSum = 0;
			for (int value : left) {
				totalSum += value * Collections.frequency(right, value);
			}
			System.out.println(totalSum);
		}
		catch (FileNotFoundException e) {
			System.err.println("Couldn't open the file");
		}
	}
}
