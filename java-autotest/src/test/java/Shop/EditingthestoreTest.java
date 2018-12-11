package Shop;

import helper.Addproduct;
import helper.Autorization;
import helper.Createshop;
import helper.SeleniumRunner;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;
import org.openqa.selenium.firefox.FirefoxDriver;

@RunWith(SeleniumRunner.class)

public class EditingthestoreTest {

    WebDriver driver = new ChromeDriver();
    //WebDriver driver = new FirefoxDriver();

    @Test
    public void Editingthestore_Test() throws InterruptedException {

        Autorization user = new Autorization (driver);
        user.autorization_login();

//        Createshop first = new Createshop (driver);
//        first.create_shop();
//
//        Addproduct add = new Addproduct(driver);
//        add.add_product();

        driver.findElement(By.xpath("//*[@data-test='userDropdownButton']")).click();
        driver.findElement(By.xpath("//*[@data-test='header-user-menu-myShops']")).click();
        driver.findElement(By.xpath("//*[@data-test='name']")).sendKeys();
    }
}
