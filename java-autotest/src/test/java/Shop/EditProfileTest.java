package Shop;

import Page.shop.Shop;
import Steps.Autorization;
import Helper.SeleniumRunner;
import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.Keys;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;

import java.io.IOException;

@RunWith(SeleniumRunner.class)

public class EditProfileTest {

    WebDriver driver;

    @Before
    public void setUp() {
        System.setProperty("webdriver.chrome.driver", "/Users/user/chromedriver");
        driver = new ChromeDriver();
    }

    @Test
    public void EditProfile_Test() throws Exception {

        Autorization user = new Autorization(driver);
        user.autorization_login();

        Assert.assertEquals("Storiqa", driver.getTitle());
        driver.findElement(By.xpath("//div[normalize-space(text())='Hi, user22684 U.']"));
        junit.framework.Assert.assertEquals("https://stage.stq.cloud/", driver.getCurrentUrl());
        driver.findElement(By.xpath("//*[@data-test='userDropdownButton']")).click();
        driver.findElement(By.xpath("//*[@data-test='header-user-menu-profileLink']")).click();

        driver.findElement(By.xpath("//*[@data-test='firstNameInput']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
        driver.findElement(By.xpath("//*[@data-test='lastNameInput']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");

        driver.findElement(By.xpath("//*[@data-test='firstNameInput']")).sendKeys("Slava");
        driver.findElement(By.xpath("//*[@data-test='lastNameInput']")).sendKeys("Geroyam");
        driver.findElement(By.xpath("//*[@data-test='savePersonalDataButton']")).click();
        driver.findElement(By.xpath("//div[normalize-space(text())='Hi, Slava G.']"));

        driver.findElement(By.xpath("//*[@data-test='firstNameInput']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
        driver.findElement(By.xpath("//*[@data-test='lastNameInput']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");

        driver.findElement(By.xpath("//*[@data-test='firstNameInput']")).sendKeys("user22684");
        driver.findElement(By.xpath("//*[@data-test='lastNameInput']")).sendKeys("user22684");
        driver.findElement(By.xpath("//*[@data-test='savePersonalDataButton']")).click();
        driver.findElement(By.xpath("//div[normalize-space(text())='Hi, user22684 U.']"));

        driver.findElement(By.xpath("//*[@data-test='profileGenderSelect']")).click();
        driver.findElement(By.xpath("//div[normalize-space(text())='Male']")).click();

        driver.findElement(By.xpath("//*[@data-test='profileBirthdate-year']")).click();
        driver.findElement(By.xpath("//div[normalize-space(text())='2015']")).click();

        driver.findElement(By.xpath("//*[@data-test='profileBirthdate-month']")).click();
        driver.findElement(By.xpath("//div[normalize-space(text())='March']")).click();

        driver.findElement(By.xpath("//*[@data-test='profileBirthdate-day']")).click();
        driver.findElement(By.xpath("//div[normalize-space(text())='01']")).click();

        driver.findElement(By.xpath("//*[@data-test='phoneInput']")).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
        driver.findElement(By.xpath("//*[@data-test='phoneInput']")).sendKeys("79999999900");
        driver.findElement(By.xpath("//*[@data-test='savePersonalDataButton']")).click();

        //Shipping Addresses

//        driver.findElement(By.xpath("//*[@data-test='header-user-menu-profileLink']")).click();
        //driver.findElement(By.xpath("//a[@href='/profile/shipping-addresses']")).click();
        driver.get("https://stage.stq.cloud/profile/shipping-addresses");
        driver.findElement(By.xpath("//*[@data-test='addShippingAddressButton']")).click();
        driver.findElement(By.xpath(Shop.COUNTRY.getCl())).click();
        driver.findElement(By.xpath(Shop.RUSSIA.getCl())).click();
        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys("\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b");
        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys("Tverskaya Street, 1");
        driver.findElement(By.xpath(Shop.ADDRES.getCl())).sendKeys(Keys.ENTER);
        Thread.sleep(5000);
        driver.findElement(By.xpath(Shop.STREET.getCl())).click();
        driver.findElement(By.xpath("//*[@data-test='postalCode']")).sendKeys("125009");
        driver.findElement(By.xpath("//*[@data-test='streetNumber']")).sendKeys("1");
        driver.findElement(By.xpath("//*[@data-test='locality']")).sendKeys("Moscow");
        driver.findElement(By.xpath("//*[@data-test='administrativeAreaLevel2']")).sendKeys("Moscow");
        driver.findElement(By.xpath("//*[@data-test='route']")).sendKeys("Tverskaya Street");
        driver.findElement(By.xpath("//*[@data-test='saveShippingAddressButton']")).click();
//        driver.close();
    }
}
