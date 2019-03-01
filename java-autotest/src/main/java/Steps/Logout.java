package Steps;

import Helper.Getdate;
import Helper.Screenshot;
import Helper.SeleniumRunner;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

@RunWith(SeleniumRunner.class)

public class Logout {
    WebDriver driver;
    Screenshot screenshot;

    public Logout(WebDriver driver){
        this.driver = driver;
        screenshot = new Screenshot(driver);
    }

    public WebElement dropDown() {
        return driver.findElement(By.xpath("//*[@data-test='userDropdownButton']"));
    }
    public WebElement logOut() {
        return driver.findElement(By.xpath("//*[@data-test='header-user-menu-logoutLink']"));
    }

    public void logot_user() {
        try {
            Thread.sleep(20000);
//            WebElement element = wait.until(ExpectedConditions.elementToBeClickable(By.xpath("//*[@data-test='userDropdownButton']")));
//            element.click();
//            driver.findElement(By.xpath("//div[normalize-space(text())='Hi, user22684 U.']")).click();
//            Thread.sleep(5000);
            dropDown().click();
            Thread.sleep(20000);
            logOut().click();
        }
        catch (Exception e) {
            System.out.println("LogOutError" + e.getMessage());
            screenshot.takesScreenshot("LogOut" + Getdate.Date());
        }
    }
}
