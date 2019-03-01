package Steps;

import Helper.Generator;
import Helper.Getdate;
import Helper.Screenshot;
import Helper.SeleniumRunner;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

import static Helper.Constants.DELL;
import static Helper.Generator.generateShortDescription;
import static Helper.Generator.generateText;

@RunWith(SeleniumRunner.class)

public class EditShop {

    WebDriver driver;
    Screenshot screenshot;

    public EditShop(WebDriver driver) {this.driver = driver; screenshot = new Screenshot(driver); }

    public WebElement settings() {
        return driver.findElement(By.xpath("//*[@data-test='Settings']"));
    }
    public WebElement storeName() {
        return driver.findElement(By.xpath("//*[@data-test='name']"));
    }
    public WebElement slogan() {
        return driver.findElement(By.xpath("//*[@data-test='slogan']"));
    }
    public WebElement webSite() {
        return driver.findElement(By.xpath("//*[@data-test='slug']"));
    }
    public WebElement shortDescription() {
        return driver.findElement(By.xpath("//*[@data-test='shortDescription']"));
    }
    public WebElement saveStore() {
        return driver.findElement(By.xpath("//*[@data-test='saveStoreButton']"));
    }

    public void edit_shop() throws InterruptedException {
        try {
            settings().click();
//            Wait.sleep(driver, 10);
            Thread.sleep(4000);
            storeName().sendKeys(DELL);
            storeName().sendKeys("my edit test shop");
            slogan().sendKeys(generateText());
            webSite().sendKeys(DELL);
            webSite().sendKeys(Generator.generateSite());
            shortDescription().sendKeys(DELL);
            shortDescription().sendKeys(generateShortDescription());
            saveStore().click();
        }
        catch (Exception e){
            System.out.println("EditShopError" + e.getMessage());
            edit_shop();
            screenshot.takesScreenshot("EditShop" + Getdate.Date());
        }
    }
}
