package Shop;

import Page.autorization_page.AutorizationLoc;
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
import java.util.concurrent.TimeUnit;

@RunWith(SeleniumRunner.class)

public class ChangePasswordTest {

    WebDriver driver;

    @Before
    public void setUp() {
        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
        driver = new ChromeDriver();
    }

    @Test
    public void ChangePassword_Test() throws Exception {

        Autorization user = new Autorization(driver);
        user.autorization_login();

        Assert.assertEquals("Storiqa", driver.getTitle());
        driver.findElement(By.xpath("//div[normalize-space(text())='Hi, user22684 U.']"));
        junit.framework.Assert.assertEquals("https://stage.stq.cloud/", driver.getCurrentUrl());
        driver.findElement(By.xpath("//*[@data-test='userDropdownButton']")).click();
        driver.findElement(By.xpath("//*[@data-test='header-user-menu-profileLink']")).click();
//        driver.findElement(By.xpath("//*[@data-test='header-user-menu-profileLink']")).click();
        driver.get("https://stage.stq.cloud/profile/security");
        driver.findElement(By.xpath("//*[@data-test='oldPasswordInput']")).sendKeys("M4n3b2v1");
        driver.findElement(By.xpath("//*[@data-test='newPasswordInput']")).sendKeys("1234567Qq");
        driver.findElement(By.xpath("//*[@data-test='repeatNewPasswordInput']")).sendKeys("1234567Qq");
        driver.findElement(By.xpath("//*[@data-test='saveSecuritySettingsButton']")).click();
        driver.findElement(By.xpath("//*[@data-test='userDropdownButton']")).click();
        driver.findElement(By.xpath("//*[@data-test='header-user-menu-logoutLink']")).click();
        driver.findElement(By.xpath(AutorizationLoc.SIGNIN.getCl())).click();
        driver.findElement(By.xpath(AutorizationLoc.LOGIN.getCl())).sendKeys("22684@crapmail.tld");
        driver.findElement(By.xpath(AutorizationLoc.PASSWORD.getCl())).sendKeys("1234567Qq");
        driver.manage().timeouts().implicitlyWait(15, TimeUnit.SECONDS);
        driver.findElement(By.xpath(AutorizationLoc.ENTER.getCl())).click();
        driver.findElement(By.xpath("//div[normalize-space(text())='Hi, user22684 U.']"));
        junit.framework.Assert.assertEquals("https://stage.stq.cloud/", driver.getCurrentUrl());
        driver.findElement(By.xpath("//*[@data-test='userDropdownButton']")).click();
        driver.findElement(By.xpath("//*[@data-test='header-user-menu-profileLink']")).click();
//        driver.findElement(By.xpath("//*[@data-test='header-user-menu-profileLink']")).click();
        driver.get("https://stage.stq.cloud/profile/security");
        driver.findElement(By.xpath("//*[@data-test='oldPasswordInput']")).sendKeys("1234567Qq");
        driver.findElement(By.xpath("//*[@data-test='newPasswordInput']")).sendKeys("M4n3b2v1");
        driver.findElement(By.xpath("//*[@data-test='repeatNewPasswordInput']")).sendKeys("M4n3b2v1");
        driver.findElement(By.xpath("//*[@data-test='saveSecuritySettingsButton']")).click();

        driver.close();
    }
}
