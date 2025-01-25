import java.io.BufferedReader;
import java.io.FileReader;
import java.io.File;
import java.util.Arrays;
import java.util.List;
import java.util.ArrayList;
import java.util.HashMap;
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
            //int count = 0;
            while ((s = bf.readLine()) != null) {
                col1.add(Integer.parseInt(s.split("\\s+")[0]));
                col2.add(Integer.parseInt(s.split("\\s+")[1]));
            }
            List<Integer> col1_stream = col1.stream().sorted().collect(Collectors.toList());
            List<Integer> col2_stream = col2.stream().sorted().collect(Collectors.toList());
            HashMap<Integer, Integer> similarities = new HashMap<Integer, Integer>();
            long sum = 0;
            for (int i = 0; i < 1000; i++) {
                Integer key = col2.get(i);
                Integer count = similarities.get(key);
                if (count == null) {
                    similarities.put(key, 1);
                } else {
                    similarities.put(key, count + 1);
                }
            }
            for (int v : col1) {
                Integer sims = similarities.get(v);
                if (sims != null) {
                    sum += (v * sims);
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
