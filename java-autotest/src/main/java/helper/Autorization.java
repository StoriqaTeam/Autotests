package helper;

import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import Page.autorization_page.AutorizationLoc;

import java.util.concurrent.TimeUnit;

public class Autorization {
    WebDriver driver;



    public Autorization(WebDriver driver){
        this.driver = driver;
    }

    public void autorization_login() {
        driver.manage().window().maximize();
        System.out.print("Page title is: " + driver.getTitle());       //Проверка title
        driver.get("https://storiqa:whenLambo%3F@stage.stq.cloud/auth");        //Переходим на главную страницу административной панели.
        driver.findElement(By.xpath(AutorizationLoc.SIGNIN.getCl())).click();
        driver.findElement(By.xpath(AutorizationLoc.LOGIN.getCl())).sendKeys("k.russkikh@storiqa.com");
        driver.findElement(By.xpath(AutorizationLoc.PASSWORD.getCl())).sendKeys("1234567Qq");
        driver.manage().timeouts().implicitlyWait(15, TimeUnit.SECONDS);
        driver.findElement(By.xpath(AutorizationLoc.ENTER.getCl())).click();
        driver.findElement(By.xpath("//div[normalize-space(text())='Hi, Russkii T.']"));
        System.out.print("Page title is: " + driver.getTitle());       //Проверка title
    }
}
