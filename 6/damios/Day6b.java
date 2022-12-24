import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.util.Objects;

public class Day6b {

	private static int markerLength = 14;

	public static void main(String[] args) {
		SimpleFileReader.getInstance().readFile("advent-of-code/input6.txt", (line, count, isLast) -> {
			char[] chars = line.toCharArray();
			for (int i = markerLength - 1; i < chars.length; i++) {
				boolean result = true;

				for (int y = 1; y < markerLength; y++) {
					if (!result)
						break;

					for (int x = 0; x < y; x++) {
						if (!result)
							break;
						result &= chars[i - y] != chars[i - x];
					}
				}

				if (result) {
					System.out.println(i + 1);
					return;
				}
			}
		});
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
