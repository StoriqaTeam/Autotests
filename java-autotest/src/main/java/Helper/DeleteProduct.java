package Helper;

import java.util.Random;

public class DeleteProduct {

    private static final String[] DELL_BUTTON = {
            "deleteProductButton_2",
            "deleteProductButton_3",
            "deleteProductButton_4",
            "deleteProductButton_5",
            "deleteProductButton_6",
            "deleteProductButton_7",
            "deleteProductButton_8"};

    public static String getRandomDelete() {
        return DELL_BUTTON[new Random().nextInt(DELL_BUTTON.length)];
        }
}
