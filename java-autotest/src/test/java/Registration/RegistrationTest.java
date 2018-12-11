package Registration;

import Page.autorization_page.AutorizationLoc;
import helper.Generator;
import helper.SeleniumRunner;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;


import java.util.concurrent.TimeUnit;

import static junit.framework.Assert.assertEquals;

@RunWith(SeleniumRunner.class)


public class RegistrationTest {

    WebDriver driver;

    @Before
    public void setUp(){
        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
        driver = new ChromeDriver();
    }
    //WebDriver driver = new ChromeDriver();


        @Test
        public void Registration_Test() throws InterruptedException {

            driver.get("https://storiqa:whenLambo%3F@stage.stq.cloud/auth");
            driver.findElement(By.xpath(AutorizationLoc.SIGNUP.getCl())).click();
            //driver.findElement(By.xpath(AutorizationLoc.ENTER.getCl())).isDisplayed();
            driver.findElement(By.xpath("//div[normalize-space(text())='Sign Up']"));
            driver.findElement(By.xpath(AutorizationLoc.FIRSTNAME.getCl())).sendKeys("Ivan");
            driver.findElement(By.xpath(AutorizationLoc.LASTNAME.getCl())).sendKeys("Petrov");
            driver.findElement(By.xpath(AutorizationLoc.LOGIN.getCl())).sendKeys(Generator.generateEmail());
            driver.findElement(By.xpath(AutorizationLoc.PASSWORD.getCl())).sendKeys("1234567Qq");
            driver.manage().timeouts().implicitlyWait(90, TimeUnit.SECONDS);
            //driver.findElement(By.xpath(AutorizationLoc.TERMS.getCl())).click();
            //driver.manage().timeouts().implicitlyWait(40, TimeUnit.SECONDS);
            driver.findElement(By.xpath(AutorizationLoc.PRIVASY.getCl())).click();
            driver.findElement(By.xpath(AutorizationLoc.ENTER.getCl())).click();


            driver.close();
        }
    }
