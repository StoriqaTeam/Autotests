package Helper;

import org.openqa.selenium.WebDriver;

import java.util.*;


public class Category {

    WebDriver driver;

//    private WebElement choose_category()  {
//        return driver.findElement(By.xpath("//*[@data-test='categorySelector']"));
//    }
//
//    // Офис 1 уровень
//    private WebElement office1()  {
//        return driver.findElement(By.xpath("//*[@data-test='categoryItem_46']"));
//    }
//    //Офис 2 уровень
//    private WebElement office2()  {
//        return driver.findElement(By.xpath("//*[@data-test='categoryItem_47']"));
//    }
//    //Офис 3 уровень
//    private WebElement postcards()  {
//        return driver.findElement(By.xpath("//*[@data-test='categoryItem_48']"));
//    }
//
//    //Аксессуары 1 уровень
//    private WebElement accessories()  {
//        return driver.findElement(By.xpath("//*[@data-test='categoryItem_1']"));
//    }
//    //Аксессуары 2 уровень
//    private WebElement accessories2()  {
//        return driver.findElement(By.xpath("//*[@data-test='categoryItem_3']"));
//    }

    private static List<String> homeDecor = Arrays.asList(
            "//*[@data-test='categoryItem_41']",
            "//*[@data-test='categoryItem_42']",
            "//*[@data-test='categoryItem_49']",
            "//*[@data-test='categoryItem_72']",
            "//*[@data-test='categoryItem_73']",
            "//*[@data-test='categoryItem_74']",
            "//*[@data-test='categoryItem_75']",
            "//*[@data-test='categoryItem_76']",
            "//*[@data-test='categoryItem_77']"
    );

    private static List<String> lighting = Arrays.asList(
            "//*[@data-test='categoryItem_71']",
            "//*[@data-test='categoryItem_45']",
            "//*[@data-test='categoryItem_44']",
            "//*[@data-test='categoryItem_70']"
    );

    private static List<String> bagsWallets = Arrays.asList(
            "//*[@data-test='categoryItem_30']",
            "//*[@data-test='categoryItem_31']",
            "//*[@data-test='categoryItem_65']",
            "//*[@data-test='categoryItem_66']",
            "//*[@data-test='categoryItem_67']",
            "//*[@data-test='categoryItem_68']",
            "//*[@data-test='categoryItem_69']"
    );

    private static List<String> womens = Arrays.asList(
            "//*[@data-test='categoryItem_34']",
            "//*[@data-test='categoryItem_35']",
            "//*[@data-test='categoryItem_52']",
            "//*[@data-test='categoryItem_80']"
    );

    private static List<String> mans = Arrays.asList(
            "//*[@data-test='categoryItem_51']",
            "//*[@data-test='categoryItem_79']"
    );

    private static List<String> accessories2 = Arrays.asList(
            "//*[@data-test='categoryItem_32']",
            "//*[@data-test='categoryItem_9']",
            "//*[@data-test='categoryItem_64']",
            "//*[@data-test='categoryItem_59']",
            "//*[@data-test='categoryItem_28']",
            "//*[@data-test='categoryItem_60']",
            "//*[@data-test='categoryItem_61']",
            "//*[@data-test='categoryItem_62']",
            "//*[@data-test='categoryItem_63']"
    );

    private static List<String> calendars = Arrays.asList(
            "//*[@data-test='categoryItem_48']"
    );


    private static HashMap<String, List> homeAndLiving = new HashMap();
    private static HashMap<String, List> accessories = new HashMap();
    private static HashMap<String, List> office = new HashMap();

    public static HashMap<String, HashMap<String, List>> categories = new HashMap();

    static {
        homeAndLiving.put("//*[@data-test='categoryItem_40']", homeDecor);
        homeAndLiving.put("//*[@data-test='categoryItem_43']", lighting);

        accessories.put("//*[@data-test='categoryItem_3']", accessories2);
        accessories.put("//*[@data-test='categoryItem_50']", mans);
        accessories.put("//*[@data-test='categoryItem_33']", womens);
        accessories.put("//*[@data-test='categoryItem_29']", bagsWallets);

        office.put("//*[@data-test='categoryItem_47']", calendars);

        categories.put("//*[@data-test='categoryItem_39']", homeAndLiving);
        categories.put("//*[@data-test='categoryItem_1']", accessories);
        categories.put("//*[@data-test='categoryItem_46']", office);
    }

    //    private static ArrayList<String, List, HashMap> offise = new ArrayList();
    public static void main(String[] args) {

        Random random = new Random();

        List<String> keys = new ArrayList<String>(Category.categories.keySet());
        String category1 = keys.get(random.nextInt(keys.size()));
        HashMap<String, List> categoryList2 = Category.categories.get(category1);
        keys = new ArrayList<String>(categoryList2.keySet());
        String category2 = keys.get(random.nextInt(keys.size()));
        List categoryList3 = categoryList2.get(category2);
        String category3 = (String) categoryList3.get(random.nextInt(categoryList3.size()));

        System.out.println(category1);
        System.out.println(category2);
        System.out.println(category3);
    }

}
