package Helper;

import java.util.Random;

public class SelectionVariant {

    private static final String[] VARIANT = {
            "productRow_2",
            "productRow_3",
            "productRow_4",
            "productRow_5",
            "productRow_6",
            "productRow_7",
            "productRow_8"};

    public static String getRandonVariant() { return VARIANT[new Random().nextInt(VARIANT.length)]; }
}
