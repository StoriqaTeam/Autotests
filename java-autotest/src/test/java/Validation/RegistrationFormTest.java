package Validation;

import Page.autorization_page.AutorizationLoc;
import helper.SeleniumRunner;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;
import org.testng.annotations.Test;

@RunWith(SeleniumRunner.class)

public class RegistrationFormTest {

    WebDriver driver = new ChromeDriver();

    @Test
    public void RegistrationForm_Test() throws InterruptedException {

        driver.get("https://storiqa:whenLambo%3F@stage.stq.cloud/auth");
        driver.findElement(By.xpath(AutorizationLoc.SIGNUP.getCl())).click();
        driver.findElement(By.xpath(AutorizationLoc.ENTER.getCl())).isDisplayed();
        }
    }
