package Shop;

import Page.shop.Shop;
import Steps.Autorization;
import Helper.*;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.Keys;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;

import java.io.IOException;


@RunWith(SeleniumRunner.class)

public class AddProductTest {

    WebDriver driver;

    @Before
    public void setUp(){
        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
        driver = new ChromeDriver();
    }

    //WebDriver driver = new ChromeDriver();

    @Test
    public void AddProduct_Test() throws Exception {

        Autorization user = new Autorization(driver);
        user.autorization_login();

//        Createshop first = new Createshop(driver);
//        first.create_shop();

        org.junit.Assert.assertEquals("Storiqa", driver.getTitle());
        driver.findElement(By.xpath("//div[normalize-space(text())='Hi, user22684 U.']"));
//        org.junit.Assert.assertEquals("https://stage.stq.cloud/", driver.getCurrentUrl());
//        driver.findElement(By.xpath(Shop.SELLONSTORIQA.getCl())).click();

        driver.get("https://stage.stq.cloud/manage/wizard?");

         //Переходим в визард

        driver.findElement(By.xpath(Shop.STORENAME.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
        driver.findElement(By.xpath(Shop.WEBADDRESS.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
        //Thread.sleep(7000);
        driver.findElement(By.xpath(Shop.SHORTDESCRIPTION.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");

        //Thread.sleep(10000);

        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).isDisplayed();
        driver.findElement(By.xpath(Shop.STORENAME.getCl())).sendKeys("my test shop");
        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).isDisplayed();
//        Thread.sleep(7000);
        driver.findElement(By.xpath(Shop.WEBADDRESS.getCl())).sendKeys(Generator.generateSite());
//        Thread.sleep(7000);
        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).isDisplayed();
        driver.findElement(By.xpath(Shop.SHORTDESCRIPTION.getCl())).sendKeys("my test shop");
        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).isEnabled();
//        Thread.sleep(3000);
//        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        Wait.sleep(driver, 10);
        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).click();
//        Thread.sleep(10000);

//        driver.findElement(By.xpath("//div[normalize-space(text())='Set up store']"));
        driver.findElement(By.xpath(Shop.MAINLANGUAGE.getCl())).click();
        driver.findElement(By.xpath(Shop.ENGLISH.getCl())).click();
        driver.findElement(By.xpath(Shop.COUNTRY.getCl())).click();
//        driver.findElement(By.xpath("//*[@data-test='stq-input']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        driver.findElement(By.xpath(Shop.ALBANIA.getCl())).click();
//        driver.findElement(By.xpath("//*[.='Albania']")).click();
//        driver.findElement(By.xpath(Shop.COUNTRY.getCl())).click();
//        driver.findElement(By.xpath("//*[@data-test='stq-input']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
        driver.findElement(By.xpath(Shop.RUSSIA.getCl())).click();
        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys("Tverskaya Street, 1");
        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys(Keys.ENTER);
        Thread.sleep(4000);
        driver.findElement(By.xpath(Shop.STREET.getCl())).click();
        driver.findElement(By.xpath("//*[@data-test='streetNumber'='1']"));
        driver.findElement(By.xpath("//*[@data-test='locality'='Moscow']"));
        driver.findElement(By.xpath("//*[@data-test='administrativeAreaLevel2'='Moscow']"));
        driver.findElement(By.xpath("//*[@data-test='route'='Tverskaya Street']"));
        driver.findElement(By.xpath("//*[@data-test='postalCode'='125009']"));
        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).click();

//        driver.findElement(By.xpath("//div[normalize-space(text())='Fill your store with goods']"));
        driver.findElement(By.xpath(Shop.ADDFIRSTPRODUCTFOTO.getCl())).click();
        driver.findElement(By.xpath("//div[normalize-space(text())='Add new product']"));
//        driver.findElement(By.xpath("//*[@data-test='wizardBackButton']")).click();
//        driver.findElement(By.xpath("//*[@data-test='closeWizard']")).click();
//        driver.findElement(By.xpath("//*[@data-test='wizardUploaderProductFotoFirst']")).click();

        //Создание продукта

        driver.findElement(By.xpath("//div[normalize-space(text())='Add new product']"));
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
        driver.findElement(By.xpath(Shop.VENDORCODE.getCl())).sendKeys(Generator.generateSite());
        driver.findElement(By.xpath(Shop.CASHEBACK.getCl())).sendKeys("10");
        driver.findElement(By.xpath(Shop.QUANTITY.getCl())).sendKeys("100");
        //driver.findElement(By.xpath(Shop.CURRENCY.getCl())).click();
        //driver.manage().timeouts().implicitlyWait(40, TimeUnit.SECONDS);
        //driver.findElement(By.xpath(Shop.STQ.getCl())).sendKeys(Keys.ENTER);
        driver.findElement(By.xpath(Shop.SAVEPRODUCTBUTTON.getCl())).click();
//        driver.findElement(By.xpath("//div[normalize-space(text())='Fill your store with goods']"));
        driver.findElement(By.xpath("//*[@data-test='wizardBackButton']")).click();
        driver.findElement(By.xpath("//*[@data-test='publishPopupWrapper']")).click();
        driver.findElement(By.xpath("//*[@data-test='cancelPublishWizardButton']")).click();
//        driver.findElement(By.xpath("//div[normalize-space(text())='my produkt']"));
//        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).click();
        //driver.switchTo().window()
//        driver.findElement(By.xpath("//*[@data-test='closeWizard']")).click();

//        Element.byXpath(driver, "//*[@data-test='wizardBackButton']");


        driver.close();
    }
}
