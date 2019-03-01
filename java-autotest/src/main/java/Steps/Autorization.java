package Steps;

import Helper.Getdate;
import Helper.Screenshot;
import Helper.SeleniumRunner;
import Helper.Wait;
import org.junit.runner.RunWith;
import org.openqa.selenium.*;


import java.io.IOException;

import static Helper.Constants.LOGIN_SHOP;
import static Helper.Constants.PASS;

@RunWith(SeleniumRunner.class)

public class Autorization {
    WebDriver driver;
    Screenshot screenshot;


//        final private String MY_LOGIN = "22684@crapmail.tld";
//        final private String MY_PASS = "M4n3b2v1";

    public Autorization(WebDriver driver){
        this.driver = driver;
        screenshot = new Screenshot(driver);
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



    public void autorization_login() throws InterruptedException {
//        driver.manage().window().maximize();
////        Wait.sleep(driver, 10);
////        driver.get("https://storiqa:whenLambo%3F@stage.stq.cloud/auth");        //Переходим на главную страницу административной панели.
////        signIn().click();
////        login().sendKeys(LOGIN_SHOP);
////        pass().sendKeys(PASS);
////        Wait.sleep(driver, 20);
////        inButton().click();
////        driver.findElement(By.xpath("//div[normalize-space(text())='Hi, user22684 U.']"));
////        junit.framework.Assert.assertEquals("https://stage.stq.cloud/", driver.getCurrentUrl());

        try {
            driver.manage().window().maximize();
            Wait.sleep(driver, 10);
            driver.get("https://storiqa:whenLambo%3F@stage.stq.cloud/auth");        //Переходим на главную страницу административной панели.
            signIn().click();
            login().sendKeys(LOGIN_SHOP);
            pass().sendKeys(PASS);
            Wait.sleep(driver, 20);
            inButton().click();
            driver.findElement(By.xpath("//div[normalize-space(text())='Hi, Russkii']"));
            junit.framework.Assert.assertEquals("https://stage.stq.cloud/", driver.getCurrentUrl());
        }

        catch (Exception e) {
            System.out.println("TEST_LOG Autorization Error" + e.getMessage());
            screenshot.takesScreenshot("Autorization" + Getdate.Date());
        }
    }
}
