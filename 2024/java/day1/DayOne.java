import java.io.BufferedReader;
import java.io.FileReader;
import java.io.File;
import java.util.Arrays;
import java.util.List;
import java.util.ArrayList;
import java.util.stream.Stream;
import java.util.stream.Collectors;

public class DayOne {
    public static void main(String[] args) {
        BufferedReader bf = null;
        try {
            bf = new BufferedReader(new FileReader("input.txt"));
            String s;
            ArrayList<Integer> col1 = new ArrayList<Integer>();
            ArrayList<Integer> col2 = new ArrayList<Integer>();
            ArrayList<Integer> d = new ArrayList<Integer>();
            int count = 0;
            while ((s = bf.readLine()) != null) {
                col1.add(Integer.parseInt(s.split("\\s+")[0]));
                col2.add(Integer.parseInt(s.split("\\s+")[1]));
            }
            List<Integer> col1_stream = col1.stream().sorted().collect(Collectors.toList());
            List<Integer> col2_stream = col2.stream().sorted().collect(Collectors.toList());
            long sum = 0;
            for (int i = 0; i < 1000; i++) {
                //d.add(col1_stream.get(i) - col2_stream.get(i));
                sum += Math.abs(col1_stream.get(i) - col2_stream.get(i));
                if (i < 10) {
                    System.out.println(
                            col1_stream.get(i).toString() + " + " + col2_stream.get(i).toString() + " = " + sum);
                }
            }
            System.out.println(sum);
        } catch (Exception e) {
            e.printStackTrace();
        } finally {
            if (bf != null) {
                try {
                    bf.close();
                } catch (Exception e) {
                    e.printStackTrace();
                }
            }
        }
    }

}
