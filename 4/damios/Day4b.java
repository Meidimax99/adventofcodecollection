import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.util.Objects;

public class Day4b {

	private static int overlapCount = 0;

	public static void main(String[] args) {
		SimpleFileReader.getInstance().readFile("advent-of-code/input4.txt", (line, count, isLast) -> {
			String[] parts = line.split("[,-]");

			IntPair a = new IntPair(Integer.valueOf(parts[0]), Integer.valueOf(parts[1]));
			IntPair b = new IntPair(Integer.valueOf(parts[2]), Integer.valueOf(parts[3]));

			if (b.getDifference() > a.getDifference()) {
				IntPair tmp = a;
				a = b;
				b = tmp;
			}

			if ((b.x >= a.x && b.x <= a.y) || (b.y >= a.x && b.y <= a.y))
				overlapCount++;
		});

		System.out.println(overlapCount);
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

	/* INT PAIR */
	public static class IntPair {

		public final int x;
		public final int y;

		public IntPair(int x, int y) {
			this.x = x;
			this.y = y;
		}

		public int getDifference() {
			return y - x;
		}

	}

}
