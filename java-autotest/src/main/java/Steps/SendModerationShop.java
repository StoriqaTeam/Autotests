package Steps;

import Helper.Getdate;
import Helper.Screenshot;
import Helper.SeleniumRunner;
import Helper.Wait;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

@RunWith(SeleniumRunner.class)

public class SendModerationShop {

    WebDriver driver;
    Screenshot screenshot;

    public SendModerationShop(WebDriver driver) {this.driver = driver; screenshot = new Screenshot(driver); }

    public WebElement sendModerationShop() {
        return driver.findElement(By.xpath("//*[@data-test='sendToModerationStoreButton']"));
    }
    public WebElement storeIsOnModeration() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Store is on moderation']"));
    }

    public void sendmoderation_shop() throws InterruptedException {
        try {
            Wait.sleep(driver, 10);
            sendModerationShop().click();
//            sendmoderation_shop();
//            storeIsOnModeration();
        }
        catch (Exception e) {
            System.out.print("SendModerationShopError" + e.getMessage());
            screenshot.takesScreenshot("SendModerationShop" + Getdate.Date());
        }
    }
}
