package Shop;

import Steps.Autorization;
import Helper.SeleniumRunner;
import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;

import java.io.IOException;

@RunWith(SeleniumRunner.class)

public class EditBaseProductTest {

    WebDriver driver;

    @Before
    public void setUp() {
        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
        driver = new ChromeDriver();
    }

    @Test
    public void AddProduct_Test() throws Exception {

        Autorization user = new Autorization(driver);
        user.autorization_login();

        Assert.assertEquals("Storiqa", driver.getTitle());
        driver.findElement(By.xpath("//div[normalize-space(text())='Hi, user22684 U.']"));
        junit.framework.Assert.assertEquals("https://stage.stq.cloud/", driver.getCurrentUrl());
        driver.findElement(By.xpath("//*[@data-test='userDropdownButton']")).click();
        driver.findElement(By.xpath("//*[@data-test='header-user-menu-myShops']")).click();
        driver.findElement(By.xpath("//div[normalize-space(text())='Settings']"));

        //Edit Base product

        driver.findElement(By.xpath("//*[@data-test='store-menu-goods']")).click();
        driver.findElement(By.xpath("//div[normalize-space(text())='Goods']"));
        driver.findElement(By.xpath("//*[@data-test='editProductButton']")).click();
        //driver.findElement(By.xpath("//*[@xpath='1'='DRAFT']"));
        driver.findElement(By.xpath("//*[@data-test='name']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
        driver.findElement(By.xpath("//*[@data-test='name']")).sendKeys("Alll shop");

//        driver.findElement(By.xpath("//*[@data-test='productPhotosUploader']")).click();
//        driver.findElement(By.xpath("//*[@data-test='productPhotosUploader']")).(fileUpload);
//        driver.findElement(By.xpath("//*[@data-test='productPhotosUploader']")).sendKeys("/Пользователи/user/2018-11-30 11.57.18.jpg");

//        driver.findElement(By.xpath("//*[@data-test='productPhotosUploader']")).click();
//        Thread.sleep(9000);
//        File file =new File("/Users/user/2018-11-30 11.57.18.jpg");
//        driver.findElement(By.xpath("//*[@data-test='productPhotosUploader']")).sendKeys(file.getAbsolutePath(), (Keys.ENTER));

        driver.findElement(By.xpath("//*[@data-test='shortDescription']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
        driver.findElement(By.xpath("//*[@data-test='shortDescription']")).sendKeys("russkii test");

        driver.findElement(By.xpath("//*[@data-test='longDescription']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
        driver.findElement(By.xpath("//*[@data-test='longDescription']")).sendKeys("russkii test");

        driver.findElement(By.xpath("//*[@data-test='seoTitle']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
        driver.findElement(By.xpath("//*[@data-test='seoTitle']")).sendKeys("russkii test");

        driver.findElement(By.xpath("//*[@data-test='seoDescription']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
        driver.findElement(By.xpath("//*[@data-test='seoDescription']")).sendKeys("russkii test");

        driver.findElement(By.xpath("//*[@data-test='variantPriceInputInput']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
        driver.findElement(By.xpath("//*[@data-test='variantPriceInputInput']")).sendKeys("50");

        driver.findElement(By.xpath("//*[@data-test='variantCashbackInput']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
        driver.findElement(By.xpath("//*[@data-test='variantCashbackInput']")).sendKeys("10");

        driver.findElement(By.xpath("//*[@data-test='saveProductButton']")).click();

        driver.close();
    }
}