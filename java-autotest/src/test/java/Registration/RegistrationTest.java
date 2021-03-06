package Registration;

import Page.autorization_page.AutorizationLoc;
import Helper.Generator;
import Helper.SeleniumRunner;
import Helper.Wait;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;
import org.openqa.selenium.chrome.ChromeDriver;


@RunWith(SeleniumRunner.class)


public class RegistrationTest {

    WebDriver driver;

    public WebElement signUp() {
        return driver.findElement(By.xpath("//*[@data-test='headerSignUpButton']"));
    }
    public WebElement firstName() {
        return driver.findElement(By.xpath("//*[@data-test='firstName']"));
    }
    public WebElement lastName() {
        return driver.findElement(By.xpath("//*[@data-test='lastName']"));
    }
    public WebElement upButton() {
        return driver.findElement(By.xpath("//*[@data-test='signUpButton']"));
    }
    public WebElement login() {
        return driver.findElement(By.xpath("//*[@data-test='email']"));
    }
    public WebElement password() {
        return driver.findElement(By.xpath("//*[@data-test='password']"));
    }
    public WebElement terms() {
        return driver.findElement(By.xpath("//*[@data-test='terms']"));
    }
    public WebElement privacy() {
        return driver.findElement(By.xpath("//*[@data-test='privacy']"));
    }

    @Before
    public void setUp(){
        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
        driver = new ChromeDriver();
//        System.setProperty("webdriver.opera.driver", "/Users/user/operadriver");
//        driver = new OperaDriver();
//        System.setProperty("webdriver.gecko.driver", "/Users/user/geckodriver");
//        driver = new FirefoxDriver();
    }
    //WebDriver driver = new ChromeDriver();


    @Test
    public void Registration_Test() throws InterruptedException {

        try {
            driver.get("https://storiqa:whenLambo%3F@stage.stq.cloud/auth");
            signUp().click();
            Wait.sleep(driver, 10);
            firstName().sendKeys(Generator.generateFirstName());
            lastName().sendKeys(Generator.generateLastName());
            login().sendKeys(Generator.generateEmail());
            password().sendKeys(Generator.generatePass());
            terms().click();
            privacy().click();
            upButton().click();
        }
        catch (Exception e) {
            System.out.println("RegistrationTest");
            Registration_Test();
        }

        driver.close();
        }
    }
