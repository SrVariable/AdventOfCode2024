import java.util.*;
import java.io.*;

public class Main {
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
			left.sort(null);
			right.sort(null);
			int totalSum = 0;
			for (int i = 0; i < left.size(); ++i) {
				totalSum += Math.abs(left.get(i) - right.get(i));
			}
			System.out.println(totalSum);
		}
		catch (FileNotFoundException e) {
			System.err.println("Couldn't open the file");
		}
	}
}
