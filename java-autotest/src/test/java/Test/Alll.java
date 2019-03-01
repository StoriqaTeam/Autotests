package Test;

import Steps.*;
import Helper.SeleniumRunner;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;
import org.openqa.selenium.chrome.ChromeDriver;


@RunWith(SeleniumRunner.class)

public class Alll {

    WebDriver driver;

    public WebElement signIn() {
        return driver.findElement(By.xpath("//*[@data-test='headerSignInButton']"));
    }

    @Before
    public void setUp() {
        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
        driver = new ChromeDriver();
//        System.setProperty("webdriver.opera.driver", "/Users/user/operadriver");
//        driver = new OperaDriver();
//        System.setProperty("webdriver.gecko.driver", "/Users/user/geckodriver");
//        driver = new FirefoxDriver();
    }

    @Test
    public void Test() throws InterruptedException {

        driver.get("https://storiqa:whenLambo%3F@stage.stq.cloud/auth");        //Переходим на главную страницу административной панели.
        signIn().click();
//        driver.close();
    }
}