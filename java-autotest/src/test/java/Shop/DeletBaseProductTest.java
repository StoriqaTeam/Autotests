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

import java.io.IOException;

@RunWith(SeleniumRunner.class)

public class DeletBaseProductTest {

    WebDriver driver;

    @Before
    public void setUp(){
        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
        driver = new ChromeDriver();
    }

    //WebDriver driver = new ChromeDriver();

    @Test
    public void DeletBaseProduct_Test() throws Exception {

        Autorization user = new Autorization(driver);
        user.autorization_login();

        Assert.assertEquals("Storiqa", driver.getTitle());
        driver.findElement(By.xpath("//div[normalize-space(text())='Hi, user22684 U.']"));
        Assert.assertEquals("https://stage.stq.cloud/", driver.getCurrentUrl());
        //driver.findElement(By.xpath(Shop.SELLONSTORIQA.getCl())).click();
        //Assert.assertEquals("https://stage.stq.cloud/start-selling/en", driver.getCurrentUrl());
        //driver.findElement(By.xpath("//div[normalize-space(text())='Online crypto marketplace']"));
        //driver.findElement(By.xpath(Shop.STARTSELLING.getCl())).click();

        driver.findElement(By.xpath("//*[@data-test='userDropdownButton']")).click();
        driver.findElement(By.xpath("//*[@data-test='header-user-menu-myShops']")).click();
        driver.findElement(By.xpath("//*[@data-test='store-menu-goods']")).click();
        driver.findElement(By.xpath("//*[@data-test='deleteProductButton']")).click();

        driver.close();
    }
}
