package Steps;

import Helper.Getdate;
import Helper.Screenshot;
import Helper.SeleniumRunner;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

import static Helper.Constants.*;

@RunWith(SeleniumRunner.class)

public class AddContacts {

    WebDriver driver;
    Screenshot screenshot;

    public AddContacts (WebDriver driver) { this.driver = driver; screenshot = new Screenshot(driver); }

    public WebElement contacts() {
        return driver.findElement(By.xpath("//*[@data-test='Contacts']"));
    }
    public WebElement email() {
        return driver.findElement(By.xpath("//*[@data-test='email']"));
    }
    public WebElement phone() {
        return driver.findElement(By.xpath("//*[@data-test='phone']"));
    }
    public WebElement facebookUrl() {
        return driver.findElement(By.xpath("//*[@data-test='facebookUrl']"));
    }
    public WebElement instagramUrl() {
        return driver.findElement(By.xpath("//*[@data-test='instagramUrl']"));
    }
    public WebElement twitterUrl() {
        return driver.findElement(By.xpath("//*[@data-test='twitterUrl']"));
    }
    public WebElement saveButton() {
        return driver.findElement(By.xpath("//*[@data-test='stqButton']"));
    }

    public void add_contacts() throws InterruptedException {

        try {
            Thread.sleep(3000);
            contacts().click();
            email().sendKeys(DELL);
            phone().sendKeys(DELL);
            facebookUrl().sendKeys(DELL);
            twitterUrl().sendKeys(DELL);
            instagramUrl().sendKeys(DELL);
            email().sendKeys(EMAIL);
            phone().sendKeys(PHONE);
            facebookUrl().sendKeys(FACEBOOK);
            instagramUrl().sendKeys(INSTA);
            twitterUrl().sendKeys(TWITTER);
            saveButton().click();
        }

        catch (Exception e) {
            System.out.println("AddContacts Error" + e.getMessage());
            screenshot.takesScreenshot("AddContacts" + Getdate.Date());
        }
    }
}
