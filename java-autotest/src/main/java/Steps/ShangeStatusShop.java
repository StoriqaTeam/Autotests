package Steps;

import Helper.Getdate;
import Helper.Screenshot;
import Helper.SeleniumRunner;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;


@RunWith(SeleniumRunner.class)

public class ShangeStatusShop {

    WebDriver driver;
    Screenshot screenshot;

//    public ShangeStatusShop(WebDriver driver){
//        this.driver = driver;
//        screenshot = new Screenshot(driver);
//    }

    public ShangeStatusShop(WebDriver driver) {
        this.driver = driver;
        screenshot = new Screenshot(driver);
    }

    public WebElement userDropdown() {
        return driver.findElement(By.xpath("//*[@data-test='userDropdownButton']"));
    }
    public WebElement myShops() {
        return driver.findElement(By.xpath("//*[@data-test='header-user-menu-myShops']"));
    }
    public WebElement goods() {
        return driver.findElement(By.xpath("//*[@data-test='Goods']"));
    }

    public void status_shop() throws InterruptedException {

        try {
            Thread.sleep(3000);
            userDropdown().click();
            myShops().click();
            goods().click();
        }

        catch (Exception e) {
            System.out.println("ShangeStatusShopError" + e.getMessage());
            screenshot.takesScreenshot("ShangeStatusShop" + Getdate.Date());
        }
    }
}
