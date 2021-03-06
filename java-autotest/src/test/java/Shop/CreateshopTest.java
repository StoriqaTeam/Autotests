package Shop;

import Page.shop.Shop;
import Steps.Autorization;
import Helper.*;
import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.Keys;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;

import java.io.IOException;
import java.util.concurrent.TimeUnit;

import static Helper.Constants.ADDRESS;


@RunWith(SeleniumRunner.class)

public class CreateshopTest {

    WebDriver driver;
    Element element;

    @Before
    public void setUp(){
        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
        driver = new ChromeDriver();
        element = new Element(driver);
    }

    //WebDriver driver = new ChromeDriver();

    @Test
    public void Createshop_Test() throws Exception {

        Autorization user = new Autorization(driver);
        user.autorization_login();


        Assert.assertEquals("Storiqa", driver.getTitle());
        driver.findElement(By.xpath("//div[normalize-space(text())='Hi, user22684 U.']"));
        Assert.assertEquals("https://stage.stq.cloud/", driver.getCurrentUrl());
        driver.findElement(By.xpath(Shop.SELLONSTORIQA.getCl())).click();
        //Assert.assertEquals("https://stage.stq.cloud/start-selling/en", driver.getCurrentUrl());
        //driver.findElement(By.xpath("//div[normalize-space(text())='Online crypto marketplace']"));
//        driver.findElement(By.xpath(Shop.STARTSELLING.getCl())).click();

        driver.get("https://stage.stq.cloud/manage/wizard?");

        // Переходим в визард
//        Assert.assertEquals("https://stage.stq.cloud/manage/wizard", driver.getCurrentUrl());
//        driver.findElement(By.xpath("//div[normalize-space(text())='Give your store a name']"));

        driver.findElement(By.xpath(Shop.STORENAME.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
        driver.findElement(By.xpath(Shop.WEBADDRESS.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        Thread.sleep(7000);
        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        driver.findElement(By.xpath(Shop.SHORTDESCRIPTION.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");

        //Thread.sleep(10000);


        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).isDisplayed();
        driver.findElement(By.xpath(Shop.STORENAME.getCl())).sendKeys("my test shop");
        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).isDisplayed();
        driver.findElement(By.xpath(Shop.WEBADDRESS.getCl())).sendKeys("test123qwe");
        //Thread.sleep(7000);
        //driver.findElement(By.xpath("//div[normalize-space(text())='Vacant']"));
        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).isDisplayed();
        driver.findElement(By.xpath(Shop.SHORTDESCRIPTION.getCl())).sendKeys("my test shop");
        //driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).isEnabled();
//        Thread.sleep(3000);
        Wait.sleep(driver, 20);
        driver.findElement(By.xpath("//*[@data-test='wizardBackButton']")).click();
//        Thread.sleep(10000);

        driver.findElement(By.xpath("//div[normalize-space(text())='Set up store']"));
        driver.findElement(By.xpath(Shop.MAINLANGUAGE.getCl())).click();
        driver.findElement(By.xpath(Shop.ENGLISH.getCl())).click();
        driver.findElement(By.xpath(Shop.COUNTRY.getCl())).click();
        //Thread.sleep(5000);
        //driver.findElement(By.xpath("//*[@data-test='AddressFormSelect']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        driver.findElement(By.xpath(Shop.ALBANIA.getCl())).click();
//        driver.findElement(By.xpath(Shop.COUNTRY.getCl())).click();
        driver.findElement(By.xpath(Shop.RUSSIA.getCl())).click();
        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys("Tverskaya Street, 1");
//        Element.byXpath(driver, Shop.ADDRES.getCl()).sendKeys(ADDRESS);
        element.address().sendKeys(ADDRESS);
        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys(Keys.ENTER);
//        Thread.sleep(5000);
        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        driver.findElement(By.xpath(Shop.STREET.getCl())).click();
        driver.findElement(By.xpath("//*[@data-test='streetNumber'='1']"));
        driver.findElement(By.xpath("//*[@data-test='locality'='Moscow']"));
        driver.findElement(By.xpath("//*[@data-test='administrativeAreaLevel2'='Moscow']"));
        driver.findElement(By.xpath("//*[@data-test='route'='Tverskaya Street']"));
        driver.findElement(By.xpath("//*[@data-test='postalCode'='125009']"));
        driver.findElement(By.xpath(Shop.NEXTSTEPBUTTON.getCl())).click();

        driver.findElement(By.xpath("//div[normalize-space(text())='Fill your store with goods']"));
        driver.findElement(By.xpath(Shop.ADDFIRSTPRODUCTFOTO.getCl())).click();
        driver.findElement(By.xpath("//div[normalize-space(text())='Add new product']"));

//        driver.close();
    }
}