package Steps;

import Helper.Getdate;
import Helper.Screenshot;
import Helper.SeleniumRunner;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

@RunWith(SeleniumRunner.class)

public class LogoutAdmin {

    WebDriver driver;
    Screenshot screenshot;

    public LogoutAdmin(WebDriver driver) {
        this.driver = driver;
        screenshot = new Screenshot(driver);
    }

    public WebElement logOut() {
        return driver.findElement(By.xpath("//*[@data-test='email']"));
    }

    public void logout_admin() throws InterruptedException {

        try {
            driver.findElement(By.xpath("//div[normalize-space(text())='Storiqa admin panel']"));
        }

        catch (Exception e) {
            System.out.println("TEST_LOG LogoutAdmin Error" + e.getMessage());
            screenshot.takesScreenshot("LogoutAdmin Error" + Getdate.Date());
        }
    }
}
