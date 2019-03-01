package Helper;

import java.util.Random;

public class Generator {

    public static String generateEmail() {
        Random random = new Random();
        int length = random.nextInt(30) + 1;
        StringBuilder email = new StringBuilder();

        for (int i = 0; i < length; i++)
            email.append((char) (random.nextInt(26) + 65));

        email.append("@test.ru");

        return email.toString().toLowerCase();
    }

    public static String generateSite() {
        Random random = new Random();
        int length = random.nextInt(25) +  3;
        StringBuilder web = new StringBuilder();

        for (int i = 0; i < length; i++)
            web.append((char) (random.nextInt(26) + 65));

        web.append("1");

        return web.toString().toLowerCase();
    }

    public static String generatePass() {
        Random random = new Random();
        int length = random.nextInt(25) +  6;
        StringBuilder pass = new StringBuilder();

        for (int i = 0; i < length; i++)
            pass.append((char) (random.nextInt(26) + 65));

        return pass.toString().toLowerCase() + "1Q";
    }

    public static String generateVendor() {
        Random random = new Random();
        int length = random.nextInt(10) + 3;
        StringBuilder ven = new StringBuilder();

        for (int i = 0; i < length; i++)
            ven.append((char) (random.nextInt(26) + 65));

        ven.append("1");

        return ven.toString().toLowerCase();
    }

    public static String generateFirstName() {
        Random random = new Random();
        int length = random.nextInt(25) +  1;
        StringBuilder first = new StringBuilder();

        for (int i = 0; i < length; i++)
            first.append((char) (random.nextInt(26) + 65));

        first.append("1");

        return first.toString().toLowerCase();
    }

    public static String generateLastName() {
        Random random = new Random();
        int length = random.nextInt(25) +  1;
        StringBuilder last = new StringBuilder();

        for (int i = 0; i < length; i++)
            last.append((char) (random.nextInt(26) + 65));

        last.append("1");

        return last.toString().toLowerCase();
    }

    public static String generateText() {
        Random random = new Random();
        int length = random.nextInt(75) +  1;
        StringBuilder text = new StringBuilder();

        for (int i = 0; i < length; i++)
            text.append((char) (random.nextInt(26) + 65));

        text.append("1");

        return text.toString().toLowerCase();
    }

    public static String generateShortDescription() {
        Random random = new Random();
        int length = random.nextInt(169) +  1;
        StringBuilder descroption = new StringBuilder();

        for (int i = 0; i < length; i++)
            descroption.append((char) (random.nextInt(26) + 65));

        descroption.append("1");

        return descroption.toString().toLowerCase();
    }

    public static String generateSeoTitle() {
        Random random = new Random();
        int length = random.nextInt(49) +  1;
        StringBuilder seo = new StringBuilder();

        for (int i = 0; i < length; i++)
            seo.append((char) (random.nextInt(26) + 65));

        seo.append("1");

        return seo.toString().toLowerCase();
    }

    public static String generatePhone() {
        Random random = new Random();
        StringBuilder phone = new StringBuilder("+7");
        if (random.nextInt(2) == 0)
            phone.append("495");
        else
            phone.append("499");
        for (int i = 0; i < 7; i++)
            phone.append(random.nextInt(10));

        return phone.toString();
    }

    public static String generateName() {
        Random random = new Random();
        int length = random.nextInt(10) +  1;
        StringBuilder name = new StringBuilder();

        for (int i = 0; i < length; i++)
            name.append((char) (random.nextInt(26) + 65));

        return name.toString().toLowerCase();
    }

    public static String generateShopName() {
        Random random = new Random();
        int length = random.nextInt(49) +  1;
        StringBuilder shop = new StringBuilder();

        for (int i = 0; i < 8; i++)
            shop.append((char) (random.nextInt(26) + 65));

        shop.append(" ");

        for (int i = 0; i < 7; i++)
            shop.append((char) (random.nextInt(26) + 65));

        shop.append(" Test");

        return shop.toString().toLowerCase();
    }

    public static String generatePrice() {
        Random random = new Random();
        return String.valueOf(random.nextInt(9999999) + 1);
    }

    public static String generateCashback() {
        Random random = new Random();
        return String.valueOf(random.nextInt(100) + 1);
    }

    public static void main(String[] args) {
        for (int i = 0; i< 25; i++)
            System.out.println(generateCashback());
    }
}
