package Helper;

import org.openqa.selenium.WebDriver;

public class ParameterizationURL {

    WebDriver driver;

    String[] paths = driver.getCurrentUrl().split("/");
    String productId = paths[paths.length-1];

}
