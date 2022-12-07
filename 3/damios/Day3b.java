import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.util.Objects;

public class Day3b {

	private static int priorities = 0;
	private static int i = 0;

	public static void main(String[] args) {
		SimpleThreeLineFileReader.getInstance().readFile("advent-of-code/input3.txt", (line1, line2, line3) -> {
			for (char c1 : line1.toCharArray()) {
				for (char c2 : line2.toCharArray()) {
					if (c1 == c2) {
						for (char c3 : line3.toCharArray()) {
							if (c2 == c3) {
								priorities += c1 > 90 ? c1 - 96 : c1 - 38;
								return;
							}
						}
					}
				}
			}
		});

		System.out.println(priorities);
	}

	/* SIMPLE THREE LINE FILE READER */
	public static class SimpleThreeLineFileReader {
		private static final SimpleThreeLineFileReader instance = new SimpleThreeLineFileReader();

		private SimpleThreeLineFileReader() {
		}

		public static SimpleThreeLineFileReader getInstance() {
			return instance;
		}

		public void readFile(String path, TriConsumer<String, String, String> lineConsumer) {
			try (BufferedReader br = new BufferedReader(new FileReader(new File(path)))) {
				String line;
				int i = 0;

				while ((line = br.readLine()) != null) {
					lineConsumer.accept(line, br.readLine(), br.readLine());
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
