package Helper;

import org.openqa.selenium.WebDriver;

import java.util.concurrent.TimeUnit;

public class Wait {

    public static void sleep(WebDriver driver, int time) {
        driver.manage().timeouts().implicitlyWait(time, TimeUnit.SECONDS);
    }

}


// Wait.sleep(driver, 10);