package Steps;

import Helper.Getdate;
import Helper.Screenshot;
import Helper.SeleniumRunner;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

@RunWith(SeleniumRunner.class)

public class SearchProducts {

    WebDriver driver;
    Screenshot screenshot;

    public SearchProducts(WebDriver driver) { this.driver = driver; screenshot = new Screenshot(driver); }

    public WebElement searchInput() {
        return driver.findElement(By.xpath("//*[@data-test='searchInput']"));
    }
    public WebElement searchInputSelectWrap() {
        return driver.findElement(By.xpath("//*[@data-test='icon']"));
    }
    public WebElement searchButton() {
        return driver.findElement(By.xpath("//*[@data-test='searchButton']"));
    }

    public void search_products() throws InterruptedException {

        try {
            Thread.sleep(3000);
//            searchInputSelectWrap().click();
//            searchInput().click();
            searchInput().sendKeys("Russkii");
            searchButton().click();
//            String[] paths = driver.getCurrentUrl().split("/");
//            String productId = paths[paths.length-1];
        }

        catch (Exception e) {
            System.out.println("SearchProducts" + e.getMessage());
            screenshot.takesScreenshot("SearchProducts" + Getdate.Date());
        }
    }
}
