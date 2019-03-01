package Shop;

import Steps.Autorization;
import Helper.Generator;
import Helper.SeleniumRunner;
import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;

import java.io.IOException;
import java.util.concurrent.TimeUnit;

@RunWith(SeleniumRunner.class)

public class AddProductSearchTest {

    WebDriver driver;

    @Before
    public void setUp() {
        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
        driver = new ChromeDriver();
    }

    @Test
    public void AddProductSearch_Test() throws Exception {

        Autorization user = new Autorization(driver);
        user.autorization_login();

        Assert.assertEquals("Storiqa", driver.getTitle());
        driver.findElement(By.xpath("//div[normalize-space(text())='Hi, user22684 U.']"));
        junit.framework.Assert.assertEquals("https://stage.stq.cloud/", driver.getCurrentUrl());
        driver.findElement(By.xpath("//*[@data-test='userDropdownButton']")).click();
        driver.findElement(By.xpath("//*[@data-test='header-user-menu-myShops']")).click();
        driver.findElement(By.xpath("//*[@data-test='store-menu-goods']")).click();
        driver.findElement(By.xpath("//div[normalize-space(text())='Goods']"));

        driver.findElement(By.xpath("//*[@data-test='addProductButton']")).click();
        driver.findElement(By.xpath("//*[@data-test='name']")).sendKeys("Test");
        driver.findElement(By.xpath("//*[@data-test='shortDescription']")).sendKeys("Test");
        driver.findElement(By.xpath("//*[@data-test='longDescription']")).sendKeys("Test");
        driver.findElement(By.xpath("//*[@data-test='vendorCode']")).sendKeys(Generator.generateSite());
        driver.findElement(By.xpath("//*[@data-test='categorySelector']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_46']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_47']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_48']")).click();
        driver.findElement(By.xpath("//*[@data-test='variantPriceInputInput']")).sendKeys("11");
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();

        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();
        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();
        //driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        //Thread.sleep(7000);
        driver.findElement(By.xpath("//*[@data-test='store-menu-goods']")).click();

        driver.findElement(By.xpath("//*[@data-test='addProductButton']")).click();
        driver.findElement(By.xpath("//*[@data-test='name']")).sendKeys("NeTest");
        driver.findElement(By.xpath("//*[@data-test='shortDescription']")).sendKeys("NeTest");
        driver.findElement(By.xpath("//*[@data-test='longDescription']")).sendKeys("NeTest");
        driver.findElement(By.xpath("//*[@data-test='vendorCode']")).sendKeys(Generator.generateSite());
        driver.findElement(By.xpath("//*[@data-test='categorySelector']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_46']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_47']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_48']")).click();
        driver.findElement(By.xpath("//*[@data-test='variantPriceInputInput']")).sendKeys("11");
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();

        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        //driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();
        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();
        driver.manage().timeouts().implicitlyWait(20, TimeUnit.SECONDS);
        //Thread.sleep(7000);
        driver.findElement(By.xpath("//*[@data-test='store-menu-goods']")).click();

        driver.findElement(By.xpath("//*[@data-test='addProductButton']")).click();
        driver.findElement(By.xpath("//*[@data-test='name']")).sendKeys("PoluTest");
        driver.findElement(By.xpath("//*[@data-test='shortDescription']")).sendKeys("PoluTest");
        driver.findElement(By.xpath("//*[@data-test='longDescription']")).sendKeys("PoluTest");
        driver.findElement(By.xpath("//*[@data-test='vendorCode']")).sendKeys(Generator.generateSite());
        driver.findElement(By.xpath("//*[@data-test='categorySelector']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_46']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_47']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_48']")).click();
        driver.findElement(By.xpath("//*[@data-test='variantPriceInputInput']")).sendKeys("11");
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();

        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();
        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();
        //driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        //Thread.sleep(7000);
        driver.findElement(By.xpath("//*[@data-test='store-menu-goods']")).click();

        driver.findElement(By.xpath("//*[@data-test='addProductButton']")).click();
        driver.findElement(By.xpath("//*[@data-test='name']")).sendKeys("Testovii");
        driver.findElement(By.xpath("//*[@data-test='shortDescription']")).sendKeys("Testovii");
        driver.findElement(By.xpath("//*[@data-test='longDescription']")).sendKeys("Testovii");
        driver.findElement(By.xpath("//*[@data-test='vendorCode']")).sendKeys(Generator.generateSite());
        driver.findElement(By.xpath("//*[@data-test='categorySelector']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_46']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_47']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_48']")).click();
        driver.findElement(By.xpath("//*[@data-test='variantPriceInputInput']")).sendKeys("11");
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();

        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();
        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();
        //driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        //Thread.sleep(7000);
        driver.findElement(By.xpath("//*[@data-test='store-menu-goods']")).click();

        driver.findElement(By.xpath("//*[@data-test='addProductButton']")).click();
        driver.findElement(By.xpath("//*[@data-test='name']")).sendKeys("12Test");
        driver.findElement(By.xpath("//*[@data-test='shortDescription']")).sendKeys("12Test");
        driver.findElement(By.xpath("//*[@data-test='longDescription']")).sendKeys("12Test");
        driver.findElement(By.xpath("//*[@data-test='vendorCode']")).sendKeys(Generator.generateSite());
        driver.findElement(By.xpath("//*[@data-test='categorySelector']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_46']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_47']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_48']")).click();
        driver.findElement(By.xpath("//*[@data-test='variantPriceInputInput']")).sendKeys("11");
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();

        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();
        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();
        //driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        //Thread.sleep(7000);
        driver.findElement(By.xpath("//*[@data-test='store-menu-goods']")).click();

        driver.findElement(By.xpath("//*[@data-test='addProductButton']")).click();
        driver.findElement(By.xpath("//*[@data-test='name']")).sendKeys("Te12st");
        driver.findElement(By.xpath("//*[@data-test='shortDescription']")).sendKeys("Te12st");
        driver.findElement(By.xpath("//*[@data-test='longDescription']")).sendKeys("Te12st");
        driver.findElement(By.xpath("//*[@data-test='vendorCode']")).sendKeys(Generator.generateSite());
        driver.findElement(By.xpath("//*[@data-test='categorySelector']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_46']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_47']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_48']")).click();
        driver.findElement(By.xpath("//*[@data-test='variantPriceInputInput']")).sendKeys("11");
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();

        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();
        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();
        //driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        //Thread.sleep(7000);
        driver.findElement(By.xpath("//*[@data-test='store-menu-goods']")).click();

        driver.findElement(By.xpath("//*[@data-test='addProductButton']")).click();
        driver.findElement(By.xpath("//*[@data-test='name']")).sendKeys("TestTest");
        driver.findElement(By.xpath("//*[@data-test='shortDescription']")).sendKeys("TestTest");
        driver.findElement(By.xpath("//*[@data-test='longDescription']")).sendKeys("TestTest");
        driver.findElement(By.xpath("//*[@data-test='vendorCode']")).sendKeys(Generator.generateSite());
        driver.findElement(By.xpath("//*[@data-test='categorySelector']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_46']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_47']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_48']")).click();
        driver.findElement(By.xpath("//*[@data-test='variantPriceInputInput']")).sendKeys("11");
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();

        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();
        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();
        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        driver.findElement(By.xpath("//*[@data-test='store-menu-goods']")).click();

        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        driver.findElement(By.xpath("//*[@data-test='addProductButton']")).click();
        driver.findElement(By.xpath("//*[@data-test='name']")).sendKeys("NoTestNo");
        driver.findElement(By.xpath("//*[@data-test='shortDescription']")).sendKeys("NoTestNo");
        driver.findElement(By.xpath("//*[@data-test='longDescription']")).sendKeys("NoTestNo");
        driver.findElement(By.xpath("//*[@data-test='vendorCode']")).sendKeys(Generator.generateSite());
        driver.findElement(By.xpath("//*[@data-test='categorySelector']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_46']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_47']")).click();
        driver.findElement(By.xpath("//*[@data-test='categoryItem_48']")).click();
        driver.findElement(By.xpath("//*[@data-test='variantPriceInputInput']")).sendKeys("11");
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();

        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();
        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();
        driver.manage().timeouts().implicitlyWait(10, TimeUnit.SECONDS);
        driver.findElement(By.xpath("//*[@data-test='store-menu-goods']")).click();
    }
}
