package Validation;

import Page.autorization_page.AutorizationLoc;
import Helper.SeleniumRunner;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;

import static org.openqa.selenium.support.ui.ExpectedConditions.elementToBeClickable;

@RunWith(SeleniumRunner.class)

public class RegistrationFormTest {

    WebDriver driver;

    @Before
    public void setUp(){
        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
        driver = new ChromeDriver();
    }

    //WebDriver driver = new ChromeDriver();

    @Test
    public void RegistrationForm_Test() throws InterruptedException {

        driver.get("https://storiqa:whenLambo%3F@stage.stq.cloud/auth");
        driver.findElement(By.xpath(AutorizationLoc.SIGNUP.getCl())).click();

        //driver.findElement(By.xpath(AutorizationLoc.ENTER.getCl())).click();
        //public static ExpectedCondition elementToBeClickable(By.ByXPath("//*[@data-test='signInButton']"));
        }
    }
