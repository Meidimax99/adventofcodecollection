import java.io.FileNotFoundException;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.stream.Stream;

public class Day1a {

	private static int mostCalories = 0;
	private static int currentCalories = 0;

	public static void main(String... args) throws FileNotFoundException, IOException {
		try (Stream<String> stream = Files.lines(Paths.get("advent-of-code/input1.txt"))) {
			stream.forEach((line) -> processLine(line));
			processLine("");
		}

		System.out.println(mostCalories);
	}

	private static void processLine(String line) {
		if (line.length() > 0) {
			currentCalories += Integer.valueOf(line);
		} else {
			if (currentCalories > mostCalories) {
				mostCalories = currentCalories;
			}
			currentCalories = 0;
		}
	}

}
