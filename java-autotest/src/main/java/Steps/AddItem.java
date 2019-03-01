package Steps;

import Helper.*;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Random;

@RunWith(SeleniumRunner.class)

public class AddItem {
    private static final int VARIANTS_AMOUNT = 3;
    WebDriver driver;
    Element element;
    Screenshot screenshot;

    public AddItem(WebDriver driver) {
        this.driver = driver;
        screenshot = new Screenshot(driver);
    }

    // Создания товара с характеристиками

    public WebElement goods() {
        return driver.findElement(By.xpath("//*[@data-test='Goods']"));
    }
    public WebElement addProduct() {
        return driver.findElement(By.xpath("//*[@data-test='addProductButton']"));
    }
    public WebElement productName() {
        return driver.findElement(By.xpath("//*[@data-test='name']"));
    }
    public WebElement shotrDescription() {
        return driver.findElement(By.xpath("//*[@data-test='shortDescription']"));
    }
    public WebElement vendorCode() {
        return driver.findElement(By.xpath("//*[@data-test='vendorCode']"));
    }
    public WebElement categorySelector() {
        return driver.findElement(By.xpath("//*[@data-test='categorySelector']"));
    }
    public WebElement category1() {
        return driver.findElement(By.xpath("//*[@data-test='categoryItem_1']"));
    }
    public WebElement category3() {
        return driver.findElement(By.xpath("//*[@data-test='categoryItem_3']"));
    }
    public WebElement category32() {
        return driver.findElement(By.xpath("//*[@data-test='categoryItem_32']"));
    }
    public WebElement attributtesSelect() {
        return driver.findElement(By.xpath("//*[@data-test='customAttributtesSelect']"));
    }
    public WebElement size() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Size']"));
    }
    public WebElement addAttributtes() {
        return driver.findElement(By.xpath("//*[@data-test='customAttributtesAddButton']"));
    }
    public WebElement material() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Material']"));
    }
    public WebElement colour() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Colour']"));
    }
    public WebElement brown() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Brown']"));
    }
    public WebElement blue() {
        return driver.findElement(By.xpath("//*[@data-test='characteristicSelect_item_Colour_Blue']"));
    }
    public WebElement glass() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Glass']"));
    }
    public WebElement metal() {
        return driver.findElement(By.xpath("//*[@data-test='characteristicSelect_item_Material_Metal']"));
    }
    public WebElement size52() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='52']"));
    }
    public WebElement size46() {
        return driver.findElement(By.xpath("//*[@data-test='characteristicSelect_item_Size_46']"));
    }
    public WebElement saveButton() {
        return driver.findElement(By.xpath("//*[@data-test='saveProductButton']"));
    }
    public WebElement price() {
        return driver.findElement(By.xpath("//*[@data-test='variantPriceInputInput']"));
    }
    public WebElement casheBack() {
        return driver.findElement(By.xpath("//*[@data-test='variantCashbackInput']"));
    }

    public void additem_product(int number) throws InterruptedException {

        try {
            if (number != 0)
            goods().click();
            driver.findElement(By.xpath("//div[normalize-space(text())='Goods']"));
            addProduct().click();
            productName().sendKeys("product" + number);
            shotrDescription().sendKeys(Generator.generateShortDescription());
            categorySelector().click();

            Random random = new Random();
            List<String> keys = new ArrayList<String>(Category.categories.keySet());
            String category1 = keys.get(random.nextInt(keys.size()));
            HashMap<String, List> categoryList2 = Category.categories.get(category1);
            keys = new ArrayList<String>(categoryList2.keySet());
            String category2 = keys.get(random.nextInt(keys.size()));
            List categoryList3 = categoryList2.get(category2);
            String category3 = (String) categoryList3.get(random.nextInt(categoryList3.size()));

            driver.findElement(By.xpath(category1)).click();
            driver.findElement(By.xpath(category2)).click();
            driver.findElement(By.xpath(category3)).click();

            //category1().click();
            //category3().click();
            //category32().click();

            attributtesSelect().click();
            size().click();
            addAttributtes().click();
            material().click();
            addAttributtes().click();
            colour().click();
            addAttributtes().click();
            brown().click();
            blue().click();
            glass().click();
            metal().click();
            size52().click();
            size46().click();
            price().sendKeys(Generator.generatePrice());
            casheBack().sendKeys(Generator.generateCashback());
            saveButton().click();
            Wait.sleep(driver, 20);
        }
        catch (Exception e) {
            System.out.println("AddItemError" + e.getMessage());
            additem_product(-- number);
            screenshot.takesScreenshot("AddItem" + Getdate.Date());
        }

        AddVariant addVariant = new AddVariant(driver);
        for (int i = 0; i < VARIANTS_AMOUNT; i++) {
            addVariant.addvariant_variant(i);
        }

        Thread.sleep(6000);

    }
}
