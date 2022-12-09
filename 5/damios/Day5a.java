import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.ListIterator;
import java.util.Map.Entry;
import java.util.Objects;
import java.util.Stack;

public class Day5a {

	private static int stackCount;
	private static ArrayList<String> firstFewLines = new ArrayList<>();
	private static HashMap<Integer, Stack> stackMap = new HashMap<>();

	public static void main(String[] args) {
		SimpleFileReader.getInstance().readFile("advent-of-code/input5.txt", (line, count, isLast) -> {
			if (line.startsWith("[")) { // ignore first few lines for now
				firstFewLines.add(line);
			} else if (line.startsWith(" ")) { // start parsing now that we know the number of stacks
				stackCount = (line.length() - 3) / 4 + 1;

				// Create the stacks
				for (int i = 1; i <= stackCount; i++) {
					stackMap.put(i, new Stack<String>());
				}

				// Fill the stacks
				ListIterator<String> it = firstFewLines.listIterator(firstFewLines.size());

				while (it.hasPrevious()) {
					char[] chars = it.previous().toCharArray();
					int currentIndex = 0;

					for (int i = 1; i < (stackCount * 4); i += 4) {
						currentIndex++;

						if (chars[i] != ' ') {
							stackMap.get(currentIndex).push(chars[i]);
						}
					}
				}
			} else if (line.startsWith("m")) { // execute the commands
				String[] parts = line.split(" ");

				for (int i = 0; i < Integer.valueOf(parts[1]); i++) {
					stackMap.get(Integer.valueOf(parts[5])).push(stackMap.get(Integer.valueOf(parts[3])).pop());
				}
			}

		});

		for (Entry<Integer, Stack> e : stackMap.entrySet())
			System.out.print(e.getValue().peek());
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
