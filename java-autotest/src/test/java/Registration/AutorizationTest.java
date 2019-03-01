package Registration;


import Helper.SeleniumRunner;
import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;
import Page.autorization_page.AutorizationLoc;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;


import java.util.concurrent.TimeUnit;

import static junit.framework.Assert.assertEquals;

@RunWith(SeleniumRunner.class)

public class AutorizationTest {

    WebDriver driver;

    @Before
    public void setUp(){
        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
        driver = new ChromeDriver();
    }

    //WebDriver driver = new ChromeDriver();

    @Test
    public void Autorization_Test() throws InterruptedException {

        driver.get("https://storiqa:whenLambo%3F@stage.stq.cloud/auth");
        Assert.assertEquals("Storiqa", driver.getTitle());
        //driver.findElement(By.xpath("//div[@class='AuthButtons__signUpButton___2jzhw']")).click();
        driver.findElement(By.xpath(AutorizationLoc.SIGNIN.getCl())).click();
        driver.findElement(By.xpath(AutorizationLoc.LOGIN.getCl())).sendKeys("22684@crapmail.tld");
        driver.findElement(By.xpath(AutorizationLoc.PASSWORD.getCl())).sendKeys("M4n3b2v1");
        driver.manage().timeouts().implicitlyWait(15, TimeUnit.SECONDS);
        driver.findElement(By.xpath(AutorizationLoc.ENTER.getCl())).click();
        driver.findElement(By.xpath("//div[normalize-space(text())='Hi, user22684 U.']"));

        driver.close();
    }
}
