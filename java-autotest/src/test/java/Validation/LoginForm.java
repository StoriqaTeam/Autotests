package Validation;

import Helper.Getdate;
import Helper.Screenshot;
import Helper.Wait;
import org.junit.Before;
import org.junit.Test;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;
import org.openqa.selenium.chrome.ChromeDriver;
import org.openqa.selenium.support.ui.WebDriverWait;

import static Helper.Constants.LOGIN_SHOP;
import static Helper.Constants.PASS;
import static Helper.ValidationsLogin.INVALID_LOGINS;

public class LoginForm {
    WebDriver driver;
    Screenshot screenshot;

    @Before
    public void setUp() {
        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
        driver = new ChromeDriver();
        screenshot = new Screenshot(driver);
//        wait = new WebDriverWait(driver, 30);
    }


    public WebElement login() {
        return driver.findElement(By.xpath("//*[@data-test='email']"));
    }

    public WebElement signIn() {
        return driver.findElement(By.xpath("//*[@data-test='headerSignInButton']"));
    }

    public WebElement pass() {
        return driver.findElement(By.xpath("//*[@data-test='password']"));
    }

    public WebElement inButton() {
        return driver.findElement(By.xpath("//*[@data-test='signInButton']"));
    }

    @Test
    public void login_form() throws InterruptedException {
        driver.manage().window().maximize();
        Wait.sleep(driver, 10);
        driver.get("https://storiqa:whenLambo%3F@stage.stq.cloud/auth");        //Переходим на главную страницу административной панели.
        signIn().click();

    }

    private void testInvalidLogins() {
        for (int i = 0; i < INVALID_LOGINS.length; i++) {
            login().sendKeys(INVALID_LOGINS[i]);
            pass().sendKeys(PASS);
            Wait.sleep(driver, 20);
//            wait.untill(invisibilityOfElementLoccated(inButton()));
            try {
                inButton().click();
                driver.findElement(By.xpath("//div[normalize-space(text())='Hi, Russkii']"));
                junit.framework.Assert.assertEquals("https://stage.stq.cloud/", driver.getCurrentUrl());
            } catch (Exception e) {
                System.out.println("TEST_LOG Autorization Error" + e.getMessage());
                screenshot.takesScreenshot("Autorization" + Getdate.Date());
            }
        }
    }
}
