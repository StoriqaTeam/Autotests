package Steps;

import Helper.Screenshot;
import Helper.SeleniumRunner;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

@RunWith(SeleniumRunner.class)

public class ChangeStatus {

    WebDriver driver;
    Screenshot screenshot;

    public ChangeStatus(WebDriver driver) { this.driver = driver; screenshot = new Screenshot(driver); }

    public WebElement stausDropdown() {
        return driver.findElement(By.xpath("//*[@data-test='storiqaIcon']"));
    }
}
