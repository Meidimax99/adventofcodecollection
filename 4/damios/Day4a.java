import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.util.Objects;

public class Day4a {

	private static int fullyContainedCount = 0;

	public static void main(String[] args) {
		SimpleFileReader.getInstance().readFile("advent-of-code/input4.txt", (line, count, isLast) -> {
			String[] parts = line.split("[,-]");

			IntPair left = new IntPair(Integer.valueOf(parts[0]), Integer.valueOf(parts[1]));
			IntPair right = new IntPair(Integer.valueOf(parts[2]), Integer.valueOf(parts[3]));

			if (left.getDifference() > right.getDifference()) {
				if (right.x >= left.x && right.y <= left.y)
					fullyContainedCount++;
			} else if (left.getDifference() < right.getDifference()) {
				if (left.x >= right.x && left.y <= right.y)
					fullyContainedCount++;
			} else {
				if (left.x == right.x)
					fullyContainedCount++;
			}

		});

		System.out.println(fullyContainedCount);
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
