package helper;

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
        int lengh = random.nextInt(25);
        StringBuilder web = new StringBuilder();

        for (int i = 0; i < lengh; i++)
            web.append((char) (random.nextInt(26) + 65));

        web.append("1");

        return web.toString().toLowerCase();
    }

    public static void main(String[] args) {
        for (int i = 0; i< 25; i++)
            System.out.println(generateSite());
    }
}
