package Steps;

import Helper.Getdate;
import Helper.Screenshot;
import Helper.SeleniumRunner;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

@RunWith(SeleniumRunner.class)

public class CreateDeliveryPackage {

    WebDriver driver;
    Screenshot screenshot;

    public CreateDeliveryPackage(WebDriver driver) {
        this.driver = driver;
        screenshot = new Screenshot(driver);
    }

    public WebElement delivery() {
        return driver.findElement(By.xpath("//*[@data-test='menu-delivery']"));
    }

    public void delivery_package() throws InterruptedException {

        try {
            delivery().click();
        }

        catch (Exception e) {
            System.out.println("CreateDeliveryPackage Error" + e.getMessage());
            screenshot.takesScreenshot("CreateDeliveryPackage" + Getdate.Date());
        }
    }
}
