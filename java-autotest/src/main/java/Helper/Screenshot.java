package Helper;

import org.apache.commons.io.FileUtils;
import org.openqa.selenium.OutputType;
import org.openqa.selenium.TakesScreenshot;
import org.openqa.selenium.WebDriver;

import java.io.File;

public class Screenshot {

    WebDriver driver;

    public Screenshot (WebDriver driver) { this.driver = driver; }

    public void takesScreenshot(String title)
    {
        try {
            File scrFile = ((TakesScreenshot)driver).getScreenshotAs(OutputType.FILE);
            FileUtils.copyFile(scrFile, new File("/Users/user/test/"+ title + ".png"));
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
