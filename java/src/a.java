import java.io.File;
import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.util.HashMap;
import java.util.Scanner;

public class a {
    public static void main(String[] args) throws IOException {
        var input = new FileInputStream("pi-billion.txt");
        var lookup = new HashMap<String, Integer>();
        var prev = "";
        int c;
        while ((c = input.read()) != -1) {
            if (prev.length() == 8) {
                prev = prev.substring(1) + (char) c;
            } else {
                prev += (char) c;
            }
            if (prev.startsWith("19") || prev.startsWith("20"))
                lookup.merge(prev, 1, Integer::sum);
        }
        System.out.println("asd");
    }
}