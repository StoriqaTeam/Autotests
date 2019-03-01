package Shop;

import Steps.Autorization;
import Helper.SeleniumRunner;
import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;
import org.openqa.selenium.firefox.FirefoxDriver;

import java.io.IOException;

@RunWith(SeleniumRunner.class)

public class SearchShopTest {

    WebDriver driver;

    @Before
    public void setUp() {
//        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
//        driver = new ChromeDriver();

        System.setProperty("webdriver.gecko.driver", "/Users/user/geckodriver");
        driver = new FirefoxDriver();
    }
    @Test
    public void SearchShop_Test() throws Exception {

        Autorization user = new Autorization(driver);
        user.autorization_login();

//        Assert.assertEquals("Storiqa", driver.getTitle());
//        driver.findElement(By.xpath("//div[normalize-space(text())='Hi, Russkii T.']"));
//        junit.framework.Assert.assertEquals("https://stage.stq.cloud/", driver.getCurrentUrl());

        driver.findElement(By.xpath("//*[@data-test='searchInputSelect']")).click();
        driver.findElement(By.id("id='stores'")).click();
    }
}
