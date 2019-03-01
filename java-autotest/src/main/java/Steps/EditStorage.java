package Steps;

import Helper.Getdate;
import Helper.Screenshot;
import Helper.SeleniumRunner;
import Helper.Wait;
import com.ibm.icu.impl.UResource;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.Keys;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

import static Helper.Constants.*;

@RunWith(SeleniumRunner.class)

public class EditStorage {

    WebDriver driver;
    Screenshot screenshot;

    public EditStorage (WebDriver driver) { this.driver = driver; screenshot = new Screenshot(driver); }

    public WebElement storages() {
        return driver.findElement(By.xpath("//*[@data-test='Storages']"));
    }
    public WebElement editStorage() {
        return driver.findElement(By.xpath("//*[@data-test='editStorageDataButton']"));
    }
    public WebElement nameStorage() {
        return driver.findElement(By.xpath("//*[@data-test='name']"));
    }

    public WebElement address() {
        return driver.findElement(By.xpath("//*[@data-test='autocompleteAddress']"));
    }

    public WebElement streetNumber() {
        return driver.findElement(By.xpath("//*[@data-test='streetNumber']"));
    }

    public WebElement streetAddress() {
        return driver.findElement(By.xpath("//*[@data-test='route']"));
    }

    public WebElement locality() {
        return driver.findElement(By.xpath("//*[@data-test='locality']"));
    }

    public WebElement region() {
        return driver.findElement(By.xpath("//*[@data-test='administrativeAreaLevel2']"));
    }

    public WebElement postalCode() {
        return driver.findElement(By.xpath("//*[@data-test='postalCode']"));
    }
    public WebElement saveButton() {
        return driver.findElement(By.xpath("//*[@data-test='cancelSaveStorageButton']"));
    }

    public void edit_storage() throws InterruptedException {

        try {
            Thread.sleep(3000);
            storages().click();
            Wait.sleep(driver, 10);
            editStorage().click();
            nameStorage().sendKeys(DELL);
            nameStorage().sendKeys(NAME_SHOP);
            address().sendKeys(DELL);
            address().sendKeys(ADDRESS2);
            address().sendKeys(Keys.ENTER);
            Wait.sleep(driver, 15);
            streetNumber().sendKeys(APART);
            locality().sendKeys(LOCALITY);
            region().sendKeys("Moscow");
            streetAddress().sendKeys("Semenovskaya Ploshchad'");
            postalCode().sendKeys("105318");
            saveButton().click();
        }

        catch (Exception e) {
            System.out.println("TEST_LOG EditStorage Error" + e.getMessage());
            screenshot.takesScreenshot("EditStorage" + Getdate.Date());
        }
    }
}
