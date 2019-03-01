package Shop;

import Page.shop.Shop;
import Steps.Autorization;
import Helper.SeleniumRunner;
import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.Keys;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;

import java.io.IOException;

@RunWith(SeleniumRunner.class)

public class EditShopTest {

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

        //Settings

        Assert.assertEquals("Storiqa", driver.getTitle());
        driver.findElement(By.xpath("//div[normalize-space(text())='Hi, user22684 U.']"));
        junit.framework.Assert.assertEquals("https://stage.stq.cloud/", driver.getCurrentUrl());
        driver.findElement(By.xpath("//*[@data-test='userDropdownButton']")).click();
        driver.findElement(By.xpath("//*[@data-test='header-user-menu-myShops']")).click();
        driver.findElement(By.xpath("//div[normalize-space(text())='Settings']"));
        driver.findElement(By.xpath(Shop.STORENAME.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
        driver.findElement(By.xpath(Shop.STORENAME.getCl())).sendKeys("Russkii shop");
        driver.findElement(By.xpath(Shop.SLOGAN.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
        driver.findElement(By.xpath(Shop.SLOGAN.getCl())).sendKeys("ГБО");
        driver.findElement(By.xpath(Shop.SHORTDESCRIPTION.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
        driver.findElement(By.xpath(Shop.SHORTDESCRIPTION.getCl())).sendKeys("testovii test");
//        driver.findElement(By.xpath(Shop.LONGDESCRIPTION.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        driver.findElement(By.xpath(Shop.LONGDESCRIPTION.getCl())).sendKeys("My shop");
        driver.findElement(By.xpath("//*[@data-test='saveStoreButton']")).click();

        //Storages Edit

//        driver.findElement(By.xpath(Shop.STORAGESSETTINGS.getCl())).click();
//        driver.findElement(By.xpath("//div[normalize-space(text()='Storages')]"));
//        driver.findElement(By.xpath(Shop.EDITSTORAGEADMIN.getCl())).click();
//        driver.findElement(By.xpath(Shop.STORENAME.getCl())).sendKeys("Sore for test");
//        //driver.findElement(By.xpath(Shop.COUNTRY.getCl())).click();
//        Thread.sleep(5000);
//        //driver.findElement(By.xpath(Shop.COUNTRY.getCl())).sendKeys(Keys.DELETE);
//        //driver.findElement(By.xpath(Shop.COUNTRY.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
//        driver.findElement(By.xpath("//*[@data-test='AddressFormSelect']")).sendKeys("Belarus");
//        //driver.findElement(By.xpath(Shop.COUNTRY.getCl())).sendKeys("Belarus");
//        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys("Praspyekt Nyezalyezhnastsi 1");

        //Contacts Edit

        driver.findElement(By.xpath("//*[@data-test='store-menu-contacts']")).click();
        driver.findElement(By.xpath("//*[@data-test='email']")).sendKeys("test@test.ru");
        driver.findElement(By.xpath("//*[@data-test='phone']")).sendKeys("79012345678");
        driver.findElement(By.xpath("//*[@data-test='facebookUrl']")).sendKeys("facebook.com");
        driver.findElement(By.xpath("//*[@data-test='instagramUrl']")).sendKeys("insta.ru");
        driver.findElement(By.xpath("//*[@data-test='twitterUrl']")).sendKeys("twitter.ru");

        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys("Tverskaya Street, 2");
        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys(Keys.ENTER);
        Thread.sleep(4000);
        driver.findElement(By.xpath(Shop.STREET.getCl())).click();
        driver.findElement(By.xpath("//*[@data-test='streetNumber']")).sendKeys("2");
        driver.findElement(By.xpath("//*[@data-test='locality']")).sendKeys("Moskva");
        driver.findElement(By.xpath("//*[@data-test='administrativeAreaLevel2']")).sendKeys("Moskva");
        driver.findElement(By.xpath("//*[@data-test='route']")).sendKeys("Tverskaya Street");
        driver.findElement(By.xpath("//*[@data-test='postalCode']")).sendKeys("125009");
        driver.findElement(By.xpath("//*[@data-test='stqButton']")).click();
    }
}
