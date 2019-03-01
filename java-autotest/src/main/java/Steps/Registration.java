package Steps;

import Helper.*;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

@RunWith(SeleniumRunner.class)

public class Registration {
    WebDriver driver;
    Screenshot screenshot;

    public Registration(WebDriver driver){
        this.driver = driver;
        screenshot = new Screenshot(driver);
    }

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

    public void registration_user() {
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
            System.out.println("RegistrationUser" + e.getMessage());
            registration_user();
            screenshot.takesScreenshot("Registration" + Getdate.Date());
        }
    }
}
