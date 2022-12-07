import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.util.Objects;

public class Day2b {

	private static final int[][] POINTS_LUT = { { 3, 4, 8 }, { 1, 5, 9 }, { 2, 6, 7 } };
	private static int points = 0;

	public static void main(String[] args) {
		SimpleFileReader.getInstance().readFile("advent-of-code/input2.txt", (line, count, isLast) -> {
			String[] parts = line.split(" ");
			points += POINTS_LUT[parts[0].toCharArray()[0] - 65][parts[1].toCharArray()[0] - 88];
		});

		System.out.println(points);
	}

	/* SIMPLE FILE READER */
	public static class SimpleFileReader {

		private static final SimpleFileReader instance = new SimpleFileReader();

		private SimpleFileReader() {
		}

		public static SimpleFileReader getInstance() {
			return instance;
		}

		public void readFile(String path, TriConsumer<String, Integer, Boolean> lineConsumer) {
			try (BufferedReader br = new BufferedReader(new FileReader(new File(path)))) {
				String line = br.readLine();
				int i = 0;

				while (line != null) {
					lineConsumer.accept(line, i++, (line = br.readLine()) == null);
				}
			} catch (Exception e) {
				e.printStackTrace();
			}
		}

		@FunctionalInterface
		public interface TriConsumer<T, U, V> {

			void accept(T t, U u, V v);

			default TriConsumer<T, U, V> andThen(TriConsumer<? super T, ? super U, ? super V> after) {
				Objects.requireNonNull(after);

				return (l, m, r) -> {
					accept(l, m, r);
					after.accept(l, m, r);
				};
			}
		}

	}

}
