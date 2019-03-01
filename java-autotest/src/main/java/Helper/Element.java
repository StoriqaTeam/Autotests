package Helper;

import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;

public class Element {


    WebDriver driver;

    public Element(WebDriver driver) {
        this.driver = driver;
    }

    public WebElement saveButton() {
        return driver.findElement(By.xpath("//*[@data-test='saveButton']"));
    }

    public WebElement address() {
        return driver.findElement(By.xpath("//*[@data-test='autocompleteAddress']"));
    }

    public WebElement dropDown() {
        return driver.findElement(By.xpath("//*[@data-test='userDropdownButton']"));
    }

    public WebElement myShop() {
        return driver.findElement(By.xpath("//*[@data-test='header-user-menu-myShops']"));
    }

    public WebElement goods() {
        return driver.findElement(By.xpath("//*[@data-test='Goods']"));
    }

    public WebElement addProduct() {
        return driver.findElement(By.xpath("//*[@data-test='addProductButton']"));
    }

    public WebElement productName() {
        return driver.findElement(By.xpath("//*[@data-test='name']"));
    }

    public WebElement storeSettings() {
        return driver.findElement(By.xpath("//*[@data-test='store-menu-storages']"));
    }

    public static WebElement byXpath(WebDriver driver, String s) {

        return driver.findElement(By.xpath(s));
    }
}

// element.address().sendKeys(ADDRESS);
