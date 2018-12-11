package Registration;

import Page.autorization_page.AutorizationLoc;
import helper.SeleniumRunner;
import javafx.scene.input.DataFormat;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;


import java.text.DateFormat;
import java.text.SimpleDateFormat;

import static junit.framework.Assert.assertEquals;

@RunWith(SeleniumRunner.class)

public class RegistrationTest {

    WebDriver driver = new ChromeDriver();

    //DateFormat dateFormat = new SimpleDateFormat("MM/dd/yyyy HH:mm:ss");

        @Test
        public void Registration_Test() throws InterruptedException {

            driver.get("https://storiqa:whenLambo%3F@stage.stq.cloud/auth");
            driver.findElement(By.xpath(AutorizationLoc.SIGNUP.getCl())).click();
            driver.findElement(By.xpath(AutorizationLoc.ENTER.getCl())).isDisplayed();
            driver.findElement(By.xpath("//div[normalize-space(text())='Sign Up']"));
            driver.findElement(By.xpath(AutorizationLoc.FIRSTNAME.getCl())).sendKeys("Ivan");
            driver.findElement(By.xpath(AutorizationLoc.LASTNAME.getCl())).sendKeys("Petrov");
            driver.findElement(By.xpath(AutorizationLoc.LOGIN.getCl())).sendKeys("Date()" );
            driver.findElement(By.xpath(AutorizationLoc.PASSWORD.getCl())).sendKeys("1234567Qq");
            driver.findElement(By.xpath(AutorizationLoc.TERMS.getCl())).click();
            driver.findElement(By.xpath(AutorizationLoc.PRIVASY.getCl())).click();
            driver.findElement(By.xpath(AutorizationLoc.ENTER.getCl())).click();


            driver.close();
        }
    }
