import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.util.ArrayList;
import java.util.Objects;

public class Day7b {

	private static Entry rootFile = new Entry(null, "");
	private static Entry currentFile = rootFile;
	private static Entry smallestFittingToRemove = rootFile;

	private static int neededSpace;

	public static void main(String[] args) {
		SimpleFileReader.getInstance().readFile("advent-of-code/input7.txt", (line, count, isLast) -> {
			if (count == 0) {
				return;
			}
			if (line.startsWith("$ ")) {
				String[] parts = line.split(" ");
				if (parts.length == 3) { // i.e. it is a 'cd' command
					if (parts[2].equals("..")) { // go up one
						currentFile = currentFile.parent;
					} else { // go one down
						Entry f = new Entry(currentFile, parts[2]);
						currentFile.content.add(f);
						currentFile = f;
					}
				}
			} else { // i.e. it is a dir or a file shown by the 'ls' command
				String[] parts = line.split(" ");
				if (parts[0].equals("dir")) {
					// ignore, because we will access it later anyway
				} else {
					Entry f = new Entry(currentFile, parts[1], Integer.valueOf(parts[0]));
					currentFile.content.add(f);
					currentFile.dirSize += f.fileSize;

					updateParentDirSizes(currentFile, f.fileSize);
				}
			}
		});

		neededSpace = 30000000 - (70000000 - rootFile.getSize());
		findBigDirs(rootFile);

		System.out.println(smallestFittingToRemove.getSize());
	}

	private static void updateParentDirSizes(Entry f, int fileSize) {
		if (f.parent != null) {
			f.parent.dirSize += fileSize;
			updateParentDirSizes(f.parent, fileSize);
		}
	}

	private static void findBigDirs(Entry f) {
		for (Entry e : f.content) {
			if (!e.isFile) {
				if (e.getSize() >= neededSpace) {
					if (e.getSize() < smallestFittingToRemove.getSize())
						smallestFittingToRemove = e;

					findBigDirs(e);
				}
			}
		}
	}

	/* FILE SYSTEM ENTRY */
	public static class Entry {

		public Entry(Entry parent, String name, int fileSize) {
			this.parent = parent;
			this.name = name;
			this.fileSize = fileSize;
			this.isFile = true;
		}

		public Entry(Entry parent, String name) {
			this.parent = parent;
			this.name = name;
			this.isFile = false;
			this.fileSize = -1;
		}

		private final Entry parent;
		private final String name;
		private final ArrayList<Entry> content = new ArrayList<Day7b.Entry>();;
		private final int fileSize;
		private int dirSize;
		public final boolean isFile;

		public String getPath() {
			return (parent != null ? parent.getPath() : "") + name + (isFile ? "" : "/");
		}

		public int getSize() {
			if (isFile)
				return fileSize;
			return dirSize;
		}
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
