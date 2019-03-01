package Shop;

import Steps.AddItem;
import Steps.Autorization;
import Helper.*;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.WebElement;
import org.openqa.selenium.chrome.ChromeDriver;

import java.io.IOException;

@RunWith(SeleniumRunner.class)

public class AddVariantTest {

    static final int PRODUCTS_AMOUNT = 9;

    WebDriver driver;

    public WebElement deliveryTab() {
        return driver.findElement(By.xpath("//*[@data-test='deliveryTab']"));
    }
    public WebElement localShippingprice() {
        return driver.findElement(By.xpath("//*[@data-test='localShippingFixPrice']"));
    }
    public WebElement localPickup() {
        return driver.findElement(By.xpath("//*[@data-test='localPickupCheckbox']"));
    }
    public WebElement lshippingPrice() {
        return driver.findElement(By.xpath("//*[@data-test='shippingPickupPriceInput']"));
    }
    public WebElement localShippingPrice() {
        return driver.findElement(By.xpath("//*[@data-test='shippingLocalServicePriceInput']"));
    }
    public WebElement shippingAdd() {
        return driver.findElement(By.xpath("//*[@data-test='shippingAddCompanyButton']"));
    }
    public WebElement interShippingprice() {
        return driver.findElement(By.xpath("//*[@data-test='interShippingFixPrice']"));
    }
    public WebElement iShippingPrice() {
        return driver.findElement(By.xpath("//*[@data-test='shippingInterServicePriceInput']"));
    }
    public WebElement selectAllcountries() {
        return driver.findElement(By.xpath("//*[@data-test='shipping-all-countries']"));
    }
    public WebElement saveShipping() {
        return driver.findElement(By.xpath("//*[@data-test='saveShippingButton']"));
    }
    public WebElement dropDown() {
        return driver.findElement(By.xpath("//*[@data-test='userDropdownButton']"));
    }
    public WebElement myShop() {
        return driver.findElement(By.xpath("//*[@data-test='header-user-menu-myShops']"));
    }

    @Before
    public void setUp() {
        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
        driver = new ChromeDriver();
    }

    @Test
    public void AddVariant_Test() throws Exception {

        Autorization user = new Autorization(driver);
        user.autorization_login();

//        Assert.assertEquals("Storiqa", driver.getTitle());
//        driver.findElement(By.xpath("//div[normalize-space(text())='Hi, user22684 U.']"));
//        junit.framework.Assert.assertEquals("https://stage.stq.cloud/", driver.getCurrentUrl());

        dropDown().click();
        myShop().click();

        AddItem addItem = new AddItem(driver);
        for (int i = 0; i < PRODUCTS_AMOUNT; i++) {
            addItem.additem_product(i);
        }

        Thread.sleep(5000);

        //Добавление доставки.
//        driver.findElement(By.xpath("//*[@data-test='deliveryTab']")).click();
        deliveryTab().click();
//        driver.findElement(By.xpath("//*[@data-test='localShippingFixPrice']")).click();
        localShippingprice().click();
//        driver.findElement(By.xpath("//*[@data-test='localPickupCheckbox']")).click();
        localPickup().click();
//        driver.findElement(By.xpath("//*[@data-test='shippingPickupPriceInput']")).sendKeys("55");
        lshippingPrice().sendKeys("55");
//        driver.findElement(By.xpath("//*[@data-test='shippingLocalServicePriceInput']")).sendKeys("55");
        localShippingPrice().sendKeys("55");
//        driver.findElement(By.xpath("//*[@data-test='shippingAddCompanyButton']")).click();
        shippingAdd().click();
//        driver.findElement(By.xpath("//*[@data-test='interShippingFixPrice']")).click();
        interShippingprice().click();
//        driver.findElement(By.xpath("//*[@data-test='shippingInterServicePriceInput']")).sendKeys("55");
        iShippingPrice().sendKeys("55");
//        driver.findElement(By.xpath("//*[@data-test='shipping-all-countries']")).click();
        Wait.sleep(driver, 15);
        selectAllcountries().sendKeys();
//        driver.findElement(By.xpath("//*[@data-test='shippingAddCompanyButton']")).click();
        shippingAdd().sendKeys();
//        driver.findElement(By.xpath("//*[@data-test='saveShippingButton']")).click();
        saveShipping().click();

        driver.close();
    }
}
