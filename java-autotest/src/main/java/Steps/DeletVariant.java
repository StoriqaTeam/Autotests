package Steps;


import Helper.*;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

@RunWith(SeleniumRunner.class)

public class DeletVariant {

    WebDriver driver;
    Screenshot screenshot;

    public DeletVariant(WebDriver driver) {
        this.driver = driver;
        screenshot = new Screenshot(driver);
    }

    public WebElement goods() {
        return driver.findElement(By.xpath("//*[@data-test='Goods']"));
    }
    public WebElement deleteVariant() {
        return driver.findElement(By.xpath("//*[@data-test='" + DeleteProduct.getRandomDelete() + "']"));
    }
//    public WebElement pop_upp() {
//        return driver.findElement(By.xpath("//*[@id='inner']"));
//    }
    public WebElement confirm_delete() {
        return driver.findElement(By.xpath("//*[@data-test='confirm']"));
    }
    public WebElement pop_upp() {
        return driver.findElement(By.xpath("//div[normalize-space(text())='Delete your product?']"));
    }


    public void delete_variant() throws InterruptedException {

        try {
            Thread.sleep(3000);
            goods().click();
            Thread.sleep(5000);
            deleteVariant().click();
            pop_upp().click();
            confirm_delete().click();
        }

        catch (Exception e) {
            System.out.println("TEST_LOG DeletVariant Error" + e.getMessage());
            screenshot.takesScreenshot("DeletVariant" + Getdate.Date());
        }
    }
}
