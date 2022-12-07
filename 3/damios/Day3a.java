import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.util.Objects;

public class Day3a {

	private static int priorities = 0;

	public static void main(String[] args) {
		SimpleFileReader.getInstance().readFile("advent-of-code/input3.txt", (line, count, isLast) -> {
			String firstCompartement = line.substring(0, line.length() / 2);
			String secondCompartement = line.substring(line.length() / 2);

			for (char c1 : firstCompartement.toCharArray()) {
				for (char c2 : secondCompartement.toCharArray()) {
					if (c1 == c2) {
						priorities += c1 > 90 ? c1 - 96 : c1 - 38;
						return;
					}
				}
			}
		});

		System.out.println(priorities);
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
