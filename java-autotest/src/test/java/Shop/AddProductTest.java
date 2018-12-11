package Shop;

import Page.shop.Shop;
import helper.Autorization;
import helper.Createshop;
import helper.SeleniumRunner;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;



@RunWith(SeleniumRunner.class)

public class AddProductTest {

    WebDriver driver = new ChromeDriver();

    @Test
    public void AddProduct_Test() throws InterruptedException {

        Autorization user = new Autorization(driver);
        user.autorization_login();

        Createshop first =  new Createshop(driver);
        first.create_shop();


        //Создание продукта

        driver.findElement(By.xpath(Shop.PRODUCTNAME.getCl())).sendKeys("my produkt");
        driver.findElement(By.xpath(Shop.SHORTDESCRIPTION.getCl())).sendKeys("test produkt");
        driver.findElement(By.xpath(Shop.PRODUKTCATEGORY.getCl())).click();
        driver.findElement(By.xpath("//div[normalize-space(text())='Office & School Supplies']"));
        driver.findElement(By.xpath("//*[@data-test='categoryItem_46']")).click();
        driver.findElement(By.xpath("//div[normalize-space(text())='Calendars, Planners & Cards']"));
        driver.findElement(By.xpath("//*[@data-test='categoryItem_47']")).click();
        driver.findElement(By.xpath("//div[normalize-space(text())='Postcards']"));
        driver.findElement(By.xpath("//*[@data-test='categoryItem_48']")).click();
        driver.findElement(By.xpath(Shop.PRICE.getCl())).sendKeys("1000");
        driver.findElement(By.xpath(Shop.VENDORCODE.getCl())).sendKeys("123");
        driver.findElement(By.xpath(Shop.CASHEBACK.getCl())).sendKeys("10");
        driver.findElement(By.xpath(Shop.QUANTITY.getCl())).sendKeys("100");
        //driver.findElement(By.xpath(Shop.CURRENCY.getCl())).click();
        //driver.manage().timeouts().implicitlyWait(40, TimeUnit.SECONDS);
        //driver.findElement(By.xpath(Shop.STQ.getCl())).sendKeys(Keys.ENTER);
        driver.findElement(By.xpath(Shop.SAVEPRODUCTBUTTON.getCl())).click();
        driver.findElement(By.xpath("//div[normalize-space(text())='Fill your store with goods']"));
        driver.findElement(By.xpath("//div[normalize-space(text())='my produkt']"));

        driver.close();
    }
}
